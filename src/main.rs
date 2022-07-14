// fix recursion bug for macros
// see https://github.com/yewstack/yew/issues/513
#![recursion_limit = "128"]


use yew::prelude::*;
use std::rc::Rc;

use web_sys::{HtmlInputElement, CanvasRenderingContext2d};

// https://rustwasm.github.io/book/game-of-life/implementing.html

use gloo_file::{File, Blob};
use gloo::timers::callback::Timeout;

use image::RgbaImage;

mod canvas;
mod convert;
mod download;


pub enum Msg {
    Restart,
    Files(Vec<File>),
    ReadingDone(String, FileData),

    RequestPreview(usize, u8),
    RunPreview(usize, u8),

    RequestConvert(u8),
    RunConvert(u8),

    Download(Rc<Blob>),
}

#[derive(Debug)]
pub enum State {
    WaitingForSelection,
    ReadingImages(Vec<gloo_file::callbacks::FileReader>),
    CreatingPreview(usize, u8),
    ShowingPreview(usize, u8, usize),
    Converting,
    WaitingForDownload(Rc<Blob>),
    Done(gloo_file::callbacks::FileReader),
}


pub enum FileData {
    Encoded(Vec<u8>),
    Decoded(RgbaImage),
}

impl FileData {
    // get the decoded image from the file.
    // If it is encoded, first convert it, store the result and return a reference
    pub fn get_decoded(&mut self) -> &RgbaImage {
        match self {
            FileData::Encoded(im) => {
                let decoded = convert::read_image(&im);
                *self= FileData::Decoded(decoded);
            },
            FileData::Decoded(_) => () 
        };

        match self {
            FileData::Decoded(im) => im,
            FileData::Encoded(_) => unreachable!(),
        }
    }
}

pub struct Model {
    state: State,
    images: Vec<(String, FileData)>,
    canvas_node: NodeRef,
    canvas_ctx: Option<CanvasRenderingContext2d>,
    timeout: Option<Timeout>,
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Model {
            state : State::WaitingForSelection,
            images: vec![],
            timeout: None,
            canvas_node: NodeRef::default(),
            canvas_ctx: None,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.canvas_ctx = Some(canvas::init_canvas(&self.canvas_node));
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
                                FileData::Encoded(res.expect("failed to read file")),
                        ))
                    });
                    readers.push(task);
                }
                self.state = State::ReadingImages(readers);
            }

            // message received when a file is red
            Msg::ReadingDone(name, result) => {
                if let State::ReadingImages(tasks) = &self.state {
                    // add the data to the list
                    self.images.push((name, result));
                    if self.images.len() == tasks.len() {
                        ctx.link().send_message(Msg::RequestPreview(0, 50))
                    }
                }
            }

            // when all the images are red and each time the user clicks "next image"
            Msg::RequestPreview(selected, q) => {
                self.state = State::CreatingPreview(selected, q);
                self.send_message_timeout(ctx, Msg::RunPreview(selected, q));
            }

            // read an image and display it with the selected settings
            Msg::RunPreview(selected, quality) => {
                let new_size = convert::test_display_image(
                    self.images[selected].1.get_decoded(),
                    quality,
                    self.canvas_ctx.as_ref().unwrap()
                );

                self.state = State::ShowingPreview(selected, quality, new_size);
            }

            // when the use clicks on "convert"
            Msg::RequestConvert(quality) => {
                self.state = State::Converting;
                self.send_message_timeout(ctx, Msg::RunConvert(quality));
            },

            // converts all the images and put them in a blob
            Msg::RunConvert(quality) => {
                let result = convert::convert_and_zip_images(&self.images, quality);
                let blob = Blob::new_with_options(result.as_slice(), Some("zip"));
                self.state = State::WaitingForDownload(Rc::new(blob));
            }

            // download button
            Msg::Download(blob_ref) => {
                let task = gloo_file::callbacks::read_as_data_url(
                    &blob_ref,
                    |x| download::download(&x.unwrap(), "images.zip"));
                self.state = State::Done(task);
            },

            Msg::Restart => {
                self.state = State::WaitingForSelection;
                self.images.clear();
                self.timeout = None;
            },
        }
        true
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let ui = match &self.state {
            State::WaitingForSelection => html!{
                <div class="vert-menu">
                    <label class="button" for="upload">{"Choisissez vos fichiers"}</label>
                    <input id="upload" type="file" multiple=true onchange={ctx.link().callback(get_files) }/>
                    </div>
            },
            State::ReadingImages(_) => html! {
                <h3> {"lecture des image en cours ..."} </h3>
            },
            State::CreatingPreview(_, _) => html!{
                <center> <h1 style="color:red; font-size=4ex"> {". . ."} </h1> </center>
            },

            &State::ShowingPreview(selected, q, size) => {
                let link = ctx.link();
                let n_images = self.images.len();
                let next = move |_| Msg::RequestPreview((selected+1)%n_images, q);
                let prev = move |_| Msg::RequestPreview((selected-1+n_images)%n_images, q);
                let qual = move |e: Event| Msg::RequestPreview(
                    selected, 
                    e.target_unchecked_into::<HtmlInputElement>().value_as_number() as u8);

                html! {
                    <div>
                        <p> {format!("taille :  {} Ko", size/1000)} </p>
                        <div class="vert-menu">
                            <button onclick={link.callback(prev)}> {"<--"} </button>
                            <input onchange={link.callback(qual)} type="range" min="1" max="75" value={q.to_string()} class="slider"/>
                            <button onclick={link.callback(next)}> {"-->"} </button>
                            <button onclick={link.callback(move |_| Msg::RequestConvert(q))}> {"convertir"} </button>
                        </div>
                    </div>

                }
            },
            State::WaitingForDownload(blob_ref) => html! {
                <button onclick={ctx.link().callback_once({
                    let b = blob_ref.clone();
                    move |_| Msg::Download(b)
                })}> {"télécharger"} </button>
            }
            ,
            State::Converting => html ! {
                <p> {"conversion en cours ..."} </p>
            },
            State::Done(_) => html! {
                <div>
                    <p> {"Voici vos images !"} </p>
                    <button onclick={ctx.link().callback(|_| Msg::Restart)}>{"Recommencer"}</button>
                </div>
            },
        };

        html! {
            <div>
                <h2> {"Convertisseur d'images"} </h2> 
                {ui}
            <canvas id="canvas" ref={self.canvas_node.clone()} ></canvas>
                </div>
        }
    }
}

impl Model {
    fn send_message_timeout(&mut self, ctx: &Context<Self>, msg: Msg){
        canvas::clear_canvas(self.canvas_ctx.as_ref().unwrap());
        self.timeout = Some( {
            let link = ctx.link().clone();
            Timeout::new(50, move || link.send_message(msg))
        });
    }
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


fn main() {
    yew::start_app::<Model>();
}
