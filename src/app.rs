use crate::components::button::BootstrapButton;
use crate::components::table::Table;
use crate::components::TableOptions;
use crate::connector::fetchit;
use crate::data::track_details::TrackDetail;
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
    track_detail: Vec<TrackDetail>,
}

impl Component for App {
    type Message = CallBackMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        spawn_local(async move {
            let option = fetchit().await.unwrap();
            debug!("received: {:?}", option);
        });

        let track_detail = vec![
            TrackDetail {
                file: "1".to_string(),
                title: "2".to_string(),
                description: "3".to_string(),
                county: "4".to_string(),
                state: "5".to_string(),
                track_type: "6".to_string(),
                difficulty: 0,
                track_time: 0,
                track_length: 0,
                start: "7".to_string(),
                end: "8".to_string(),
                adv_username: "9".to_string(),
                url: "10".to_string(),
                tags: vec![],
            },
            TrackDetail {
                file: "a".to_string(),
                title: "b".to_string(),
                description: "c".to_string(),
                county: "d".to_string(),
                state: "e".to_string(),
                track_type: "6".to_string(),
                difficulty: 0,
                track_time: 0,
                track_length: 0,
                start: "7".to_string(),
                end: "8".to_string(),
                adv_username: "9".to_string(),
                url: "10".to_string(),
                tags: vec![],
            },
        ];

        App { link, track_detail }
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
        let columns = columns![("file", "File Name")("title", "Title")(
            "description",
            "Description"
        )("county", "Country")];

        let options = TableOptions { orderable: true };

        html! {
            <>
                <Table columns=columns data=&self.track_detail, options=Some(options),/>
                <BootstrapButton onsignal=self.link.callback(|_| CallBackMsg::Add) title="Add route" />
                <BootstrapButton onsignal=self.link.callback(|_| CallBackMsg::Remove) title="Remove route" />
            </>
        }
    }
}
