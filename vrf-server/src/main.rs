mod database;

use database::{Database, Row};

fn main() {
    let database = Database::new("./output.db");

    database.insert(Row {
        seed: String::from("wow"),
        input: String::from("wow"),
        output: String::from("wow"),
        proof: String::from("wow"),
    });

    let size = database.size();

    let row = database.get(size);
    println!("Result: {:?}", row);

    for _ in 0..4 {
        database.insert(Row {
            seed: String::from("wow"),
            input: String::from("wow"),
            output: String::from("wow"),
            proof: String::from("wow"),
        });
    }

    let size = database.size();
    println!("Size: {}", size);
}
