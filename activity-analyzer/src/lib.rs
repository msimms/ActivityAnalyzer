// Copyright (c) 2021 Michael J. Simms. All rights reserved.

extern crate gpx;
extern crate serde;
extern crate serde_json;
extern crate tcx;
extern crate fit_file;

mod utils;
mod cadence_analyzer;
mod geo_json_reader;
mod location_analyzer;
mod power_analyzer;
mod heart_rate_analyzer;

use wasm_bindgen::prelude::*;
use std::io::BufReader;
use std::ffi::c_void;

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

fn make_final_report(location_analyzer: &location_analyzer::LocationAnalyzer, power_analyzer: Option<&power_analyzer::PowerAnalyzer>, cadence_analyzer: Option<&cadence_analyzer::CadenceAnalyzer>, hr_analyzer: Option<&heart_rate_analyzer::HeartRateAnalyzer>) -> String {
    let mut max_power = 0.0;
    let mut avg_power = 0.0;
    let mut best_5_sec_power = 0.0;
    let mut best_12_min_power = 0.0;
    let mut best_20_min_power = 0.0;
    let mut best_1_hour_power = 0.0;
    let mut normalized_power = 0.0;
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
            normalized_power = power_analyzer.np;
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
        "Activity Type": location_analyzer.activity_type,
        "Start Time (ms)": location_analyzer.start_time_ms,
        "End Time (ms)": location_analyzer.last_time_ms,
        "Elapsed Time": (location_analyzer.last_time_ms - location_analyzer.start_time_ms) / 1000,
        "Total Distance": location_analyzer.total_distance,
        "Total Vertical Distance": location_analyzer.total_vertical,
        "Average Speed": location_analyzer.avg_speed,
        "Best 1K": location_analyzer.get_best_time(location_analyzer::BEST_1K),
        "Best Mile": location_analyzer.get_best_time(location_analyzer::BEST_MILE),
        "Best 5K": location_analyzer.get_best_time(location_analyzer::BEST_5K),
        "Best 10K": location_analyzer.get_best_time(location_analyzer::BEST_10K),
        "Best 15K": location_analyzer.get_best_time(location_analyzer::BEST_15K),
        "Best Half Marathon": location_analyzer.get_best_time(location_analyzer::BEST_HALF_MARATHON),
        "Best Marathon": location_analyzer.get_best_time(location_analyzer::BEST_MARATHON),
        "Mile Splits": location_analyzer.mile_splits,
        "KM Splits": location_analyzer.km_splits,
        "Times": location_analyzer.times,
        "Speed Times": location_analyzer.speed_times,
        "Speeds": location_analyzer.speed_graph,
        "Altitude Readings": location_analyzer.altitude_graph,
        "Gradient Curve": location_analyzer.gradient_curve,
        "Latitude Readings": location_analyzer.latitude_readings,
        "Longitude Readings": location_analyzer.longitude_readings,
        "Intervals": location_analyzer.significant_intervals,
        "Maximum Power": max_power,
        "Average Power": avg_power,
        "5 Second Power": best_5_sec_power,
        "12 Minute Power": best_12_min_power,
        "20 Minute Power": best_20_min_power,
        "1 Hour Power": best_1_hour_power,
        "Normalized Power": normalized_power,
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
    utils::set_panic_hook();

    let mut analysis_report_str = String::new();

    let data = BufReader::new(s.as_bytes());
    let res = gpx::read(data);

    match res {
        Err(_e) => {
            alert("Error parsing the GPX file.");
        }
        Ok(gpx) => {
            let mut location_analyzer = location_analyzer::LocationAnalyzer::new();

            // Iterate through the tracks.
            for track in gpx.tracks {

                // Get the track name.
                match &track._type {
                    Some(activity_type) => location_analyzer.set_activity_type(activity_type.to_string()),
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

                        location_analyzer.append_location((time * 1000) as u64, lat, lon, alt);
                        location_analyzer.update_speeds();
                    }
                }
            }

            // For calculations that only make sense once all the points have been added.
            location_analyzer.analyze();

            // Copy items to the final report.
            analysis_report_str = make_final_report(&location_analyzer, None, None, None);
        }
    }

    analysis_report_str
}

