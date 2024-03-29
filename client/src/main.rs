#![recursion_limit = "1024"]

mod blogs;
mod components;
mod protocols;
mod projects;

use std::rc::Rc;

use gloo_net::http::Request;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::glyphs::{Glyph, GlyphType};

use crate::components::blog_list::BlogListPage;
use crate::components::blog_page::BlogPage;
use crate::components::home::HomePage;
use crate::components::projects::ProjectsPage;

use crate::protocols::blog_list_toml::BlogListMetadata;
use crate::protocols::projects_list_toml::ProjectsMetadata;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    // shortcuts
    #[at("/blogs/palantir_essay/essay.pdf")]
    EssayPDF,

    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/blogs")]
    BlogList,
    #[at("/resume")]
    Resume,
    #[at("/blogs/:slug")]
    Blog { slug: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Projects => html! { <ProjectsPage /> },
        Route::BlogList => html! { <BlogListPage /> },
        Route::Blog { slug } => html! { <BlogPage slug={slug}/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Resume => {
            web_sys::window().unwrap().location().set_pathname("/static/content/resume.pdf").unwrap();
            html! { <h1>{ "Redirecting to resume..." }</h1> }
        },
        Route::EssayPDF => {
            web_sys::window().unwrap().location().set_pathname("/static/content/blogs/palantir_essay/essay.pdf").unwrap();
            html! { <h1>{ "Redirecting to pdf..." }</h1> }
        },
    }
}

fn toggle_local_storage() -> Option<()> {
    let key = "theme";
    let click_default = "dark";
    let window = web_sys::window()?;
    let local_storage = window.local_storage().ok()??;
    let (_old_value, new_value) = match local_storage.get(key) {
        Ok(Some(val)) if val == "light" => ("light", "dark"),
        Ok(Some(val)) if val == "dark" => ("dark", "light"),
        _ => ("", click_default),
    };
    local_storage.set_item(key, new_value).unwrap();

    // set document element's 'class' to 'dark'
    window
        .document()?
        .document_element()?
        .set_class_name(new_value);

    Some(())
}

#[function_component(Header)]
fn header() -> Html {
    let is_expanded = use_state(|| false);
    // if it's expanded, show everything
    let hidden = if *is_expanded {"flex" } else {"hidden"};
    let rest_of_menu =  html! {
        <>
            <div class={classes!(hidden, "md:flex", "md:flex-row", "items-center", "justify-center")}>
                <Link<Route> to={Route::Home} classes={classes!("text-white", "dark:text-black", "rounded", "hover:bg-gray-500", "hover:text-gray-100", "hover:font-medium", "py-2", "px-2", "md:mx-2")}>{"Home"}</Link<Route>>
                <Link<Route> to={Route::BlogList} classes={classes!("text-white", "dark:text-black", "rounded", "hover:bg-gray-500", "hover:text-gray-100", "hover:font-medium", "py-2", "px-2", "md:mx-2")}>{"Blogs"}</Link<Route>>
                <Link<Route> to={Route::Projects} classes={classes!("text-white", "dark:text-black", "rounded", "hover:bg-gray-500", "hover:text-gray-100", "hover:font-medium", "py-2", "px-2", "md:mx-2")}>{"Projects"}</Link<Route>>
            </div>
            <div class={classes!(hidden, "md:flex", "md:flex-row", "items-center", "justify-center")}>
                <div class={classes!("font-bold", "text-gray-800", "md:text-3xl", "flex", "")}>
                    <a href="https://github.com/tfpk/"><Glyph glyph={GlyphType::Github} color="text-white dark:text-black" /></a>
                    <a href="https://linkedin.com/in/tfpk/"><Glyph glyph={GlyphType::Linkedin} color="text-white dark:text-black" /></a>
                    <a href="https://twitter.com/tfpk_/"><Glyph glyph={GlyphType::Twitter} color="text-white dark:text-black" /></a>
                    <button class={classes!("bg-transparent", "border-transparent")} onclick={|_| {toggle_local_storage();}}><Glyph glyph={GlyphType::Brightness} color="text-white dark:text-black" /></button>
                </div>
            </div>
        </>
    };
    html! {
        <header>
            <div class={classes!("w-full")}>
                <nav class={classes!("bg-accent", "shadow-lg")}>
                    <div class={classes!("flex", "flex-col", "md:flex-row", "justify-between", "py-2", "px-8", "md:px-12")}>
                        <div class={classes!("flex", "justify-between", "items-center", "w-full", "md:w-auto")}>
                            <div class={classes!("font-bold", "text-white", "md:text-3xl", "flex")}>
                               <img class={classes!("relative", "mr-4", "inline-block", "h-9", "w-9", "rounded-md", "object-cover", "object-center")} alt="Icon" src="/static/icon_tfpk.jpg"/>
                                <Link<Route> to={Route::Home} classes={classes!("text-2xl", "text-white", "dark:text-black")}>
                                  {"TFPK"}
                                </Link<Route>>
                            </div>
                            <div class={classes!("md:hidden")}>
                                <button
                                    type="button"
                                    class={classes!("block", "text-white", "hover:text-gray-500", "focus:text-gray-500", "focus:outline-none")}
                                    onclick={move |_| {is_expanded.set(!*(is_expanded))}}
                                >
                                    <Glyph glyph={GlyphType::Snackbar} />
                                </button>
                            </div>
                        </div>
                        {rest_of_menu}
                    </div>
                </nav>
            </div>
        </header>
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Content {
    blog_metadata: Rc<BlogListMetadata>,
    project_metadata: Rc<ProjectsMetadata>,
    url: Rc<String>,
}

#[function_component(App)]
pub fn app() -> Html {
    let content = Rc::new(use_state(|| Content {
        blog_metadata: BlogListMetadata { blogs: vec![] }.into(),
        project_metadata: ProjectsMetadata { project_groups: vec![] }.into(),
        url: web_sys::window().unwrap().location().origin().unwrap().into(),
    }));
    {
        let content = content.clone();
        use_effect_with_deps(
            |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let url_rc = content.url.clone();
                    let url = url_rc.as_str();
                    let blogs_toml = Request::new(&format!("{url}/static/content/blogs.toml"))
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    let projects_toml = Request::new(&format!("{url}/static/content/projects.toml"))
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    content.set(Content {
                        blog_metadata: BlogListMetadata::from_text(&blogs_toml).into(),
                        project_metadata: ProjectsMetadata::from_text(&projects_toml).into(),
                        url: url_rc,
                    });
                });
            },
            (),
        );
    }

    html! {
        <ContextProvider<Rc<UseStateHandle<Content>>> context={content}>
            <BrowserRouter>
                <Header />
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<Rc<UseStateHandle<Content>>>>
    }
}
// This is the entry point for the web app
fn main() {
    yew::Renderer::<App>::new().render();
}
