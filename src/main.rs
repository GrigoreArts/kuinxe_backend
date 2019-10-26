use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello World Again!")
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:7878")
    .unwrap()
    .run()
    .unwrap();
}