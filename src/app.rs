use crate::components::button::BootstrapButton;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/js/wasm_bridge.js")]
extern "C" {
    fn read_gpx(url: &str) -> JsValue;
    fn remove(layer: &JsValue);
}

pub enum CallBackMsg {
    Add,
    Remove
}

pub struct App {
    link: ComponentLink<Self>,
    layer: Option<JsValue>
}

impl Component for App {
    type Message = CallBackMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            layer: None
        }
    }

    fn update(&mut self, action: Self::Message) -> ShouldRender {
        let window = web_sys::window().expect("no global `window` exists");
        match action {
            CallBackMsg::Add => {
                let location = window.location();
                let mut location: String = location.href().expect("To get URL");
                debug!("{:?}", location);
                location.push_str("trackz/gpx/20140124_110945_brisbane-to-sydney-adventure-ride.gpx");
                let js_value = read_gpx(location.as_str());
                // let js_value: String = js_value.into_serde().unwrap();
                // debug!("received: {:?}", js_value);
                self.layer = Some(js_value);
            }
            CallBackMsg::Remove => {
                let value = self.layer.as_ref().unwrap();
                remove(value);
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
