use crate::components::button::BootstrapButton;
use crate::data::geojson::*;
use load_dotenv::load_dotenv;
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::StorageService;

const GEOJSON_KEY: &'static str = "geojsonData";
load_dotenv!();

#[wasm_bindgen(module = "/js/wasm_bridge.js")]
extern "C" {
    fn read_gpx(url: &str) -> JsValue;
    fn remove();
}

pub enum CallBackMsg {
    Add,
    Remove
}

pub struct App {
    link: ComponentLink<Self>,
    counter: i32,
    storage: StorageService,
    geo_data: Vec<Feature>,
    position: Vec<f64>,
}

impl Component for App {
    type Message = CallBackMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Session).expect("storage was disabled by the user");
        let Json(geo_data) = storage.restore(GEOJSON_KEY);
        let geo_data = geo_data.unwrap_or_else(|_| Vec::new());

        let lat = env!("LATITUDE", "Cound not find LATITUDE in .env");
        let lng = env!("LONGITUDE", "Cound not find LONGITUDE in .env");
        let lat: f64 = str2f64(lat);
        let lng: f64 = str2f64(lng);

        let position = vec![lng, lat];
        App {
            link,
            counter: 0,
            storage,
            geo_data,
            position,
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
                debug!("received: {:?}", js_value);
            }
            CallBackMsg::Remove => {
                remove();
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

fn str2f64(s: &str) -> f64 {
    s.trim().parse().expect("Failed parsing a String to f64")
}
