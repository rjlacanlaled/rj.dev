mod pages;
mod components;
mod strings;

use actix_files::Files as ActixFiles;
use actix_web::web::route;
use actix_web::{ App, HttpServer };

use pages::base::not_found;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(pages::home::page)
            .service(ActixFiles::new("/", "./src/static").prefer_utf8(true))
            .default_service(route().to(not_found))
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}