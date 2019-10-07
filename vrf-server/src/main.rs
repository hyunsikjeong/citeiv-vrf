#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod database;

use std::sync::Mutex;

use database::{Database, Row};
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use vrf::openssl::CipherSuite;

#[derive(Serialize, Deserialize)]
struct ReqMessage {
    input: String,
}

#[derive(Serialize, Deserialize)]
struct ReqResponse {
    index: i64,
}

#[post("/req", format = "json", data = "<message>")]
fn req(message: Json<ReqMessage>, db: State<'_, Mutex<Database>>) -> Json<ReqResponse> {
    let mut database = db.lock().expect("database lock.");
    database.insert(message.0.input);
    Json(ReqResponse {
        index: database.size(),
    })
}

#[get("/get/<idx>")]
fn get(idx: i64, db: State<'_, Mutex<Database>>) -> Json<Row> {
    let database = db.lock().expect("database lock.");
    Json(database.get_row(idx))
}

#[get("/size")]
fn size(db: State<'_, Mutex<Database>>) -> Json<ReqResponse> {
    let database = db.lock().expect("database lock.");
    Json(ReqResponse {
        index: database.size(),
    })
}

#[get("/pubkey")]
fn pubkey(db: State<'_, Mutex<Database>>) -> JsonValue {
    let mut database = db.lock().expect("database lock.");
    json!({ "pubkey": hex::encode(database.pubkey()) })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn main() {
    // TODO: secret key storage
    let secret_key =
        hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721").unwrap();
    let database = Mutex::new(Database::new(
        "./output.db",
        CipherSuite::P256_SHA256_SWU,
        secret_key,
    ));

    rocket::ignite()
        .mount("/", routes![req, get, size, pubkey])
        .register(catchers![not_found])
        .manage(database)
        .launch();
}
