mod server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn get_users() -> impl Responder {
    HttpResponse::Ok().json("[{id:1, name:\"Qng\"}, {id:2, name:\"Ada\"}]")
}
async fn get_user_details() -> impl Responder {
    HttpResponse::Ok().json("{id:1, name:\"Qng\"}")
}

async fn create_user() -> impl Responder {
    HttpResponse::Ok().json("{ msg:\"User Created Successfully!\"}")
}

async fn update_user() -> impl Responder {
    HttpResponse::Ok().json("{ msg:\"User Updated Successfully!\"}")
}

async fn delete_user() -> impl Responder {
    HttpResponse::Ok().json("{ msg:\"User Deleted Successfully!\"}")
}

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
