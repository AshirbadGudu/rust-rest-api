use actix_web::{web, App, HttpServer};
mod server;
mod users;
use users::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("http://localhost:{}", server::PORT);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(server::health_check))
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user_details))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::patch().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind(("127.0.0.1", server::PORT))?
    .run()
    .await
}
