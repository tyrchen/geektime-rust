#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Hello {
    name: String,
}

#[get("/", format = "json")]
fn hello() -> Json<Hello> {
    Json(Hello { name: "Tyr".into() })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
