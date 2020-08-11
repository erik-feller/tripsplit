#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use chrono::serde::ts_nanoseconds;
use chrono::{DateTime, Utc};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct TS {
    #[serde(with = "ts_nanoseconds")]
    time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Trip {
    id: i32,
    start_date: TS,
    active: bool,
}

#[derive(Deserialize, Serialize)]
struct Transaction {
    id: i32,
    trx_date: TS,
    amount: f32,
    user_id: i32,
    trip_id: i32,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test() -> &'static str {
    "This is a test"
}

#[post("/trip", data = "<input>")]
fn create_trip(input: Json<Trip>) -> &'static str {
    println!("{}", input.id);
    println!("{}", input.start_date.time);
    "This will create a trip"
}

#[post("/trx", data = "<input>")]
fn create_trx(input: Json<Transaction>) -> &'static str {
    println!("{}", input.amount);
    "This will add a transaction to a trip"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, test, create_trip, create_trx])
        .launch();
}
