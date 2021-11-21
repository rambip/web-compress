use web_sys::{window, HtmlAnchorElement};
use wasm_bindgen::JsCast;

pub fn download(url: &str, name: &str) {
    let link = window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .expect("cannot create link");

    link.set_href(url);
    link.set_download(name);
    link.click()
}
