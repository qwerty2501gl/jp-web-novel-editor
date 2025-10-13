//use crate::menu::*;
use crate::app::document::Stylesheet;
use crate::novel::*;
use dioxus::prelude::*;

static MAIN_SCSS: Asset = asset!("assets/main.scss");
static TAILWIND_CSS: Asset = asset!("assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        Stylesheet { href: MAIN_SCSS },
        Stylesheet { href: TAILWIND_CSS},
        div{
            class:"app",
            NovelEditor{}
        }
    }
}
