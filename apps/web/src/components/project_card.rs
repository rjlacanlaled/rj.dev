use maud::{ Markup, html };

pub struct Project {
    pub name: String,
    pub description: String,
    pub url: String,
    pub img_url: String,
    pub technologies: Option<Vec<String>>,
}

impl Project {
    pub fn new(
        name: String,
        description: String,
        url: String,
        img_url: String,
        technologies: Option<Vec<String>>
    ) -> Self {
        Self {
            name,
            description,
            url,
            img_url,
            technologies,
        }
    }
}

pub fn render(project: Project) -> Markup {
    html! {
        div ."mb-16 scroll-mt-16 md:mb-24 lg:mb-36 lg:scroll-mt-24 flex flex-col gap-2" {
            div class ="flex flex-col" {
                h3 class="font-medium leading-snug text-slate-200" {
                    a class="inline-flex items-baseline text-base font-semibold leading-tight text-slate-200 hover:text-teal-300 focus-visible:text-teal-300 group/link" {
                        span class="absolute -inset-x-4 -inset-y-2.5 hidden rounded md:-inset-x-6 md:-inset-y-4 lg:block" {}
                        div class="flex items-center gap-1" {
                            span { (format!("{}", project.name)) }
                            img class="w-[16px]" src="/icons/arrow-up.svg" {}
                        }
                    }
                }
            }

            span class="text-sm" { (project.description) }


            @if let Some(technologies) = project.technologies {
                ul class="flex flex-wrap gap-1 mt-2"{
                    @for tech in technologies {
                        li class="flex items-center px-3 py-1 text-xs font-medium leading-5 text-teal-300 rounded-full bg-teal-400/10 "{ (tech) }
                    }
                }
            }

            img class="w-[200px] h-[120px] rounded border-2 border-slate-200/10 transition group-hover:border-slate-200/30 sm:order-1 sm:col-span-2 sm:translate-y-1 bg-transparent" src=(project.img_url) {}
        }


    }
}
