// Copyright (c) 2021 Michael J. Simms. All rights reserved.

extern crate gpx;

mod utils;
mod location_analyzer;

use wasm_bindgen::prelude::*;

use std::io::BufReader;
use gpx::read;
use gpx::{Gpx, TrackSegment};
use gpx::errors::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct AnalysisReport {
    pub start_time_ms: f64,
    pub end_time_ms: f64,
    pub total_distance: f64,
    pub total_vertical: f64,
    pub avg_speed: f64,
    pub best_1k: f64,
    pub best_mile: f64,
    pub best_5k: f64,
    pub best_10k: f64,
    pub best_15k: f64,
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
    let mut analysis_report = AnalysisReport{ start_time_ms: 0.0, end_time_ms: 0.0, total_distance: 0.0, total_vertical: 0.0, avg_speed: 0.0, best_1k:0.0, best_mile: 0.0, best_5k: 0.0, best_10k: 0.0, best_15k: 0.0 };
    let data = BufReader::new(s.as_bytes());
    let res: Result<Gpx> = read(data);

    match res {
        Ok(gpx) => {
            let mut analyzer = location_analyzer::LocationAnalyzer::new();

            // Iterate through the tracks.
            for track in gpx.tracks {

                // Get the track name.
                match &track._type {
                    Some(activity_type) => analyzer.set_activity_type(activity_type.to_string()),
                    _ => {},
                }

                let segment: &TrackSegment = &track.segments[0];
                let points = &segment.points;

                // Iterate through the points.
                for point in points {
                    let time = point.time.unwrap().timestamp();
                    let lat = point.point().x();
                    let lon = point.point().y();
                    let alt = point.elevation.unwrap();

                    analyzer.append_location((time * 1000) as u64, lat, lon, alt);
                    analyzer.update_speeds();
                }

                // For calculations that only make sense once all the points have been added.
                analyzer.analyze();
            }

            // Copy items to the final report.
            analysis_report.start_time_ms = analyzer.start_time_ms as f64;
            analysis_report.end_time_ms = analyzer.last_time_ms as f64;
            analysis_report.total_distance = analyzer.total_distance;
            analysis_report.total_vertical = analyzer.total_vertical;
            analysis_report.avg_speed = analyzer.avg_speed;
            analysis_report.best_1k = analyzer.get_best_time(location_analyzer::BEST_1K) as f64;
            analysis_report.best_mile = analyzer.get_best_time(location_analyzer::BEST_MILE) as f64;
            analysis_report.best_5k = analyzer.get_best_time(location_analyzer::BEST_5K) as f64;
            analysis_report.best_10k = analyzer.get_best_time(location_analyzer::BEST_10K) as f64;
            analysis_report.best_15k = analyzer.get_best_time(location_analyzer::BEST_15K) as f64;
        }
        Err(_e) => {
            alert("Error parsing GPX file.");
        }
    }

    analysis_report
}

#[wasm_bindgen]
pub fn analyze_tcx(_s: &str) -> AnalysisReport {
    let analysis_report = AnalysisReport{ start_time_ms: 0.0, end_time_ms: 0.0, total_distance: 0.0, total_vertical: 0.0, avg_speed: 0.0, best_1k:0.0, best_mile: 0.0, best_5k: 0.0, best_10k: 0.0, best_15k: 0.0 };
    analysis_report
}
