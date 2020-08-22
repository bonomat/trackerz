use crate::data::track_details::{TrackDetail, Trackz};
use futures::future::try_join_all;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

const TRACKZ_PATH: &str = "trackz/trackz.json";

pub async fn load_tracks() -> Result<Option<Vec<TrackDetail>>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(TRACKZ_PATH, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);
    let future = JsFuture::from(request_promise);

    let resp = future.await?;
    let resp: Response = resp.dyn_into().expect("response not working...");

    if let Ok(json) = resp.json() {
        if let Ok(json) = JsFuture::from(json).await {
            match json.into_serde::<Trackz>() {
                Ok(trackz) => {
                    let mut track_details_futures = vec![];
                    let mut vals = trackz.gpx.into_iter().peekable();
                    while let Some(v) = vals.next() {
                        let string = v.clone();
                        track_details_futures.push(load_track_details(string));
                    }
                    let mut vals = trackz.kml.into_iter().peekable();
                    while let Some(v) = vals.next() {
                        let string = v.clone();
                        track_details_futures.push(load_track_details(string));
                    }
                    let details = try_join_all(track_details_futures).await.unwrap();
                    let track_details = details.into_iter().filter_map(|track| track).collect();

                    Ok(Some(track_details))
                }
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

pub async fn load_track_details(url: String) -> Result<Option<TrackDetail>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

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
