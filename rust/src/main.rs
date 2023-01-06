use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() //      to allow input from any origin i.e. sandbox to pc, or server to docker the instance
                    .allowed_methods(vec!["GET", "POST"]) //      to allow only two method used, "get" and "post" method of request
                    .allow_any_header() //      to allow any header information sent with the method, it doesn't matter
                    .max_age(300), //           cache time set 5 minutes for frequent update
            )
            .service(hello) //          the get request of http://0.0.0.0:8001/hi is handled by hello()
            .service(get_bid) //        the post request of http://0.0.0.0:8001/bid is handled by get_bid()
            .service(choose_trump) //   the post request of http://0.0.0.0:8001/chooseTrump is handled by choose_trump()
            .service(make_move) //      the post request of http://0.0.0.0:8001/play is handled by make_move()
    })
    .bind(("0.0.0.0", 8001))?
    .run()
    .await
}

const LOG: bool = true;

mod action_types;
mod bid_and_trump;
mod cards;
mod payload_types;
mod play;

use action_types::*;

#[get("/hi")]
async fn hello() -> impl Responder {
    let hello = Action::Hi.json_format();
    if LOG {
        println!("\n/hi\n{}", hello);
    }
    HttpResponse::Ok().body(hello)
}

#[post("/bid")]
async fn get_bid(payload: web::Json<payload_types::BidPayload>) -> Result<String> {
    let response = bid_and_trump::get_bid(&payload).json_format();
    if LOG {
        println!("\n/bid");
        println!("payload = {}", payload);
        println!("response = {}", response);
    }
    Ok(response)
}

#[post("/chooseTrump")]
async fn choose_trump(payload: web::Json<payload_types::ChooseTrumpPayload>) -> Result<String> {
    let response = bid_and_trump::choose_trump(&payload).json_format();
    if LOG {
        println!("\n/chooseTrump");
        println!("payload = {}", payload);
        println!("response = {}", response);
    }
    Ok(response)
}

#[post("/play")]
async fn make_move(payload: web::Json<payload_types::PlayPayload>) -> Result<String> {
    let response = play::make_move(&payload).json_format();
    if LOG {
        println!("\n/play");
        println!("payload = {}", payload);
        println!("response = {}", response);
    }
    Ok(response)
}
