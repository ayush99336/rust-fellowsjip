use actix_web::{web, App, HttpResponse, HttpServer, Result};

async fn hello() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Hello from Rust HTTP Server!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running on http://127.0.0.1:3000");

    HttpServer::new(|| {
        App::new().route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
