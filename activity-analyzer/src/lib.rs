// Copyright (c) 2021 Michael J. Simms. All rights reserved.

extern crate gpx;
extern crate serde;
extern crate serde_json;
extern crate tcx;

mod utils;
mod cadence_analyzer;
mod location_analyzer;
mod power_analyzer;
mod heart_rate_analyzer;

use wasm_bindgen::prelude::*;
use std::io::BufReader;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
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

fn make_final_report(analyzer: &location_analyzer::LocationAnalyzer, power_analyzer: Option<&power_analyzer::PowerAnalyzer>, cadence_analyzer: Option<&cadence_analyzer::CadenceAnalyzer>, hr_analyzer: Option<&heart_rate_analyzer::HeartRateAnalyzer>) -> String {
    let mut max_power = 0.0;
    let mut avg_power = 0.0;
    let mut best_5_sec_power = 0.0;
    let mut best_12_min_power = 0.0;
    let mut best_20_min_power = 0.0;
    let mut best_1_hour_power = 0.0;
    let mut power_readings = Vec::<f64>::new();
    let mut max_cadence = 0.0;
    let mut avg_cadence = 0.0;
    let mut cadence_readings = Vec::<f64>::new();
    let mut max_hr = 0.0;
    let mut avg_hr = 0.0;
    let mut hr_readings = Vec::<f64>::new();

    match power_analyzer {
        None => {
        }
        Some(power_analyzer) => {
            max_power = power_analyzer.max_power;
            avg_power = power_analyzer.compute_average();
            best_5_sec_power = power_analyzer.get_best_power(power_analyzer::BEST_5_SEC_POWER);
            best_12_min_power = power_analyzer.get_best_power(power_analyzer::BEST_12_MIN_POWER);
            best_20_min_power = power_analyzer.get_best_power(power_analyzer::BEST_20_MIN_POWER);
            best_1_hour_power = power_analyzer.get_best_power(power_analyzer::BEST_1_HOUR_POWER);
            power_readings = power_analyzer.power_readings.clone();
        }
    }

    match cadence_analyzer {
        None => {
        }
        Some(cadence_analyzer) => {
            max_cadence = cadence_analyzer.max_cadence;
            avg_cadence = cadence_analyzer.compute_average();
            cadence_readings = cadence_analyzer.readings.clone();
        }
    }

    match hr_analyzer {
        None => {
        }
        Some(hr_analyzer) => {
            max_hr = hr_analyzer.max_hr;
            avg_hr = hr_analyzer.compute_average();
            hr_readings = hr_analyzer.readings.clone();
        }
    }

    let analysis_report_str = serde_json::json!({
        "Start Time (ms)": analyzer.start_time_ms,
        "End Time (ms)": analyzer.last_time_ms,
        "Elapsed Time": (analyzer.last_time_ms - analyzer.start_time_ms) / 1000,
        "Total Distance": analyzer.total_distance,
        "Total Vertical Distance": analyzer.total_vertical,
        "Average Speed": analyzer.avg_speed,
        "Best 1K": analyzer.get_best_time(location_analyzer::BEST_1K),
        "Best Mile": analyzer.get_best_time(location_analyzer::BEST_MILE),
        "Best 5K": analyzer.get_best_time(location_analyzer::BEST_5K),
        "Best 10K": analyzer.get_best_time(location_analyzer::BEST_10K),
        "Best 15K": analyzer.get_best_time(location_analyzer::BEST_15K),
        "Best Half Marathon": analyzer.get_best_time(location_analyzer::BEST_HALF_MARATHON),
        "Best Marathon": analyzer.get_best_time(location_analyzer::BEST_MARATHON),
        "Mile Splits": analyzer.mile_splits,
        "KM Splits": analyzer.km_splits,
        "Times": analyzer.speed_times,
        "Speeds": analyzer.speed_graph,
        "Altitude Readings": analyzer.altitude_graph,
        "Gradient Curve": analyzer.gradient_curve,
        "Maximum Power": max_power,
        "Average Power": avg_power,
        "5 Second Power": best_5_sec_power,
        "12 Minute Power": best_12_min_power,
        "20 Minute Power": best_20_min_power,
        "1 Hour Power": best_1_hour_power,
        "Power Readings": power_readings,
        "Maximum Cadence": max_cadence,
        "Average Cadence": avg_cadence,
        "Cadence Readings": cadence_readings,
        "Maximum Heart Rate": max_hr,
        "Average Heart Rate": avg_hr,
        "Heart Rate Readings": hr_readings
    }).to_string();

    analysis_report_str
}

