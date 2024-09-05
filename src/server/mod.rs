use actix_web::{HttpServer, App, web};
use crate::constants::{DEFAULT_HOST, DEFAULT_PORT};
mod controller;
pub  async fn server(port:Option<u16>) -> std::io::Result<()>{
    println!("Server started on port {}", port.unwrap_or(DEFAULT_PORT));
    HttpServer::new(||{
        App::new()
        .service(web::resource("/{tail:.*}").route(web::route().to(controller::controller)))
    })    
    .bind((DEFAULT_HOST, port.unwrap_or(DEFAULT_PORT)))?
    .run()
    .await
}