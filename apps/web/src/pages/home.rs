use actix_web::{ HttpRequest, Result as ActixResult, get };
use maud::{ Markup, html };

use crate::pages::base::index;


#[get("/")]
pub async fn page(req: HttpRequest) -> ActixResult<Markup> {
    let title: &str = "Home";
    let desc = "Rj Dev Home Page";

    let content =
        html! {
        div {
            p { "Hello, World!" }
        }
    };

    Ok(index(req, content, title, desc).await)
}