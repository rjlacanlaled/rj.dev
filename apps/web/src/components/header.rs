use maud::{ html, Markup };

use crate::components::{ profile_card, social_links::{ self, Social } };

pub fn render() -> Markup {
    return html! {
        header ."lg:sticky lg:top-0 lg:flex lg:max-h-screen lg:w-1/2 lg:flex-col lg:justify-between lg:py-24 py-6 md:px-12 md:py-20 lg:px-24 lg:py-0 px-6" {
            div class="w-full " {
                ( profile_card::render(
                    String::from("Rj Lacanlale"),
                    String::from("Software Engineer"),
                    String::from("I mostly build decentralized apps now."))
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
