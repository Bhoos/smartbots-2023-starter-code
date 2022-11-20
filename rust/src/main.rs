mod bid;
mod cards;
mod choose_trump;
mod play;
use std::time::Instant;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use bid::BidPayload;

use choose_trump::ChooseTrumpPayload;

use play::PlayPayload;

#[get("/hi")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(r#"{"value":"hello"}"#)
}

#[post("/bid")]
async fn get_bid(payload: web::Json<BidPayload>) -> Result<String> {
    let current_bid = bid::get_bid(&payload);
    Ok(format!("{{\"bid\":{}}}", current_bid))
}

#[post("/chooseTrump")]
async fn select_trump(payload: web::Json<ChooseTrumpPayload>) -> Result<String> {
    let trump = choose_trump::select_trump(&payload);
    Ok(format!("{{\"suit\":\"{}\"}}", trump))
}
#[post("/play")]
async fn get_move(payload: web::Json<PlayPayload>) -> Result<String> {
    // let start = Instant::now();
    let player_move = play::get_move(&payload);
    // let duration = start.elapsed();
    // println!("Get move took {:?}", duration);
    Ok(player_move)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(hello)
            .service(get_bid)
            .service(select_trump)
            .service(get_move)
    })
    .bind(("0.0.0.0", 8001))?
    .run()
    .await
}
