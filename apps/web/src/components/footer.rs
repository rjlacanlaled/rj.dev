use maud::{ Markup, html };

pub fn render() -> Markup {
    return html! {
        footer ."max-w-md text-sm text-slate-500 sm:pb-0 md:px-12 md:py-20 lg:px-24 lg:py-0 px-6 py-2" {
            p { "Inspired by https://brittanychiang.com/. This project is coded in Rust using Actix Web for web framework and Maud for templating. 
                Dockerized with github workflows for CI/CD and hosted on Digital Ocean" 
            }
            input type="button" class="absolute bottom-0 right-0 inline-flex items-center px-2 py-4 font-medium hover:-text-teal-300 text-slate-400 hover:-translate-y-2 focus-visible:text-teal-300" {
                img src="/images/adventure-time-portal.gif" class="w-[50px] ml-auto" {}
            }
            
        }
    };
}
