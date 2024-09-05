use actix_web::{ HttpRequest, HttpResponse, Responder};
use crate::utils;

pub async fn controller(req:HttpRequest) -> impl Responder {
    let path = req.full_url();
    // let headers = req.headers();
    // let cookies = req.cookies();
    // let query = req.query_string();
    // let method = req.method();
    // let method = req.connection_info();
    
    utils::logger(path);
    HttpResponse::Ok().body("Hey there!")
}