#[wasm_bindgen]
pub fn analyze_gpx(s: &str) -> String {
    let mut analysis_report_str = String::new();

    let data = BufReader::new(s.as_bytes());
    let res = gpx::read(data);

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

                // Iterate through the track segments.
                for trackseg in track.segments {

                    // Iterate through the points.
                    for point in trackseg.points {
                        let time = point.time.unwrap().timestamp();
                        let lat = point.point().y();
                        let lon = point.point().x();
                        let alt = point.elevation.unwrap();

                        analyzer.append_location((time * 1000) as u64, lat, lon, alt);
                        analyzer.update_speeds();
                    }
                }
            }

            // For calculations that only make sense once all the points have been added.
            analyzer.analyze();

            // Copy items to the final report.
            analysis_report_str = make_final_report(&analyzer, None, None, None);
        }
        Err(_e) => {
            alert("Error parsing GPX file.");
        }
    }

    analysis_report_str
}

#[wasm_bindgen]
pub fn analyze_tcx(s: &str) -> String {
    let mut data = BufReader::new(s.as_bytes());
    let res = tcx::read(&mut data);
    let mut analyzer = location_analyzer::LocationAnalyzer::new();
    let mut hr_analyzer = heart_rate_analyzer::HeartRateAnalyzer::new();
    let mut cadence_analyzer = cadence_analyzer::CadenceAnalyzer::new();
    let mut power_analyzer = power_analyzer::PowerAnalyzer::new();
    let activities = res.activities.unwrap();

    for activity in activities.activities {
        for lap in activity.laps {
            for track in lap.tracks {
                for trackpoint in track.trackpoints {
                    let time = trackpoint.time.timestamp() * 1000 + trackpoint.time.timestamp_subsec_millis() as i64;
                    let position = trackpoint.position.unwrap();
                    let altitude = trackpoint.altitude_meters.unwrap();

                    analyzer.append_location(time as u64, position.latitude, position.longitude, altitude);
                    analyzer.update_speeds();

                    // Get the heart rate reading.
                    let hr = trackpoint.heart_rate;
                    match hr {
                        None => {
                        }
                        Some(hr) => {
                            hr_analyzer.append_sensor_value(time as u64, hr.value as f64);
                        }
                    }

                    // Get the cadence reading.
                    let cadence = trackpoint.cadence;
                    match cadence {
                        None => {
                        }
                        Some(cadence) => {
                            cadence_analyzer.append_sensor_value(time as u64, cadence as f64);
                        }
                    }

                    // Get the extensions.
                    let extensions = trackpoint.extensions.as_ref();
                    match extensions {
                        None => {
                        }
                        Some(extensions) => {
                            // Get the power reading.
                            let tpx = extensions.tpx.as_ref();
                            match tpx {
                                None => {
                                }
                                Some(tpx) => {
                                    let watts = tpx.watts;
                                    match watts {
                                        None => {
                                        }
                                        Some(watts) => {
                                            power_analyzer.append_sensor_value(time as u64, watts as f64);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // For calculations that only make sense once all the points have been added.
    analyzer.analyze();

    // Copy items to the final report.
    let analysis_report_str = make_final_report(&analyzer, Some(&power_analyzer), Some(&cadence_analyzer), Some(&hr_analyzer));

    analysis_report_str
}
