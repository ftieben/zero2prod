use actix_web::{ web, App, HttpResponse, HttpServer };
use actix_web::dev::Server;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/", web::get().to(health_check)))
        .bind("127.0.0.1:8080")?
        .run();
    Ok(server)
}
