use actix_web::{web, App, HttpResponse, HttpServer, Responder, post};
use image::ImageOutputFormat;
use serde_json::json;
use base64;
use std::io::Cursor;

#[post("/process-frame")]
async fn process_frame(body: web::Bytes) -> impl Responder {
    let mut image = image::load_from_memory(&body).unwrap();
    image = image.grayscale();
    let mut buffer = Cursor::new(Vec::new());
    image.write_to(&mut buffer, ImageOutputFormat::Png).unwrap();
    let base64_image = base64::encode(&buffer.get_ref());
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json!({ "image": base64_image }).to_string())
    
    //.json(json!({ "image": base64_image }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(process_frame)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}