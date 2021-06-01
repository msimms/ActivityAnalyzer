// Copyright (c) 2021 Michael J. Simms. All rights reserved.
 #![allow(dead_code)]

use lib_math::{graphics};
use std::collections::HashMap;
use serde::Deserialize;

extern crate serde;
extern crate reqwest;

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

pub struct GeoJsonReader {
    world_features: Option<Feature>,
    us_features: Option<Feature>,
    canada_features: Option<Feature>,
}

impl GeoJsonReader {
    pub fn new() -> Self {
        let reader = GeoJsonReader{ world_features: None, us_features: None, canada_features: None };
        reader
    }

    /*fn download_map_data(url: &str) -> Result<String, reqwest::Error> {
        let resp = reqwest::blocking::get(url).unwrap().text().unwrap();
        Ok(resp)
    }*/

    /// Returns a dictionary that maps the name of the geo region to it's coordinates, as an array (or array of arrays) of lat/lon.
    pub fn name_to_coordinate_map(_features: Features) -> HashMap<String, Vec<graphics::Point>> {
        let geomap = HashMap::new();
        /*for feature in features.features {
        }*/
        geomap
    }

    pub fn initialize(&mut self) {
        /*let world_geo_json_url = "https://raw.githubusercontent.com/msimms/StraenWeb/master/data/world.geo.json";
        let us_geo_json_url = "https://raw.githubusercontent.com/msimms/StraenWeb/master/data/us_states.geo.json";
        let canada_geo_json_url = "https://raw.githubusercontent.com/msimms/StraenWeb/master/data/canada.geo.json";

        let world_features_str = GeoJsonReader::download_map_data(world_geo_json_url).unwrap();
        self.world_features = serde_json::from_str(&world_features_str).unwrap();

        let us_features_str = GeoJsonReader::download_map_data(us_geo_json_url).unwrap();
        self.us_features = serde_json::from_str(&us_features_str).unwrap();*/
    }
}
