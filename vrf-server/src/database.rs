use std::path::Path;

pub struct Database {
    conn: sqlite::Connection,
}

#[derive(Debug)]
pub struct Row {
    pub seed: String,
    pub input: String,
    pub output: String,
    pub proof: String,
}

impl Database {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        // TODO: error handling
        let conn = sqlite::open(path).unwrap();
        let result = conn.execute(
            "
            CREATE TABLE outputs (seed TEXT, user_input TEXT, output TEXT, proof TEXT);
            ",
        );
        match result {
            Ok(_) => println!("Created the table"),
            Err(_) => println!("Already the table exists"),
        }
        Self { conn }
    }

    pub fn insert(&self, row: Row) {
        // TODO: remove '' from input
        // TODO: check hexstring-ness and length of seed/output/proof
        // TODO: error handling

        let mut statement = self
            .conn
            .prepare("INSERT INTO outputs VALUES(?, ?, ?, ?)")
            .unwrap();

        statement.bind(1, &*row.seed).unwrap();
        statement.bind(2, &*row.input).unwrap();
        statement.bind(3, &*row.output).unwrap();
        statement.bind(4, &*row.proof).unwrap();

        statement.next().unwrap();
    }

    pub fn get(&self, num: i64) -> Row {
        // Statement setting
        let mut statement = self
            .conn
            .prepare("SELECT * FROM outputs WHERE rowid = ?")
            .unwrap();
        statement.bind(1, num).unwrap();

        // Read
        statement.next().unwrap();
        Row {
            seed: statement.read::<String>(0).unwrap(),
            input: statement.read::<String>(1).unwrap(),
            output: statement.read::<String>(2).unwrap(),
            proof: statement.read::<String>(3).unwrap(),
        }
        // TODO: There must be one row
    }

    pub fn size(&self) -> i64 {
        let mut statement = self.conn.prepare("SELECT COUNT(*) FROM outputs").unwrap();

        statement.next().unwrap();
        statement.read::<i64>(0).unwrap()
    }
}