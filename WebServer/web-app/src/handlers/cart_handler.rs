
use crate::models::product::Product;
use actix_web::{web,get,Responder,HttpResponse};
use serde_json::json;

#[get("/cart")]
async fn get_cart() -> impl Responder {
    HttpResponse::Ok().json("Hello Cart!")
}

#[get("/cart/{cart_item}")]
async fn get_cart_item_by_id(cart_item: web::Path<i32>) -> impl Responder {
    let id : i32 = cart_item.to_string().parse().unwrap();
    let product_item = vec![
        Product {
            id: id,
            name: "Item 1".to_string()
        },
        Product {
            id: 2,
            name: "Item 2".to_string()
        },
        Product {
            id: 3,
            name: "Item 3".to_string()
        },
    ];

        let response_body = json!(product_item);
    
    
    HttpResponse::Ok().json(response_body)
}
