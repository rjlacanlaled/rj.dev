use maud::{html, Markup};

pub struct Social {
    pub name: String,
    pub url: String,
    pub icon: String,
}

impl Social {
    pub fn new(name: String, url: String, icon: String) -> Self {
        Self { name, url, icon }
    }
}

pub fn render(socials: Vec<Social>) -> Markup {
    html! {
        ul class="ml-1 mt-8 flex items-center" {
            @for social in socials {
                li class="mr-5 text-xs" {
                    a class="block hover:text-slate-200" href=(social.url) target="_blank" rel="noopener noreferrer" {
                        span class="sr-only" {}
                        img src=(social.icon) class="w-[23px]"{}
                    }
                }
            }
         }
    }
}
