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
        div ."group relative grid pb-1 transition-all sm:grid-cols-8 sm:gap-8 md:gap-4 lg:hover:!opacity-100 lg:group-hover/list:opacity-50" {
            span class="z-10 mt-1 mb-2 text-xs font-semibold tracking-wide uppercase text-slate-500 sm:col-span-2" { (format!("{} — {}", exp.from, exp.to)) }
            h3 class="font-medium leading-snug text-slate-200" {
                a class="inline-flex items-baseline text-base font-semibold leading-tight text-slate-200 hover:text-teal-300 focus-visible:text-teal-300 group/link" {
                    span class="absolute -inset-x-4 -inset-y-2.5 hidden rounded md:-inset-x-6 md:-inset-y-4 lg:block" {}
                    span { (format!("{} ·",exp.position))  }
                    span class="inline-block" { (exp.company) }
                }
            }
            @if let Some(position_progression) = exp.position_progression {
                @for position in position_progression {
                    span { (position) }
                }
            }
            span { (exp.description) }

            @if let Some(projects) = exp.projects {
                ul {
                    @for project in projects {
                        li { a href=(project.url) { (project.name)} }
                    }
                }
            }

            @if let Some(technologies) = exp.technologies {
                ul {
                    @for tech in technologies {
                        li { (tech) }
                    }
                }
            }
        }
    };
}
