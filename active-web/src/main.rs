extern crate actix_web;
extern crate handlebars;
extern crate serde_json;

use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde_json::json;

use handlebars::Handlebars;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn index_static() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index-static.html")?)
}

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Catdex",
        "cats" : [
            {
                "name": "British short hair",
                "image_path": "/static/image/british-short-hair.jpg"
            },
            {
                "name": "Persian",
                "image_path": "/static/image/persian.jpg"
            },
            {
                "name": "Ragdoll",
                "image_path": "/static/image/ragdoll.jpg"
            }
        ]
    });

    let body = hb.render("index-tmp", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Template Hadlebars
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./static/").unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("Listening on port 8080");

    HttpServer::new(move || {
        App::new()
            // Register handlebar templater
            .app_data(handlebars_ref.clone())
            // Static server in /static
            .service(
                Files::new("/static", "static")
                    .show_files_listing()
            )
            .route("/", web::get().to(index))
            .route("/index-static", web::get().to(index_static))
            .route("/hello", web::get().to(hello)
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}