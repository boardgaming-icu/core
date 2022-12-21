mod database;
mod routes;
pub mod helpers;
pub mod structs;

use dotenv::{dotenv};

#[macro_use] extern crate rocket;

#[get("/ready")]
fn ready() -> String {
    "ready".to_string()
}

#[launch]
fn rocket() -> _ {
    dotenv().unwrap();
    rocket::build()
    .mount("/", routes![ready])
    .mount("/", routes![routes::signup])
    .mount("/", routes![routes::signin])
    .mount("/", routes![routes::req_verif_code])
    .mount("/", routes![routes::res_verif_code])
}
