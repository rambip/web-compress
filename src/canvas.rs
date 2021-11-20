use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::JsCast;


pub fn init_canvas(id: &str) -> HtmlCanvasElement {
    let canvas = window().unwrap().document().unwrap()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let raw_width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;
    let raw_height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;

    canvas.set_width(u32::max(700, raw_width - 200));
    canvas.set_height(u32::max(500, raw_height - 230));

    canvas
}

pub fn get_ctx(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}
