use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use utoipa::openapi::Info;
use utoipa_swagger_ui::swagger_ui;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[post("/echo")]
async fn echo(data: web::Json<serde_json::Value>) -> impl Responder {
    HttpResponse::Ok().json(data.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let info = Info {
        title: "Rust API Example".to_string(),
        version: "1.0".to_string(),
        ..Default::default()
    };

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .wrap(swagger_ui(&info, None))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
