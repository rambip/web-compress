use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d, ImageData};
use wasm_bindgen::{JsCast, Clamped};
use yew::NodeRef;

pub fn init_canvas(canvas_node: &NodeRef) -> CanvasRenderingContext2d {
    let canvas = canvas_node.cast::<HtmlCanvasElement>()
                .unwrap();

    let raw_width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;
    let raw_height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;

    canvas.set_width(u32::max(700, raw_width - 200));
    canvas.set_height(u32::max(500, raw_height - 230));

    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

pub fn clear_canvas(ctx: &CanvasRenderingContext2d) {
    ctx.clear_rect(0.0, 0.0, 1500.0, 15000.0);
}

pub fn get_size(ctx: &CanvasRenderingContext2d) -> (u32, u32) {
    let canvas = ctx.canvas().unwrap();
    (canvas.width(), canvas.height())
}

pub fn draw_image(width: u32, height: u32, mut data: &[u8], ctx: &CanvasRenderingContext2d){
    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut data),
        width, height
    ).unwrap();

    // draw image on the canvas
    ctx.put_image_data(&new_img_data, 0.0, 0.0)
        .expect("cannot put image data on canvas");
}
