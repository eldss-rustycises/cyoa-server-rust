//! This crate was meant to be used with an existing frontend that
//! I had build for a similar server written in Go. However, upon
//! finishing the logic, I learned there was not a great way to handle
//! CORS with the rocket crate. In any case, this was still a good
//! exercise in using rocket.

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use failure::Error;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::{from_reader, Map, Value};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, RwLock};

type StoryData = Arc<RwLock<Map<String, Value>>>;

/// Retrieves a story arc from the json loaded into memory
/// when the server started.
#[get("/arc/<arc_id>")]
fn get_story_arc(arc_id: String, data: State<RwLock<Value>>) -> Json<Value> {
    let val = data.read().unwrap().as_object().unwrap()[&arc_id].clone();
    Json(val)
}

fn main() -> Result<(), Error> {
    // Read data into a thread-safe object in memory
    let json_file = File::open("data/story.json")?;
    let reader = BufReader::new(json_file);
    let data: RwLock<Value> = RwLock::new(from_reader(reader)?);
    // Pass data as managed state
    rocket::ignite()
        .manage(data)
        .mount("/", routes![get_story_arc])
        .launch();

    Ok(())
}
