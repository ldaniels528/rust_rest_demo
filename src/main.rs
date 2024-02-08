// Rust REST Demo

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use chrono::Utc;
use rand::{Rng, RngCore, thread_rng};
use rand::distributions::Uniform;
use rand::prelude::ThreadRng;
use serde::Serialize;

#[derive(Serialize)]
struct StockQuote {
    symbol: String,
    exchange: String,
    last_sale: f64,
    transaction_time: i64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/{symbol}", web::get().to(get_quote_list))
            .route("/{symbol}/{exchange}", web::get().to(get_quote))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// Define the default handler function
// returns a JSON object
// Ex: http://localhost:8080
async fn index() -> impl Responder {
    HttpResponse::Ok().json(generate_quote())
}

// Define a handler function that accepts symbol and exchange parameters
// returns a JSON object
// Ex: http://localhost:8080/quote/AAPL/NYSE
async fn get_quote(path: web::Path<(String, String, )>) -> impl Responder {
    let symbol: String = (&path.0).to_string();
    let exchange: String = (&path.1).to_string();
    HttpResponse::Ok().json(generate_quote_from_symbol_and_exchange(symbol, exchange))
}

// Define a handler function that accepts a symbol parameter
// returns a JSON object
// Ex: http://localhost:8080/AAPL
async fn get_quote_list(path: web::Path<(String, )>) -> impl Responder {
    let symbol: String = (&path.0).to_string();
    HttpResponse::Ok().json(generate_quote_array(symbol))
}

fn generate_quote() -> StockQuote {
    generate_quote_from_symbol_and_exchange(generate_random_symbol(), generate_random_exchange())
}

fn generate_quote_array(symbol: String) -> Box<[StockQuote]> {
    let mut rng: ThreadRng = thread_rng();
    let count: usize = rng.sample(Uniform::new(2, 5));
    (0..count).map(|index|
        StockQuote {
            symbol: symbol.to_string(),
            exchange: get_exchange_from_index(index),
            last_sale: 400.0 * rng.sample(Uniform::new(0.0, 1.0)),
            transaction_time: Utc::now().timestamp_millis() + rng.sample(Uniform::new(0, 1000)),
        }).collect()
}

fn generate_quote_from_symbol(symbol: String) -> StockQuote {
    StockQuote {
        symbol,
        exchange: generate_random_exchange(),
        last_sale: generate_random_last_sale(),
        transaction_time: generate_random_transaction_time(),
    }
}

fn generate_quote_from_symbol_and_exchange(symbol: String, exchange: String) -> StockQuote {
    StockQuote {
        symbol,
        exchange,
        last_sale: generate_random_last_sale(),
        transaction_time: generate_random_transaction_time(),
    }
}

fn get_exchange_from_index(exchange_index: usize) -> String {
    let exchanges = ["AMEX", "NASDAQ", "NYSE", "OTCBB", "OTHEROTC"];
    exchanges[exchange_index % exchanges.len()].parse().unwrap()
}

fn generate_random_exchange() -> String {
    let mut rng: ThreadRng = thread_rng();
    let exchange_index = rng.next_u32();
    get_exchange_from_index(exchange_index as usize)
}

fn generate_random_last_sale() -> f64 {
    let mut rng: ThreadRng = thread_rng();
    400.0 * rng.sample(Uniform::new(0.0, 1.0))
}

fn generate_random_symbol() -> String {
    let mut rng: ThreadRng = thread_rng();
    (0..5)
        .map(|_| rng.gen_range(b'A'..=b'Z') as char)
        .collect()
}

fn generate_random_transaction_time() -> i64 {
    let mut rng: ThreadRng = thread_rng();
    Utc::now().timestamp_millis() - (rng.sample(Uniform::new(0, 10000)) as i64)
}