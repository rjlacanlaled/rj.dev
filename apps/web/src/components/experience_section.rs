use maud::{ html, Markup };
use super::experience_card;

pub fn render() -> Markup {
    let experiences = vec![
        experience_card::Experience::new(
            String::from("UBx"),
            String::from("Software Engineer"),
            Some(vec![String::from("Software Engineer"), String::from("Junior Software Engineer")]),
            String::from("2019"),
            String::from("Present"),
            String::from("I mostly build decentralized apps now."),
            Some(vec![String::from("Rust"), String::from("React"), String::from("Solidity")]),
            Some(
                vec![
                    experience_card::Project::new(
                        String::from("Rj Dev"),
                        String::from("https://rjdev.io")
                    ),
                    experience_card::Project::new(
                        String::from("Rj Dev"),
                        String::from("https://rjdev.io")
                    ),
                    experience_card::Project::new(
                        String::from("Rj Dev"),
                        String::from("https://rjdev.io")
                    )
                ]
            )
        ),
        experience_card::Experience::new(
            String::from("Rj Dev"),
            String::from("Software Engineer"),
            Some(vec![String::from("I mostly build decentralized apps now.")]),
            String::from("2019"),
            String::from("Present"),
            String::from("I mostly build decentralized apps now."),
            Some(vec![String::from("Rust"), String::from("React"), String::from("Solidity")]),
            Some(
                vec![
                    experience_card::Project::new(
                        String::from("Rj Dev"),
                        String::from("https://rjdev.io")
                    ),
                    experience_card::Project::new(
                        String::from("Rj Dev"),
                        String::from("https://rjdev.io")
                    ),
                    experience_card::Project::new(
                        String::from("Rj Dev"),
                        String::from("https://rjdev.io")
                    )
                ]
            )
        )
    ];

    return html! {
        section ."scroll-mt-16 md:mb-24 lg:mb-36 lg:scroll-mt-24 flex flex-col gap-10" {
            h2 class="text-sm font-bold tracking-widest uppercase text-slate-200 lg:sr-only" {"Experience"}
            div class="flex flex-col gap-3" {
                @for exp in experiences {
                    ( experience_card::render(exp) )
                }
            }
        }
    };
}
