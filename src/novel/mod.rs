mod novel_input;
mod novel_view;

use novel_input::*;
use novel_view::*;

use dioxus::prelude::*;

#[component]
pub fn NovelEditor() -> Element {
    let novel_text = use_signal(String::default);
    let input_count = use_signal(|| 0);
    rsx! {
        div{
            class:"novel-editor",
            NovelInput{
                novel_text:novel_text,
                input_count:input_count,
            },
            NovelView{
                novel_text:novel_text,
                input_count:input_count,
            },
        }
    }
}
