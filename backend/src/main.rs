use actix_web::{get, patch, post, web::Json, web::Path, App, HttpResponse, HttpServer, Responder};
use surrealdb::sql::Data;
use std::io::Result;
use validator::Validate;

mod models;
mod db;

use crate::models::pizza::{BuyPizzaRequest, UpdatePizzaUrl};
use crate::db::Database;

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder{
    HttpResponse::Ok().body("all pizzas are here!")    
}

#[post("/buypizza")]
async fn buy_pizza(body:Json<BuyPizzaRequest>) -> impl Responder{
    let is_valid = body.validate();
    match is_valid{
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Buying a {pizza_name}"))
        }
        Err(_) => {
            HttpResponse::Ok().body("pizza name required")
        }
    }

}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaUrl>) -> impl Responder{
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Update the pizza with {uuid}"))
}

#[actix_web::main]
async fn main() -> Result<()> {
    
    let db = Database::init()
        .await
        .expect("error connecting database");
    
    let db_data = Data::new(db);

    HttpServer::new(|| 
        App::new()
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)      
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await 
}
