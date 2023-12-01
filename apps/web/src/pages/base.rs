use crate::strings;
use actix_web::{ HttpRequest, Responder };
use maud::{ html, Markup, PreEscaped, DOCTYPE };

fn body(content: Markup) -> Markup {
    html! {
        body ."max-w-screen-xl mx-auto overflow-auto bg-slate-900 font-sans"
        {
            (content)
            script src="/js/htmx.min.js" {}
            script src="https://unpkg.com/hyperscript.org@0.9.12" {}
            // TODO: Google Analytics: change UA-XXXXX-Y to be your site's ID.
            (google_analytics("UA-XXXXX-Y"))
            // Non-H5BP editorial comment: please consider using another analytics solution
            // instead of gifting your users' data to Alphabet Inc. - see e.g.
            // <https://mentalpivot.com/ethical-web-analytics-alternatives-google/>
            // for a discussion of alternatives.
        }
    }
}

fn google_analytics(site_id: &str) -> Markup {
    html! {
        script {"
            window.ga = function () {{ ga.q.push(arguments) }}; ga.q = []; ga.l = +new Date;
            ga('create', '" (site_id) "', 'auto'); ga('set', 'anonymizeIp', true); ga('set', 'transport', 'beacon'); ga('send', 'pageview')" }
        script src="https://www.google-analytics.com/analytics.js" async {}
    }
}

fn head(title: &str, desc: &str, url: &str) -> Markup {
    html! {
        head {
            meta charset=(strings::UTF8);
            title { (title) }
            meta name=(strings::DESCRIPTION) content=(desc);
            meta name=(strings::VIEWPORT) content=(strings::VIEWPORT_CONTENT);
            meta property="og:title" content=(title);
            meta property="og:type" content=(strings::WEBSITE);
            meta property="og:url" content=(url);
            meta property="og:image" content="";
            meta name="theme-color" content="#fafafa";
            link rel="preconnect" href="https://fonts.googleapis.com";
            link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
            link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:wght@100;200;300;400;500;600;700;800;900&display=swap";
            link rel="stylesheet" href="/css/output.css";
        }
    }
}

pub fn page(host: &str, title: &str, desc: &str, lang: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html class="no-js" lang=(lang) {
            (head(title, desc, host))
            (body(content))
        }
    }
}

pub async fn index(req: HttpRequest, child_content: Markup, title: &str, desc: &str) -> Markup {
    let host = format!("{}", req.uri());
    let lang = "en";
    let content =
        html! {
            #content class = "antialiased leading-relaxed bg-slate-900 text-slate-400 selection:bg-teal-300 selection:text-teal-900" {
            (child_content)
        }
    };
    page(&host, title, desc, lang, content)
}

pub async fn not_found() -> impl Responder {
    (
        html! {
            html lang="en" {
                head {
                    meta charset=(strings::UTF8);
                    title { (strings::NOT_FOUND_TITLE) }
                    meta name=(strings::VIEWPORT) content=(strings::VIEWPORT_CONTENT);
                    style { (strings::NOT_FOUND_STYLE) }
                }
                body {
                    h1 { (strings::NOT_FOUND_TITLE) }
                    p { (strings::NOT_FOUND_MESSAGE) }
                }
                (PreEscaped(strings::NOT_FOUND_COMMENT))
            }
        },
        actix_web::http::StatusCode::NOT_FOUND,
    )
}
