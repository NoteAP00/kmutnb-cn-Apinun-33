use actix_web::{get,web,Responder,HttpResponse};

#[get("/shipping")]
async fn get_shipping_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello Shipping!")
}
