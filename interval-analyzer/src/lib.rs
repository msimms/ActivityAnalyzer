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
pub struct AnalysisReport {
    pub start_time: f64,
    pub end_time: f64,
    pub total_distance: f64,
    pub total_vertical: f64,
    pub avg_speed: f64,
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Copyright (c) 2021 Michael J. Simms. All rights reserved.");
}

#[wasm_bindgen]
pub fn analyze_gpx(s: &str) -> AnalysisReport {
    let mut analysis_report = AnalysisReport{ start_time: 0.0, end_time: 0.0, total_distance: 0.0, total_vertical: 0.0, avg_speed: 0.0 };
    let data = BufReader::new(s.as_bytes());
    let res: Result<Gpx> = read(data);

    match res {
        Ok(gpx) => {
            let mut analyzer = location_analyzer::LocationAnalyzer::new();
            let track: &Track = &gpx.tracks[0];
            let segment: &TrackSegment = &track.segments[0];
            let points = &segment.points;

            for point in points {
                let time = point.time.unwrap().timestamp();
                let lat = point.point().x();
                let lon = point.point().y();
                let alt = point.elevation.unwrap();

                analyzer.append_location(time as u64, lat, lon, alt);
            }

            // Copy items to the final report.
            analysis_report.start_time = analyzer.start_time as f64;
            analysis_report.end_time = analyzer.last_time as f64;
            analysis_report.total_distance = analyzer.total_distance;
            analysis_report.total_vertical = analyzer.total_vertical;
            analysis_report.avg_speed = analyzer.avg_speed;
        }
        Err(e) => {
            alert("Error parsing GPX file.");
        }
    }

    analysis_report
}

#[wasm_bindgen]
pub fn analyze_tcx(s: &str) -> AnalysisReport {
    let mut analysis_report = AnalysisReport{ start_time: 0.0, end_time: 0.0, total_distance: 0.0, total_vertical: 0.0, avg_speed: 0.0 };
    analysis_report
}
