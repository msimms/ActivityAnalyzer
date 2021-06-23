// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use serde_derive::{Deserialize};
use std::io::Read;
use std::io::BufReader;

extern crate serde;
extern crate serde_xml_rs;

#[derive(Deserialize, Debug, Default)]
pub struct TrackPoint {
    #[serde(rename="lat")]
    pub lat: f64,
    #[serde(rename="lon")]
    pub lon: f64,
    #[serde(rename="ele")]
    pub ele: f64,
}

#[derive(Deserialize, Debug)]
pub struct TrackSegment {
    #[serde(rename="trkpt")]
    pub points: Vec<TrackPoint>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Track {
    #[serde(rename="trkseg")]
    pub segments: Vec<TrackSegment>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Metadata {
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename="gpx")]
pub struct GpxRoute {
    #[serde(rename="metadata")]
    pub metadata: Option<Metadata>,
    #[serde(rename="trk")]
    pub tracks: Option<Track>,
}

pub fn read<R: Read>(reader: &mut BufReader<R>) -> Result<GpxRoute, serde_xml_rs::Error> {
    let gpx_route = serde_xml_rs::from_reader(reader);
    gpx_route
}
