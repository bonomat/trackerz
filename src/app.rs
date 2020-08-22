use crate::components::table::Table;
use crate::components::Column;
use crate::components::TableOptions;
use crate::connector::fetchit;
use crate::data::track_details::TrackDetail;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub enum CallBackMsg {}

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
                file: "KTM_Breakout_5.gpx".to_string(),
                title: "A GPX file".to_string(),
                description: "3".to_string(),
                country: "4".to_string(),
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
                file: "Hill End Camping _ Putty National Park.kml".to_string(),
                title: "A KML file with spaces".to_string(),
                description: "c".to_string(),
                country: "d".to_string(),
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

    fn update(&mut self, _action: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let columns = vec![
            Column::new(Some("title".into()), "Title".into()),
            Column::new(Some("description".into()), "Description".into()),
            Column::new(Some("country".into()), "Country".into()),
            Column::new(Some("show".into()), "Show".into()),
            Column::new(Some("hide".into()), "Hide".into()),
        ];

        let options = TableOptions { orderable: true };

        html! {
            <>
                <Table columns=columns data=&self.track_detail, options=Some(options),/>
            </>
        }
    }
}
