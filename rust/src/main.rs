use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web, Result};
use actix_cors::Cors;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST"])
                .allow_any_header()
                .max_age(300)
            )
            .service(hello)
            .service(get_bid)
            .service(choose_trump)
            .service(make_move)
    })
    .bind(("0.0.0.0", 8001))?
    .run()
    .await
}

const LOG : bool = true;


mod bid_and_trump;
mod play;
mod payload_types;
mod action_types;
mod cards;

use action_types::*;
// use bid_and_trump::*;

#[get("/hi")]
async fn hello() -> impl Responder {
    let hello = Action::Hi.json_format();
    if LOG {
        // use cards::*;
        println!("\n/hi\n{}",hello);
    }
    HttpResponse::Ok().body(hello)
}

#[post("/bid")]
async fn get_bid(payload : web::Json<payload_types::BidPayload>) -> Result<String> {
    let response = bid_and_trump::get_bid(&payload).json_format();
    if LOG {
        println!("\n/bid");
        println!("payload = {}", payload);
        println!("response = {}", response);

    }
    Ok(response)
}

#[post("/chooseTrump")]
async fn choose_trump(payload: web::Json<payload_types::ChooseTrumpPayload>)  -> Result<String> {
    // let response = format!("{{\"suit\":\"{}\"}}", bid_and_trump::choose_trump(&payload));
    let response = bid_and_trump::choose_trump(&payload).json_format();
    if LOG {
        println!("\n/chooseTrump");
        println!("payload = {}", payload);
        println!("response = {}", response);
    }
    Ok(response)
}

#[post("/play")]
async fn make_move(payload : web::Json<payload_types::PlayPayload>) -> Result<String> {
    let response = play::make_move(&payload).json_format();
    // let response = Action::Play(PlayAction::CardThrow(
    //     cards::Card::new(&"JS".to_string())
    // )).json_format();

    if LOG {
        println!("\n/play");
        println!("payload = {}", payload);
        println!("response = {}", response);
    }
    Ok(response)
}