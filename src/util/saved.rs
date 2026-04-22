use sqlite::Connection;
use sqlite::State;

pub struct Store {
    conn: Connection,
}

pub fn init_db() -> Store {
    let conn = sqlite::open("payos.db").expect("DATABASE NOT LOADED!!!");

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS people (name TEXT);
    ",
    )
    .unwrap();

    Store { conn }
}

impl Store {
    pub fn get_people(&self) -> Vec<String> {
        let mut statement = self.conn.prepare("SELECT * FROM people").unwrap();

        let mut res = Vec::new();

        while let Ok(State::Row) = statement.next() {
            res.push(statement.read::<String, _>("name").unwrap());
        }

        res
    }

    pub fn add_person(&self, name: &str) {
        let mut statement = self.conn.prepare("INSERT INTO people VALUES (?)").unwrap();
        statement.bind((1, name)).unwrap();
        let _ = statement.next();
    }
}
