mod database;
mod routes;
#[macro_use] extern crate rocket;

#[get("/ready")]
fn ready() -> String {
    "ready".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![ready])
    .mount("/", routes![routes::signup])
    .mount("/", routes![routes::signin])
    .mount("/", routes![routes::req_verif_code])
}
