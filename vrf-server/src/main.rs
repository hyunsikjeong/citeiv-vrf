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

const API_VERSION: &str = "0.1";

#[derive(Serialize, Deserialize)]
struct ReqMessage {
    input: String,
}

#[derive(Serialize, Deserialize)]
struct ReqResponse {
    index: i64,
}

#[post("/req", format = "json", data = "<message>")]
fn req(message: Json<ReqMessage>, db: State<'_, Mutex<Database>>) -> Option<Json<ReqResponse>> {
    let mut database = db.lock().expect("database lock.");
    match database.insert(message.0.input) {
        Ok(_) => Some(Json(ReqResponse {
            index: database.size().unwrap(),
        })),
        Err(_) => None,
    }
}

#[get("/get/<idx>")]
fn get(idx: i64, db: State<'_, Mutex<Database>>) -> Option<Json<Row>> {
    let database = db.lock().expect("database lock.");
    if idx <= 0 || idx > database.size().unwrap() {
        None
    } else {
        // Must succeed
        Some(Json(database.get_row(idx).unwrap()))
    }
}

#[get("/size")]
fn size(db: State<'_, Mutex<Database>>) -> Json<ReqResponse> {
    let database = db.lock().expect("database lock.");
    Json(ReqResponse {
        index: database.size().unwrap(),
    })
}

#[get("/pubkey")]
fn pubkey(db: State<'_, Mutex<Database>>) -> JsonValue {
    let mut database = db.lock().expect("database lock.");
    json!({ "pubkey": hex::encode(database.pubkey().unwrap()) })
}

#[get("/version")]
fn version(_db: State<'_, Mutex<Database>>) -> JsonValue {
    json!({ "version": API_VERSION })
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
    let database = Database::new("./output.db", CipherSuite::P256_SHA256_SWU, secret_key);

    let db_mutex = match database {
        Ok(v) => Mutex::new(v),
        Err(e) => {
            eprintln!("Database creation error: {}", e);
            return;
        }
    };

    rocket::ignite()
        .mount("/", routes![req, get, size, pubkey, version])
        .register(catchers![not_found])
        .manage(db_mutex)
        .launch();
}
