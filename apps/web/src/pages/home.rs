use actix_web::{ get, HttpRequest, Result as ActixResult };
use maud::{ html, Markup };

use crate::{
    components::{ about_section, header, experience_section, project_section, footer },
    pages::base::index,
};

#[get("/")]
pub async fn page(req: HttpRequest) -> ActixResult<Markup> {
    let title: &str = "Home";
    let desc = "Rj Dev Home Page";

    let content =
        html! {
            div ."pointer-events-none inset-0 z-30 transition duration-300 lg:absolute" {
                div class="w-full flex flex-col gap-[7rem]" {
                    ( header::render() )
                    ( about_section::render() )
                    ( experience_section::render() )
                    ( project_section::render() ) 
                    ( footer::render() )
                }
            }
    };

    Ok(index(req, content, title, desc).await)
}
