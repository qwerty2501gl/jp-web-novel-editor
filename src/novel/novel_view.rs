use dioxus::prelude::*;
use jp_web_novel_text::{
    DictionaryPhrase, DictionaryWord, NewLinePhrase, Parser, Phrase, PlainPhrase, RubyPhrase,
    WhiteSpacePhrase, WhiteSpaceType,
};

#[derive(PartialEq, Clone, Props)]
pub struct NovelInputProps {
    novel_text: Signal<String>,
    input_count: Signal<usize>,
}

#[component]
pub fn NovelView(props: NovelInputProps) -> Element {
    let parser = Parser::default();
    let novel_element = rsx! {
        div{
            class:"novel-view",
            div{
                class:"text-view-area",
                {render_phrases(&parser,&props.novel_text.read())},
            }
        }
    };
    rsx! {
            {novel_element}
    }
}

pub fn render_phrases(parser: &Parser, input: &str) -> Element {
    rsx! {
        for frag in parser.parse_iter(input){
            {
                match frag.phrase() {
                    Phrase::Plain(phrase) => render_plain(phrase),
                    Phrase::Ruby(phrase) => render_ruby(phrase),
                    Phrase::NewLine(phrase) => render_new_line(phrase),
                    Phrase::WhiteSpace(phrase) => render_white_space(phrase),
                    Phrase::DictionaryWord(phrase) => render_dictionary_word(phrase),
                }
            }
        }
    }
}

fn render_plain(phrase: &PlainPhrase<&str>) -> Element {
    rsx! {
        span{
            {*phrase.target()}
        }
    }
}

fn render_ruby(phrase: &RubyPhrase<&str>) -> Element {
    rsx! {
        ruby{
            {*phrase.target()},
            rp{"("},
            rt{
                {*phrase.ruby()}
            },
            rp{")"},
        }
    }
}

fn render_new_line(_: &NewLinePhrase) -> Element {
    rsx! {
        br{}
    }
}

fn render_white_space(phrase: &WhiteSpacePhrase) -> Element {
    rsx! {
        span{
            style:{"margin-left:".to_string() + &( match phrase.white_space_type(){
                WhiteSpaceType::Space => 1,
                WhiteSpaceType::ZenkakuSpace => 2,
                WhiteSpaceType::Tab => 4,
            } * phrase.count() * 10
           ).to_string() + "px"},

        }
    }
}

fn render_dictionary_word(phrase: &DictionaryPhrase<&str, &DictionaryWord>) -> Element {
    unimplemented!()
}
