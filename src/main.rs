use yew::prelude::*;

use web_sys::{HtmlInputElement, CanvasRenderingContext2d};

// https://rustwasm.github.io/book/game-of-life/implementing.html

use gloo_file::{File, Blob};
use gloo::timers::callback::Timeout;

use image::RgbaImage;

mod canvas;
mod convert;
mod download;


/* example for file upload:
https://github.com/yewstack/yew/blob/master/examples/file_upload/src/main.rs
*/

pub enum Msg {
    Restart,
    Files(Vec<File>),
    ReadingDone(String, FileData),
    
    RequestPreview(usize, u8),
    RunPreview(usize, u8),

    RequestConvert(u8),
    RunConvert(u8),

    RequestDownload,
    RunDownload,
}

#[derive(Debug)]
pub enum State {
    WaitingForSelection,
    ReadingImages(Vec<gloo_file::callbacks::FileReader>),
    CreatingPreview(usize, u8),
    ShowingPreview(usize, u8, usize),
    Converting,
    WaitingForDownload,
    Done(Option<gloo_file::callbacks::FileReader>),
}

pub enum FileData {
    Encoded(Vec<u8>),
    Decoded(RgbaImage),
}

pub struct Model {
    state: State,
    images: Vec<(String, FileData)>,
    canvas_node: NodeRef,
    canvas_ctx: Option<CanvasRenderingContext2d>,
    timeout: Option<Timeout>,
    blob: Option<gloo_file::Blob>,
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
            timeout: None,
            canvas_node: NodeRef::default(),
            canvas_ctx: None,
            blob: None,
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
                self.state = State::CreatingPreview(selected, q);

                self.send_message_timeout(ctx, Msg::RunPreview(selected, q));
                }

            Msg::RunPreview(selected, quality) => {
                // TODO: fix with https://ricardomartins.cc/2016/06/08/interior-mutability
                // preview a file
                match &self.images[selected].1 {
                    FileData::Encoded(im) => {
                        let decoded = convert::read_image(&im);
                        self.images[selected].1 = FileData::Decoded(decoded);
                    },
                    FileData::Decoded(_) => () 
                };

                if let FileData::Decoded(d) = &self.images[selected].1 {
                    let new_size = convert::test_display_image(&d, quality, self.canvas_ctx.as_ref().unwrap());
                    self.state = State::ShowingPreview(selected, quality, new_size);
                }
            }
            Msg::RequestConvert(quality) => {
                self.state = State::Converting;

                canvas::clear_canvas(self.canvas_ctx.as_ref().unwrap());
                self.send_message_timeout(ctx, Msg::RunConvert(quality));
            },

            Msg::RunConvert(quality) => {
                let result = convert::convert_and_zip_images(&self.images, quality);
                let blob = Blob::new_with_options(result.as_slice(), Some("zip"));
                self.blob = Some(blob);
                self.state = State::WaitingForDownload;
            }

            Msg::Restart => {
                self.state = State::WaitingForSelection;
                self.images.clear();
                self.timeout = None;
            },
            Msg::RequestDownload => {
                self.state = State::Done(None);

                self.send_message_timeout(ctx, Msg::RunDownload);
            },
            Msg::RunDownload => {
                let task = gloo_file::callbacks::read_as_data_url(&self.blob.as_ref().unwrap(),
                |x| download::download(&x.unwrap(), "images.zip"));
                self.state = State::Done(Some(task));
            }
        }
        true
    }


    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let image_names: Html = self.images.iter().map(|(name, _data)| html! {
        //     <pre> {name} </pre>
        // }
        // ).collect();
        
        let ui = match &self.state {
            State::WaitingForSelection => html!{
                <>
                    <div>
                        <h3>{ "Choisissez vos fichiers: " }</h3></div>
                        <div>
                        // TODO: file input style
                        //<label class="button" for="upload">{"Choisissez vos fichiers"}</label>
                        <input type="file" multiple=true onchange={ctx.link().callback(get_files) }/>
                        </div>
                        </>
            },
            State::ReadingImages(_) => html! {
                    <h3> {"lecture des image en cours ..."} </h3>
            },
            State::CreatingPreview(_, _) => html!{
                <center> <h1 style="color:red"> {". . ."} </h1> </center>
            },

            &State::ShowingPreview(selected, q, size) => {
                let n_images = self.images.len();
                let next = move |_| Msg::RequestPreview((selected+1)%n_images, q);
                let prev = move |_| Msg::RequestPreview((selected-1+n_images)%n_images, q);
                let qual = move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    Msg::RequestPreview(selected, input.value_as_number() as u8)
                };
                html! {
                    <div>
                        <p> {format!("size = {}", size)} </p>
                        <button onclick={ctx.link().callback(prev)}> {"<--"} </button>
                        <input onchange={ctx.link().callback(qual)} type="range" min="1" max="100" value={q.to_string()} class="slider"/>
                        <button onclick={ctx.link().callback(next)}> {"-->"} </button>
                        <button onclick={ctx.link().callback(move |_| Msg::RequestConvert(q))}> {"convert"} </button>
                        </div>

                }
            },
            State::WaitingForDownload => html! {
                <button onclick={ctx.link().callback(move |_| Msg::RequestDownload)}> {"télécharger"} </button>
            },
            State::Converting => html ! {
                <p> {"conversion en cours ..."} </p>
            },
            State::Done(_t) => html! {
                <p> {"Voici vos images !"} </p>
            },
        };

        //<span><button onclick={ctx.link().callback(|_| Msg::Restart)}>{"Réinitialiser"}</button> </span>

        html! {
            <div>
                <h2> {"Convertisseur d'images"} </h2> 
                {ui}
                <canvas id="canvas" ref={self.canvas_node.clone()} ></canvas>
                //{image_names}
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

fn main() {
    yew::start_app::<Model>();
}
