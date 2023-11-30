use maud::{html, Markup};

pub fn render() -> Markup {
    return html! {
        section class="flex flex-col gap-10"{
            h2 class="text-sm font-bold uppercase tracking-widest text-slate-200 lg:sr-only" {"About"}
            p class="mb-4" {"Back in 2012, I decided to try my hand at creating custom Tumblr
                themes and tumbled head first into the rabbit hole of coding and 
                web development. Fast-forward to today, and I’ve had the privilege 
                of building software for an"}
        }
    };
}
