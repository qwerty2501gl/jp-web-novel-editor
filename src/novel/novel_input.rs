use dioxus::{logger::tracing::debug, prelude::*, web::WebEventExt};
use gloo_timers::callback::Timeout;
use std::cmp::min;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
#[derive(PartialEq, Clone, Props)]
pub struct NovelInputProps {
    novel_text: Signal<String>,
    input_count: Signal<usize>,
}

#[component]
pub fn NovelInput(mut props: NovelInputProps) -> Element {
    let mut text_area = use_signal(|| None);

    rsx! {
        div{
            class:"novel-input",
            div{
                contenteditable:true,
                id:"novel-text-area",
                class:"text-area",
                onmounted: move |element|{
                    text_area.set(Some(element));
                },
                oninput: move |_| {
                    *props.input_count.write()+=1;
                    let timeout = Timeout::new(min(props.novel_text.read().len() / 5,300) as u32,move ||{
                        if *props.input_count.read() > 0{
                            *props.input_count.write() -=1;
                        }

                        if *props.input_count.read() == 0 && let Some(text_area) = text_area.cloned(){
                            let element = text_area.as_web_event();
                            let element = element.dyn_into::<HtmlElement>().unwrap();

                            let current_text = element.inner_text();
                            props.novel_text.set(current_text);
                        }

                    });
                    timeout.forget();
                },
            }
        }
    }
}
