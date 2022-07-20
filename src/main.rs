use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use mongodb::Client;
use std::env;

#[path = "app/constants/index.rs"]
mod constants;
#[path = "routes/index.rs"]
mod routes;

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::scope("/routes"))
            .service(routes::index)
            .service(routes::create_user)
            .service(routes::update_user)
            .service(routes::get_all_users)
            .service(routes::delete_user)
            .service(routes::create_jwt_token)
            .service(routes::get_user),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let uri = match env::var("DB_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable DB_URL"),
    };
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(init)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