#[wasm_bindgen]
pub fn analyze_tcx(s: &str) -> String {
    utils::set_panic_hook();

    let mut data = BufReader::new(s.as_bytes());
    let res = tcx::read(&mut data);

    let mut location_analyzer = location_analyzer::LocationAnalyzer::new();
    let mut hr_analyzer = heart_rate_analyzer::HeartRateAnalyzer::new();
    let mut cadence_analyzer = cadence_analyzer::CadenceAnalyzer::new();
    let mut power_analyzer = power_analyzer::PowerAnalyzer::new();

    match res {
        Err(_e) => {
            alert("Error parsing the TCX file.");
        }
        Ok(res) => {
            let activities = res.activities;
            match activities {
                None => {
                }
                Some(activities) => {
                    // A file can contain multiple activities.
                    for activity in activities.activities {
                        location_analyzer.set_activity_type(activity.sport);

                        // Iterate through the laps.
                        for lap in activity.laps {

                            // Iterate through the tracks.
                            for track in lap.tracks {

                                // Iterate through each point.
                                for trackpoint in track.trackpoints {
                                    let time = trackpoint.time.timestamp() * 1000 + trackpoint.time.timestamp_subsec_millis() as i64;

                                    // Get the position, including altitude.
                                    let position = trackpoint.position;
                                    match position {
                                        None => {
                                        }
                                        Some(position) => {
                                            let altitude = trackpoint.altitude_meters;
                                            match altitude {
                                                None => {
                                                }
                                                Some(altitude) => {
                                                    location_analyzer.append_location(time as u64, position.latitude, position.longitude, altitude);
                                                    location_analyzer.update_speeds();
                                                }
                                            }
                                        }
                                    }

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
                }
            }

            // For calculations that only make sense once all the points have been added.
            location_analyzer.analyze();
            power_analyzer.analyze();
        }
    }

    // Copy items to the final report.
    let analysis_report_str = make_final_report(&location_analyzer, Some(&power_analyzer), Some(&cadence_analyzer), Some(&hr_analyzer));

    analysis_report_str
}

/// Context structure. An instance of this will be passed to the parser and ultimately to the callback function so we can use it for whatever.
struct AnalyzerContext {
    pub location_analyzer: location_analyzer::LocationAnalyzer,
    pub hr_analyzer: heart_rate_analyzer::HeartRateAnalyzer,
    pub cadence_analyzer: cadence_analyzer::CadenceAnalyzer,
    pub power_analyzer: power_analyzer::PowerAnalyzer,
}

impl AnalyzerContext {
    pub fn new() -> Self {
        let context = AnalyzerContext{
            location_analyzer: location_analyzer::LocationAnalyzer::new(),
            hr_analyzer: heart_rate_analyzer::HeartRateAnalyzer::new(),
            cadence_analyzer: cadence_analyzer::CadenceAnalyzer::new(),
            power_analyzer: power_analyzer::PowerAnalyzer::new() };
        context
    }
}

/// Called for each FIT record message as it is processed.
fn callback(timestamp: u32, global_message_num: u16, _local_msg_type: u8, _message_index: u16, fields: Vec<fit_file::fit_file::FitFieldValue>, context: *mut c_void) {
    let callback_context: &mut AnalyzerContext = unsafe { &mut *(context as *mut AnalyzerContext) };

    if global_message_num == fit_file::fit_file::GLOBAL_MSG_NUM_SESSION {
        let msg = fit_file::fit_file::FitSessionMsg::new(fields);
        let sport_names = fit_file::fit_file::init_sport_name_map();
        let sport_id = msg.sport.unwrap();

        callback_context.location_analyzer.set_activity_type(sport_names.get(&sport_id).unwrap().to_string());
    }
    else if global_message_num == fit_file::fit_file::GLOBAL_MSG_NUM_RECORD {
        let msg = fit_file::fit_file::FitRecordMsg::new(fields);
        let timestamp_ms = timestamp as u64 * 1000;
        let mut latitude = 0.0;
        let mut longitude = 0.0;
        let mut altitude = 0.0;
        let mut valid = true;

        match msg.position_lat {
            Some(res) => {
                latitude = fit_file::fit_file::semicircles_to_degrees(res);
            }
            None => {
                valid = false;
            }
        }
        match msg.position_long {
            Some(res) => {
                longitude = fit_file::fit_file::semicircles_to_degrees(res);
            }
            None => {
                valid = false;
            }
        }

        // Some devices don't have altitude data, so just zero it out in that case.
        match msg.altitude {
            Some(res) => {
                altitude = res as f64;
            }
            None => {
            }
        }

        if valid {
            callback_context.location_analyzer.append_location(timestamp_ms, latitude, longitude, altitude);
            callback_context.location_analyzer.update_speeds();
        }
    }
}

#[wasm_bindgen]
pub fn analyze_fit(s: &[u8]) -> String {
    utils::set_panic_hook();

    let mut context = AnalyzerContext::new();
    let context_ptr: *mut c_void = &mut context as *mut _ as *mut c_void;

    let mut data = BufReader::new(s);
    let res = fit_file::fit_file::read(&mut data, callback, context_ptr);

    match res {
        Err(_e) => {
            alert("Error parsing the FIT file.");
        }
        Ok(_res) => {
            // For calculations that only make sense once all the points have been added.
            context.location_analyzer.analyze();
            context.power_analyzer.analyze();
        }
    }

    // Copy items to the final report.
    let analysis_report_str = make_final_report(&context.location_analyzer, Some(&context.power_analyzer), Some(&context.cadence_analyzer), Some(&context.hr_analyzer));

    analysis_report_str
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::fs::File;
    use reqwest;
    use crate::analyze_tcx;

    /// Downloads a remote file to the local file path.
    fn download_test_file(local_file_name: &str, remote_file_name: &str) {
        let resp = reqwest::blocking::get(remote_file_name).unwrap().text().unwrap();
        std::fs::write(local_file_name, resp).expect("Unable to write file.");
    }

    /// Tests a local file, downloads if it does not already exist.
    fn test_file(local_file_name: &str, remote_file_name: &str) -> String {
        if !std::path::Path::new(local_file_name).exists() {
            download_test_file(local_file_name, remote_file_name);
        }

        // Read the file into a string.
        let mut content = String::new();
        match File::open(local_file_name) {
            Ok(mut file) => {
                file.read_to_string(&mut content).unwrap();
            },
            Err(error) => {
            }
        }

        // Analyze the file and return the results.
        let result = analyze_tcx(&content);
        result
    }

    #[test]
    fn file1_test() {
        let local_file_name = "tests/20180810_zwift_innsbruckring_x2.tcx";
        let remote_file_name = "https://github.com/msimms/TestFilesForFitnessApps/raw/master/tcx/20180810_zwift_innsbruckring_x2.tcx";
        let result = test_file(local_file_name, remote_file_name);

        println!("{}", result);
    }
}
