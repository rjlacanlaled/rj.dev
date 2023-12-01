use maud::{ html, Markup };
use super::experience_card;

pub fn render() -> Markup {
    let experiences = vec![
        experience_card::Experience::new(
            String::from("Lorem"),
            String::from("Lorem Ipsum"),
            Some(vec![String::from("Lorem Ipsum Pro Max"), String::from("Lorem Ipsum")]),
            String::from("2019"),
            String::from("Present"),
            String::from(
                "Lorem Ipsum Dolor Sit Amet Consectetur Adipiscing Elit Sed Do Eiusmod Tempor Incididunt Ut Labore Et Dolore 
                Magna Aliqua Ut Enim Ad Minim Veniam Quis Nostrud Exercitation Ullamco Laboris Nisi Ut Aliquip Ex Ea Commodo Consequat 
                Duis Aute Irure Dolor In Reprehenderit In Voluptate Velit Esse Cillum Dolore Eu Fugiat Nulla Pariatur Excepteur Sint Occaecat 
                Cupidatat Non Proident Sunt In Culpa Qui Officia Deserunt Mollit Anim Id Est Laborum"
            ),
            Some(vec![String::from("Rust"), String::from("React"), String::from("Solidity")]),
            Some(
                vec![
                    experience_card::Project::new(
                        String::from("Project 1"),
                        String::from("https://rjdev.io")
                    ),
                    experience_card::Project::new(
                        String::from("Project 2"),
                        String::from("https://rjdev.io")
                    ),
                    experience_card::Project::new(
                        String::from("Project 3"),
                        String::from("https://rjdev.io")
                    )
                ]
            )
        ),
        experience_card::Experience::new(
            String::from("Lorem"),
            String::from("Lorem Ipsum"),
            Some(vec![String::from("Lorem Ipsum Pro Max"), String::from("Lorem Ipsum")]),
            String::from("2019"),
            String::from("Present"),
            String::from(
                "Lorem Ipsum Dolor Sit Amet Consectetur Adipiscing Elit Sed Do Eiusmod Tempor Incididunt Ut Labore Et Dolore 
                Magna Aliqua Ut Enim Ad Minim Veniam Quis Nostrud Exercitation Ullamco Laboris Nisi Ut Aliquip Ex Ea Commodo Consequat"
            ),
            Some(vec![String::from("Rust"), String::from("React"), String::from("Solidity")]),
            Some(
                vec![
                    experience_card::Project::new(
                        String::from("Project 1 Long Long"),
                        String::from("https://rjdev.io")
                    ),
                    experience_card::Project::new(
                        String::from("Project 2"),
                        String::from("https://rjdev.io")
                    ),
                    experience_card::Project::new(
                        String::from("Project 3"),
                        String::from("https://rjdev.io")
                    )
                ]
            )
        )
    ];

    return html! {
        section ."scroll-mt-16 md:mb-24 lg:mb-36 lg:scroll-mt-24 flex flex-col gap-5" {
            div class="sticky top-0 z-20 w-screen px-6 py-5 mb-4 -mx-6 bg-slate-900/75 backdrop-blur md:-mx-12 md:px-12 lg:sr-only lg:relative lg:top-auto lg:mx-auto lg:w-full lg:px-0 lg:py-0 lg:opacity-0" {
                h2 class="px-6 text-sm font-bold tracking-widest uppercase text-slate-200 lg:sr-only" { "Experience" }
            }
            
            div class="flex flex-col w-full gap-2 px-6 text-lg font-normal md:px-12 md:py-20 lg:px-24 lg:py-0" {
                @for exp in experiences {
                    ( experience_card::render(exp) )
                }

                div class="flex items-center gap-2"{
                    a class="inline-flex items-center font-semibold leading-tight text-slate-200 group"{
                        span { "View Full Résumé" }
                        img class="w-[16px]" src="/icons/arrow-right.svg" {}
                    }
                }
            }
        }
    };
}
