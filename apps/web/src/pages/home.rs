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
            div ."pointer-events-none inset-0 z-30 transition duration-300 lg:absolute lg:grid lg:grid-cols-2 w-full py-12" {
                div class="w-full lg:sticky lg:top-0 lg:flex lg:max-h-screen lg:flex-col lg:justify-between lg:px-24 lg:py-0 mb-[5rem]" {
                    ( header::render() )
                }

                div class="flex flex-col w-full gap-14 lg:sticky lg:top-0 lg:flex lg:max-h-screen lg:justify-between lg:px-24 lg:py-0" {
                    ( about_section::render() )
                    ( experience_section::render() )
                    ( project_section::render() ) 
                    ( footer::render() )
                }
            }
    };

    Ok(index(req, content, title, desc).await)
}
