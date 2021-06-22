// Copyright (c) 2021 Michael J. Simms. All rights reserved.
 #![allow(dead_code)]

use lib_math::{graphics};
use std::collections::HashMap;
use serde::Deserialize;

extern crate serde;

#[derive(Debug, Deserialize)]
struct Coordinates {
}

#[derive(Debug, Deserialize)]
struct Properties {
    #[serde(rename="name")]
    name: String,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    #[serde(rename="type")]
    geo_type: String,
    #[serde(rename="coordinates")]
    coordinates: Coordinates,
}

#[derive(Debug, Deserialize)]
struct Feature {
    #[serde(rename="type")]
    feature_type: String,
    #[serde(rename="properties")]
    properties: Properties,
    #[serde(rename="geometry")]
    geometry: Geometry,
}

#[derive(Debug, Deserialize)]
pub struct Features {
    #[serde(rename="features")]
    features: Vec<Feature>,
}

pub struct GeoJson {
    pub world_features: Option<Features>,
    pub us_features: Option<Features>,
}

impl GeoJson {
    pub fn new() -> Self {
        let reader = GeoJson{ world_features: None, us_features: None };
        reader
    }

    pub fn load_world_data(&mut self, s: &str) {
        self.world_features = serde_json::from_str(s).unwrap();
    }

    pub fn load_us_data(&mut self, s: &str) {
        self.us_features = serde_json::from_str(s).unwrap();
    }

    /// Returns a dictionary that maps the name of the geo region to it's coordinates, as an array (or array of arrays) of lat/lon.
    pub fn name_to_coordinate_map(&mut self, _features: Features) -> HashMap<String, Vec<graphics::Point>> {
        let geomap = HashMap::new();
        /*for feature in features.features {
        }*/
        geomap
    }
}
