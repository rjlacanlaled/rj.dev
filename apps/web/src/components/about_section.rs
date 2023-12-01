use maud::{ html, Markup };

pub fn render() -> Markup {
    return html! {
        section ."scroll-mt-16 lg:mb-36 lg:scroll-mt-24 flex flex-col" {
            div class="sticky top-0 z-20 w-screen py-5 mb-4 -mx-6 bg-slate-900/75 backdrop-blur md:-mx-12 md:px-12 lg:sr-only lg:relative lg:top-auto lg:mx-auto lg:w-full lg:px-0 lg:py-0 lg:opacity-0" {
                h2 class="px-12 text-sm font-bold tracking-widest uppercase text-slate-200 lg:sr-only" { "About" }
            }

            div class="flex flex-col w-full gap-2 px-6 text-lg font-normal md:px-12 lg:px-24 lg:py-0" {
                p class="mb-4" {"I started my career as a software engineer in 2017 and decided to focus on blockchain development in 2021. I always have a passion
                    for learning difficult technologies and I am always looking for new challenges. I can say that I am a self-taught software developer and I always
                    put in time for learning even in busy work schedules."}
                p class="mb-4" {"I am currently working as a blockchain developer at UBx Philippines"}
                p class="mb-4" {"This website is actually a fun little experiment so I'm using it to learn more about Rust and its web frameworks. I'm also using to learn templating enginees like Maud"}
            }
        }
    };
}
