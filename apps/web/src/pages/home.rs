use actix_web::{get, HttpRequest, Result as ActixResult};
use maud::{html, Markup};

use crate::{
    components::{about_section, header},
    pages::base::index,
};

#[get("/")]
pub async fn page(req: HttpRequest) -> ActixResult<Markup> {
    let title: &str = "Home";
    let desc = "Rj Dev Home Page";

    let content = html! {
        div class="pointer-events-none fixed inset-0 z-30 transition duration-300 lg:absolute" {
            div class="container w-full flex flex-col gap-[7rem]" {
                ( header::render() )
                ( about_section::render() )
            }
        }
    };

    Ok(index(req, content, title, desc).await)
}
