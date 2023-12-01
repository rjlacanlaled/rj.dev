use maud::{ html, Markup };

use crate::components::project_card;

use super::project_card::Project;

pub fn render() -> Markup {
    let projects = vec![
        Project::new(
            String::from("Rust Web App"),
            String::from("Lorem Ipsum Project Projectm something something"),
            String::from("https://rjdev.io"),
            String::from("/images/placeholder.png"),
            Some(vec![String::from("Rust"), String::from("React"), String::from("Solidity")])
        ),
        Project::new(
            String::from("Rust Web App"),
            String::from("Lorem Ipsum Project Projectm something something"),
            String::from("https://rjdev.io"),
            String::from("/images/placeholder.png"),
            Some(vec![String::from("Rust"), String::from("React"), String::from("Solidity")])
        ),
        Project::new(
            String::from("Rust Web App"),
            String::from("Lorem Ipsum Project Projectm something something"),
            String::from("https://rjdev.io"),
            String::from("/images/placeholder.png"),
            Some(vec![String::from("Rust"), String::from("React"), String::from("Solidity")])
        )
    ];

    return html! {
        section ."scroll-mt-16 md:mb-24 lg:mb-36 lg:scroll-mt-24 flex flex-col gap-10" {
            h2 class="text-sm font-bold tracking-widest uppercase text-slate-200 lg:sr-only" {"Projects"}
            div class="flex flex-col gap-3" {
                @for project in projects {
                    ( project_card::render(project) )
                }

                div class="flex items-center gap-2"{
                    a class="inline-flex items-center font-semibold leading-tight text-slate-200 group"{
                        span { "View Full Project Archive" }
                        img class="w-[16px]" src="/icons/arrow-right.svg" {}
                    }
                }
            }
        }
    };
}
