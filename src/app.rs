use crate::components::button::BootstrapButton;
use crate::components::table::BootstrapTable;
use crate::connector::fetchit;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
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
}

impl Component for App {
    type Message = CallBackMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        spawn_local(async move {
            let option = fetchit().await.unwrap();
            debug!("received: {:?}", option);
        });

        App { link }
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
                    let js_value = read_gpx(location.as_str()).unwrap();
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
                <BootstrapTable/>

                <BootstrapButton onsignal=self.link.callback(|_| CallBackMsg::Add) title="Add route" />
                <BootstrapButton onsignal=self.link.callback(|_| CallBackMsg::Remove) title="Remove route" />
            </>
        }
    }
}
