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
        CREATE TABLE IF NOT EXISTS descs (name TEXT, desc TEXT);
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

    pub fn get_wheel_desc(&self) -> String {
        let mut statement = self
            .conn
            .prepare("SELECT desc FROM descs WHERE name = ?")
            .unwrap();
        statement.bind((1, "wheel")).unwrap();

        if let Ok(State::Row) = statement.next() {
            return statement
                .read::<String, _>("desc")
                .unwrap_or("Error reading desc".into());
        }

        "Spin the Wheel".into()
    }
}
