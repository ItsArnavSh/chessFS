#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[get("/api/data")]
fn get_data() -> Json<&'static str> {
    Json("{\"message\": \"Hello from Rocket!\"}")
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:5173", // Add your origins here
    ]);

    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(vec![Method::Get, Method::Post].into_iter().map(From::from).collect())
        .allow_credentials(true)
        .to_cors()
        .unwrap();

    rocket::build().mount("/", routes![get_data]).attach(cors)
}