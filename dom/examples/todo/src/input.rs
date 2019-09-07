use {
    moxie_dom::{element, prelude::*, sys},
    wasm_bindgen::JsCast,
};

#[topo::aware]
pub fn text_input(placeholder: &str, editing: bool, on_save: impl FnOnce(String) + 'static) {
    let text = state!(|| if editing {
        placeholder.clone()
    } else {
        String::new()
    });

    fn input_value(ev: impl AsRef<sys::Event>) -> String {
        let event: &sys::Event = ev.as_ref();
        let target = event.target().unwrap();
        let input: sys::HtmlInputElement = target.dyn_into().unwrap();
        let val = input.value();
        input.set_value(""); // it's a little weird to clear the text every time, TODO clean up
        val
    }

    let mut elem = element!("input")
        .attr("autoFocus", "true")
        .attr("class", "new-todo")
        .attr("placeholder", placeholder)
        .attr("type", "text")
        .attr("value", &*text)
        .on(
            |change: ChangeEvent, _| Some(input_value(change)),
            text.clone(),
        )
        .on(
            move |keypress: KeyDownEvent, _| {
                if keypress.key() == "Enter" {
                    let value = input_value(keypress);
                    if !value.is_empty() {
                        on_save(value);
                    }
                    Some("".into())
                } else {
                    None
                }
            },
            text,
        );

    if editing {
        elem = elem.attr("class", "edit");
    }
}
