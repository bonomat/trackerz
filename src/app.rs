use crate::components::button::BootstrapButton;
use crate::connector;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use wasm_timer::Delay;
use yew::prelude::*;

#[wasm_bindgen(module = "/js/wasm_bridge.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn read_gpx(url: &str) -> Result<JsValue, JsValue>;
    fn remove(layer: &str);
}

pub enum CallBackMsg {
    Add,
    Remove,
}

pub struct App {
    link: ComponentLink<Self>,
    layer: Option<JsValue>,
}

impl Component for App {
    type Message = CallBackMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link, layer: None }
    }

    fn update(&mut self, action: Self::Message) -> ShouldRender {
        let window = web_sys::window().expect("no global `window` exists");
        let location = window.location();
        let mut location: String = location.href().expect("To get URL");
        match action {
            CallBackMsg::Add => {
                spawn_local(async move {
                    location.push_str(
                        "trackz/gpx/20140124_110945_brisbane-to-sydney-adventure-ride.gpx",
                    );
                    let js_value = read_gpx(location.as_str()).await.unwrap();
                    debug!("received: {:?}", js_value);
                });
            }
            CallBackMsg::Remove => {
                remove("latest");
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <BootstrapButton onsignal=self.link.callback(|_| CallBackMsg::Add) title="Add route" />
                <BootstrapButton onsignal=self.link.callback(|_| CallBackMsg::Remove) title="Remove route" />
            </>
        }
    }
}
