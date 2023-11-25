use actix_web::{get, HttpRequest, Result as ActixResult};
use maud::{html, Markup};

use crate::pages::base::index;

#[get("/")]
pub async fn page(req: HttpRequest) -> ActixResult<Markup> {
    let title: &str = "Home";
    let desc = "Rj Dev Home Page";

    let content = html! {
        div class="container bg-red-300 w-full min-h-screen" {
            div class="container  h-full w-full p-10 bg-blue-300" {
                p { "Hello World" }
            }
         }
    };

    Ok(index(req, content, title, desc).await)
}
