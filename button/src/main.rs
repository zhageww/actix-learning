use std::collections::HashMap;

use actix_web::{web, App, HttpResponse, HttpServer, Result};
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Serialize, Deserialize)]
pub struct MyParams {
    button: String,
}

async fn index() -> Result<HttpResponse> {
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // start http server
    HttpServer::new(move || {
        App::new()
        .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/post1").route(web::post().to(post1)))
        .service(web::resource("/post2").route(web::post().to(post2)))
        .service(web::resource("/post3").route(web::post().to(post3)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}


async fn post1(params: web::Form<MyParams>) ->Result<HttpResponse> {
    println!("param : {}", params.button);
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn post2(params: web::Form<MyParams>) ->Result<HttpResponse> {
    println!("param : {}", params.button);
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn post3(params: web::Form<MyParams>) ->Result<HttpResponse> {
    println!("param : {}", params.button);
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}