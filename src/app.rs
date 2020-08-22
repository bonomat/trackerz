use crate::{
    components::{table::Table, Column, TableOptions},
    connector::load_tracks,
    data::track_details::TrackDetail,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub enum CallBackMsg {
    TrackzLoaded(Vec<TrackDetail>),
}

pub struct App {
    _link: ComponentLink<Self>,
    track_detail: Vec<TrackDetail>,
}

impl Component for App {
    type Message = CallBackMsg;
    type Properties = ();

    fn create(_properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        debug!("Called once or twice ");
        let callback = link
            .clone()
            .callback(|elements: Vec<TrackDetail>| CallBackMsg::TrackzLoaded(elements));
        spawn_local(async move {
            let trackz = load_tracks().await.unwrap().unwrap();
            debug!("received: {:?}", trackz);
            callback.emit(trackz)
        });

        App {
            _link: link,
            track_detail: vec![],
        }
    }

    fn update(&mut self, action: Self::Message) -> ShouldRender {
        match action {
            CallBackMsg::TrackzLoaded(trackz) => {
                let mut new_track_detail = self.track_detail.clone();
                trackz.iter().for_each(|track_detail| {
                    if !new_track_detail.contains(track_detail) {
                        new_track_detail.push(track_detail.clone());
                    }
                });
                self.track_detail = new_track_detail;
            }
        }
        true
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
