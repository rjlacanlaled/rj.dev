use maud::{ html, Markup };

pub struct Project {
    pub name: String,
    pub url: String,
}
pub struct Experience {
    pub company: String,
    pub position: String,
    pub position_progression: Option<Vec<String>>,
    pub from: String,
    pub to: String,
    pub description: String,
    pub technologies: Option<Vec<String>>,
    pub projects: Option<Vec<Project>>,
}

impl Experience {
    pub fn new(
        company: String,
        position: String,
        position_progression: Option<Vec<String>>,
        from: String,
        to: String,
        description: String,
        technologies: Option<Vec<String>>,
        projects: Option<Vec<Project>>
    ) -> Self {
        Self {
            company,
            position,
            position_progression,
            from,
            to,
            description,
            technologies,
            projects,
        }
    }
}

impl Project {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
        }
    }
}

pub fn render(exp: Experience) -> Markup {
    return html! {
        div ."mb-16 scroll-mt-16 md:mb-24 lg:mb-36 lg:scroll-mt-24 flex flex-col gap-2" {
            span class="z-10 mt-1 mb-2 text-xs font-semibold tracking-wide uppercase text-slate-500 sm:col-span-2" { (format!("{} — {}", exp.from, exp.to)) }

            div class ="flex flex-col" {
                h3 class="font-medium leading-snug text-slate-200" {
                    a class="inline-flex items-baseline text-base font-semibold leading-tight text-slate-200 hover:text-teal-300 focus-visible:text-teal-300 group/link" {
                        span class="absolute -inset-x-4 -inset-y-2.5 hidden rounded md:-inset-x-6 md:-inset-y-4 lg:block" {}
                        div class="flex items-center gap-1" {
                            span { (format!("{} · {}", exp.position, exp.company)) }
                            img class="w-[16px]" src="/icons/arrow-up.svg" {}
                        }
                    }
                }
                
                @if let Some(position_progression) = exp.position_progression {
                    @for position in position_progression {
                        span class="font-normal text-slate-500" { (position) }
                    }
                }
            }

            span class="text-sm" { (exp.description) }

            @if let Some(projects) = exp.projects {
                ul class="flex flex-wrap mt-2" {
                    @for project in projects {
                        li class="mr-1.5 mt-2" { 
                            span class="flex gap-1 text-sm font-medium text-slate-200" { 
                                img class="w-[12px]" src="/icons/clip.svg" {}
                                a href=(project.url) { (project.name)} 
                             }
                        }
                    }
                }
            }

            @if let Some(technologies) = exp.technologies {
                ul class="flex flex-wrap gap-1 mt-2"{
                    @for tech in technologies {
                        li class="flex items-center px-3 py-1 text-xs font-medium leading-5 text-teal-300 rounded-full bg-teal-400/10 "{ (tech) }
                    }
                }
            }
        }
    };
}
