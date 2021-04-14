// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use lib_math::{graphics};
use reqwest::*;
use std::collections::HashMap;
use serde::Deserialize;

extern crate serde;

#[derive(Debug, Deserialize)]
struct Properties {
    #[serde(rename="name")]
    name: String,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    #[serde(rename="type")]
    geo_type: String,
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
struct Features {
    #[serde(rename="features")]
    features: Vec<Feature>,
}

pub struct GeoJsonReader {
}

impl GeoJsonReader {
    pub fn new() -> Self {
        let reader = GeoJsonReader{};
        reader
    }

    fn download_map_data(remote_file_name: &str) {
    }

    /// Returns a dictionary that maps the name of the geo region to it's coordinates, as an array (or array of arrays) of lat/lon.
    pub fn name_to_coordinate_map(data: &HashMap<String, Vec<graphics::Point>>) {
    }

    pub fn initialize() {
        let world_geo_json = "https://raw.githubusercontent.com/msimms/StraenWeb/master/data/world.geo.json";
        let us_geo_json = "https://raw.githubusercontent.com/msimms/StraenWeb/master/data/us_states.geo.json";
        let canada_geo_json = "https://raw.githubusercontent.com/msimms/StraenWeb/master/data/canada.geo.json";

        GeoJsonReader::download_map_data(world_geo_json);
    }
}
