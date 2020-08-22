use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// A struct to hold some data from the github Branch API.
///
/// Note how we don't have to define every member -- serde will ignore extra
/// data when deserializing
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TrackDetails {
    pub file: String,
    pub title: String,
    pub description: String,
    pub county: String,
    pub state: String,
    pub track_type: String,
    pub difficulty: usize,
    pub track_time: usize,
    pub track_length: usize,
    pub start: String,
    pub end: String,
    pub adv_username: String,
    pub url: String,
    pub tags: Vec<String>,
}

pub async fn fetchit() -> Result<Option<TrackDetails>, JsValue> {
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
            match json.into_serde::<TrackDetails>() {
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