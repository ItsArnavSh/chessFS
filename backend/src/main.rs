#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[get("/startmap")]
fn start_map() -> Json<String> {
    let map:String = String::from("{\"message\":\"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR\"}");
    Json(map)
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

    rocket::build().mount("/", routes![start_map]).attach(cors)
}