package main

import (
	"database/sql"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"
	_ "modernc.org/sqlite"
)

func main() {

	if len(os.Args) == 1 {
		panic("db path not specified")
	}

	path := os.Args[1]

	db, err := sql.Open("sqlite", path)
	if err != nil {
		panic(err)
	}
	defer db.Close()

	db.Exec("CREATE TABLE IF NOT EXISTS people (name TEXT)")

	r := gin.Default()
	r.LoadHTMLGlob("templates/*")

	r.GET("/", func(c *gin.Context) {
		rows, err := db.Query("SELECT rowid, name FROM people")
		if err != nil {
			c.String(http.StatusInternalServerError, err.Error())
			return
		}
		defer rows.Close()

		type Person struct {
			ID   int
			Name string
		}
		var people []Person
		for rows.Next() {
			var p Person
			rows.Scan(&p.ID, &p.Name)
			people = append(people, p)
		}
		c.HTML(http.StatusOK, "index.html", gin.H{"people": people})
	})

	r.POST("/add", func(c *gin.Context) {
		name := c.PostForm("name")
		if name != "" {
			db.Exec("INSERT INTO people (name) VALUES (?)", name)
		}
		c.Redirect(http.StatusFound, "/")
	})

	r.POST("/delete/:id", func(c *gin.Context) {
		id := c.Param("id")
		db.Exec("DELETE FROM people WHERE rowid = ?", id)
		c.Redirect(http.StatusFound, "/")
	})

	r.GET("/edit/:id", func(c *gin.Context) {
		id := c.Param("id")
		var name string
		err := db.QueryRow("SELECT name FROM people WHERE rowid = ?", id).Scan(&name)
		if err != nil {
			c.Redirect(http.StatusFound, "/")
			return
		}
		c.HTML(http.StatusOK, "edit.html", gin.H{"id": id, "name": name})
	})

	r.POST("/edit/:id", func(c *gin.Context) {
		id := c.Param("id")
		name := c.PostForm("name")
		if name != "" {
			db.Exec("UPDATE people SET name = ? WHERE rowid = ?", name, id)
		}
		c.Redirect(http.StatusFound, "/")
	})

	r.GET("/exit", func(c *gin.Context) {
		os.Exit(0)
	})

	r.Run(":8080")
}
