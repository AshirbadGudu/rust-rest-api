use actix_web::{HttpResponse, Responder};

pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().json("[{id:1, name:\"Qng\"}, {id:2, name:\"Ada\"}]")
}
pub async fn get_user_details() -> impl Responder {
    HttpResponse::Ok().json("{id:1, name:\"Qng\"}")
}

pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().json("{ msg:\"User Created Successfully!\"}")
}

pub async fn update_user() -> impl Responder {
    HttpResponse::Ok().json("{ msg:\"User Updated Successfully!\"}")
}

pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().json("{ msg:\"User Deleted Successfully!\"}")
}
