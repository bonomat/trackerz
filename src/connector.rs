use crate::components::table::TableData;
use crate::data::track_details::TrackDetail;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub async fn fetchit() -> Result<Option<TrackDetail>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("trackz/gpx/KTM_Breakout_5.json", &opts).unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);
    let future = JsFuture::from(request_promise);

    let resp = future.await?;
    let resp: Response = resp.dyn_into().expect("response not working...");

    if let Ok(json) = resp.json() {
        if let Ok(json) = JsFuture::from(json).await {
            match json.into_serde::<TrackDetail>() {
                Ok(track_details) => Ok(Some(track_details)),
                Err(e) => {
                    debug!("Could not deserialize json file: {:?}", e);
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
