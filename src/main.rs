use yew::prelude::*;

// use stdweb::unstable::TryInto;
// use stdweb::web::{ImageData, CanvasRenderingContext2d, document};
// use stdweb::web::html_element::{CanvasElement, InputElement};
// use stdweb::js;
//use wasm_bindgen::JsCast;

use web_sys::{HtmlInputElement, HtmlCanvasElement};

// https://rustwasm.github.io/book/game-of-life/implementing.html

//use gloo_file::callbacks::FileReader;
use gloo_file::File;
use gloo::timers::callback::Timeout;

mod canvas;
mod convert;


/* example for file upload:
https://github.com/yewstack/yew/blob/master/examples/file_upload/src/main.rs
*/

type Data = Vec<u8>;

pub enum Msg {
    Convert,
    ReadingDone(String, Data),
    RequestPreview(usize, u32),
    RunPreview(usize, u32),
    Files(Vec<File>),
}

#[derive(Debug)]
pub enum State {
    WaitingForSelection,
    ReadingImages(Vec<gloo_file::callbacks::FileReader>),
    Preview(usize, u32),
    Done,
}


pub struct Model {
    state: State,
    images: Vec<(String, Data)>,
    canvas: Option<HtmlCanvasElement>,
    timeout: Option<Timeout>,
    converting: bool,
}

/// ask user to select files
fn get_files(e: Event) -> Msg {
    let mut result = Vec::new();
    let input: HtmlInputElement = e.target_unchecked_into();

    if let Some(files) = input.files() {
        let files = js_sys::try_iter(&files)
            .unwrap()
            .unwrap()
            .map(|v| web_sys::File::from(v.unwrap()))
            .map(File::from);
        result.extend(files);
    }
    Msg::Files(result)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Model {
            state : State::WaitingForSelection,
            images: vec![],
            canvas: None,
            timeout: None,
            converting: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Files(files) => {
                // when the use choose the files, start converting them
                let mut readers = vec![];

                for file in files.into_iter(){
                    let file_name = file.name().clone();
                    let link = ctx.link().clone();

                    let task = gloo_file::callbacks::read_as_bytes(&file, move |res| {
                        link.send_message(Msg::ReadingDone(
                                file_name,
                                res.expect("failed to read file"),
                        ))
                    });
                    readers.push(task);
                }
                self.state = State::ReadingImages(readers);
            }
            Msg::ReadingDone(name, result) => {
                // message received when a file is red
                if let State::ReadingImages(tasks) = &self.state {
                    // add the data to the list
                    self.images.push((name, result));
                    if self.images.len() == tasks.len() {
                        ctx.link().send_message(Msg::RequestPreview(0, 50))
                    }
                }
            }
            Msg::RequestPreview(selected, q) => {
                // create canvas, display message and send message to launch preview
                let canvas = self.canvas.clone().unwrap_or_else(
                    || canvas::init_canvas("canvas"));
                

                let canvas_ctx = canvas::get_ctx(&canvas);
                canvas_ctx.clear_rect(0.0, 0.0, 1500.0, 15000.0);

                self.canvas = Some(canvas);

                let t = {
                    let link = ctx.link().clone();
                    Timeout::new(50, move || link.send_message(Msg::RunPreview(selected, q)))
                };

                self.timeout = Some(t);
                self.converting = true;
                }

            Msg::RunPreview(selected, quality) => {
                // preview a file
                self.state = State::Preview(selected, quality);
                convert::test_display_image(&self.images[selected].1, quality, self.canvas.as_ref().expect("no canvas !!!"));
                self.converting = false;
            }
            Msg::Convert => {
                // convert all the images and zip them
            },
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ui_top = match self.state {
            State::WaitingForSelection => {
                html! {<>
                    <h3>{ "Choisissez vos fichiers: " }</h3>
                        <div>
                        // TODO: file input style
                        //<label class="button" for="upload">{"Choisissez vos fichiers"}</label>
                        <input type="file" multiple=true onchange={ctx.link().callback(get_files) }/>
                        </div>
                        </>
                }
            }
            State::ReadingImages(_) => html! {<h3> {"lecture des image en cours ..."} </h3>},
            State::Preview(_, _) => html! {},
            State::Done => html! {},
        };

        let ui_bottom = match self.state {
            State::Preview(_, _) if self.converting => 
                html!{<center> <h1 style="color:red"> {". . ."} </h1> </center>},

            State::Preview(selected, q) => {
                let n_images = self.images.len();
                let next = move |_| Msg::RequestPreview((selected+1)%n_images, q);
                let prev = move |_| Msg::RequestPreview((selected-1+n_images)%n_images, q);
                let qual = move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    Msg::RequestPreview(selected, input.value_as_number() as u32)
                };
                html! {
                    <div>
                        <button onclick={ctx.link().callback(prev)}> {"<--"} </button>
                        <input onchange={ctx.link().callback(qual)} type="range" min="1" max="100" value={q.to_string()} class="slider"/>
                        <button onclick={ctx.link().callback(next)}> {"-->"} </button>
                        <button onclick={ctx.link().callback(|_| Msg::Convert)}> {"convert"} </button>
                        </div>
                }
            },
            _ => html!{},
        };

        // let image_names: Html = self.images.iter().map(|(name, _data)| html! {
        //     <pre> {name} </pre>
        // }
        // ).collect();
        
        // TODO: images mini preview ?

        html! {
            <div>
                <h2> {"Convertisseur d'images"} </h2>
                {ui_top}
                <canvas id="canvas" ></canvas>
                {ui_bottom}
                //{image_names}
                </div>
        }
    }
}


fn main() {
    yew::start_app::<Model>();
}
