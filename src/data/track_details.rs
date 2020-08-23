use serde::{Deserialize, Serialize};

/// A collection of gpx/kml tracks
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Trackz {
    pub gpx: Vec<String>,
    pub kml: Vec<String>,
}

/// A struct to hold track details.
///
/// Note how we don't have to define every member -- serde will ignore extra
/// data when deserializing
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TrackDetail {
    pub file: String,
    pub title: String,
    pub description: String,
    pub country: String,
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
