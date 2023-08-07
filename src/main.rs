use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// Define handler functions for each endpoint
// Define handler functions for each endpoint

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn echo() -> impl Responder {
    HttpResponse::Ok().body("This is the echo endpoint KAKA")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// Set up the Actix Web server

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/hello").route(web::get().to(hello)))
            .service(web::resource("/echo").route(web::get().to(echo)))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
