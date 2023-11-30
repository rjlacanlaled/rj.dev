use maud::{html, Markup};

use crate::components::{
    profile_card,
    social_links::{self, Social},
};

pub fn render() -> Markup {
    return html! {
        header class="lg:sticky lg:top-0 lg:flex lg:max-h-screen lg:w-1/2 lg:flex-col lg:justify-between lg:py-24" {
            div class="w-full" {
                ( profile_card::render(
                    String::from("Brittany Chiang"),
                    String::from("Experienced Front-end Engineer"),
                    String::from("I build exceptional and accessible digital experiences for the web."))
                )
            }

            div class="w-full" {
                ( social_links::render(vec![
                    Social::new(
                        String::from("Github"),
                        String::from("github.com"),
                        String::from("/icons/github.svg")
                    ),
                    Social::new(
                        String::from("Twitter"),
                        String::from("twitter.com"),
                        String::from("/icons/twitter.svg")
                    ),
                    Social::new(
                        String::from("LinkedIn"),
                        String::from("linkedin.com"),
                        String::from("/icons/linkedin.svg")
                    ),
                    Social::new(
                        String::from("Instagram"),
                        String::from("email.com"),
                        String::from("/icons/instagram.svg")
                    ),
                ]))
            }
        }
    };
}
