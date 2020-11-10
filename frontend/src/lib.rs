use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_logger;
use log;
use yew::prelude::*;
use yewtil::future::LinkFuture;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::ConsoleService;
use sentinel::Camera;
use web_sys::{Request, RequestInit, RequestMode, Response};
// use web_sys::{Request, RequestInit, RequestMode, Response, view}; // ?

mod view_camera;

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

/// The possible states a fetch request can be in.
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

async fn fetch_resource(url: &'static str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    // opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = yew::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    cameras: FetchState<String>
}

enum Msg {
    SetFetchStateCameras(FetchState<String>),
    GetCameras,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            cameras: FetchState::Fetching
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetFetchStateCameras(state) => {
                self.cameras = state;
                ()
            },
            Msg::GetCameras => {
                self.link.send_future(async {
                    match fetch_resource("http://localhost:8000/cameras").await {
                        Ok(res) => {
                            Msg::SetFetchStateCameras(FetchState::Success(res))
                        },
                        Err(err) => Msg::SetFetchStateCameras(FetchState::Failed(err)),
                    }
                })
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        match &self.cameras {
            FetchState::NotFetching => html! { "Not fetching" },
            FetchState::Fetching => html! {
                <>
                    <button onclick=self.link.callback(|_| Msg::GetCameras)>
                        { "Get Cameras" }
                    </button>
                </>
            },
            FetchState::Success(data) => {
                log::debug!("{:?}",data);
                view_camera::index(&data)
            },
            FetchState::Failed(err) => html! { "Failed" },
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}
