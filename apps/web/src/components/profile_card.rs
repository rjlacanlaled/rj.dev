use maud::{ html, Markup };

pub fn render(name: String, title: String, description: String) -> Markup {
    html! {
        div ."gap-2 flex flex-col w-full" {
            h1 class="text-4xl font-bold tracking-tight text-slate-200 sm:text-5xl" { (name) }
            h2 class="text-lg font-medium tracking-tight text-slate-200 sm:text-xl" { (title) }
            p class="max-w-xs text-sm leading-normal" { (description) }
         }
    }
}
