// Copyright (c) 2021 Michael J. Simms. All rights reserved.

extern crate gpx;

mod utils;
mod location_analyzer;

use wasm_bindgen::prelude::*;

use std::io::BufReader;
use gpx::read;
use gpx::{Gpx, Track, TrackSegment};
use gpx::errors::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Copyright (c) 2021 Michael J. Simms. All rights reserved.");
}

#[wasm_bindgen]
pub fn analyze(s: &str) {
    let data = BufReader::new(s.as_bytes());
    let res: Result<Gpx> = read(data);

    match res {
        Ok(gpx) => {
            let track: &Track = &gpx.tracks[0];
            let segment: &TrackSegment = &track.segments[0];
            let points = &segment.points;

            for point in points {
                location_analyzer::append_location(&point.point().x(), &point.point().y(), &point.elevation.unwrap());
            }
        }
        Err(e) => {
        }
    }
}