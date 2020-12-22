use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello).service(test)
    }).bind("127.0.0.1:8080")?.run().await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Test")
}

#[derive(Debug)]
pub struct Developer {
    pub language: Vec<String>,
    pub ctype: Type,
}

impl Developer {
    fn new(language: Vec<String>, ctype: Type) -> Developer {
        Developer { language, ctype }
    }
}

#[derive(Debug)]
pub enum Type {
    Backend,
    FrontEnd,
}