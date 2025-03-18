// Copyright (c) 2021 Michael J. Simms. All rights reserved.

extern crate gpx;
extern crate serde;
extern crate serde_json;
extern crate tcx;
extern crate fit_file;

mod utils;
mod analyzer_context;
mod cadence_analyzer;
mod exporter;
mod event;
mod geojson;
mod gpx_route_reader;
mod gpx_writer;
mod location_analyzer;
mod merge_tool;
mod power_analyzer;
mod temperature_analyzer;
mod heart_rate_analyzer;
mod swim_analyzer;
mod tcx_writer;

use wasm_bindgen::prelude::*;
use std::io::BufReader;
use std::ffi::c_void;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Holds GeoJson data so lat/lon can be converted to a political description.
static mut GEO: geojson::GeoJson = geojson::GeoJson{ world_features: None, us_features: None };


pub struct ContextList {
    pub contexts: Vec<analyzer_context::AnalyzerContext>,
}

impl ContextList {
    pub fn new() -> Self {
        let list = ContextList{ contexts: Vec::new() };
        list
    }
}

static mut CONTEXT_LIST: ContextList = ContextList {
    contexts: Vec::new()
};


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Copyright (c) 2021 Michael J. Simms. All rights reserved.");
}

#[wasm_bindgen]
pub fn set_world_data(s: &str) {
    utils::set_panic_hook();

    unsafe {
        GEO.load_world_data(s);
    }
}

#[wasm_bindgen]
pub fn set_us_data(s: &str) {
    utils::set_panic_hook();

    unsafe {
        GEO.load_us_data(s);
    }
}

fn make_final_report(context: &analyzer_context::AnalyzerContext) -> String {

    let mut start_time_ms = context.location_analyzer.start_time_ms;
    let mut last_time_ms = context.location_analyzer.last_time_ms;
    let mut total_distance = context.location_analyzer.total_distance;

    // No time data from location sources? Maybe it's a swim.
    if start_time_ms == 0 {
        start_time_ms = context.swim_analyzer.get_start_time_ms();
        last_time_ms = context.swim_analyzer.get_last_time_ms();
    }
    if total_distance < 0.01 {
        total_distance = context.swim_analyzer.get_total_distance() as f64;
    }

    let analysis_report_str = serde_json::json!({
        "Activity Type": context.location_analyzer.activity_type,
        "Start Time (ms)": start_time_ms,
        "End Time (ms)": last_time_ms,
        "Elapsed Time": (last_time_ms - start_time_ms) / 1000,
        "Total Distance": total_distance,
        "Total Vertical Distance": context.location_analyzer.total_vertical,
        "Average Speed": context.location_analyzer.avg_speed,
        "Bests": context.location_analyzer.bests,
        "Mile Splits": context.location_analyzer.mile_splits,
        "KM Splits": context.location_analyzer.km_splits,
        "Times": context.location_analyzer.times,
        "Speed Times": context.location_analyzer.speed_times,
        "Speeds": context.location_analyzer.speed_graph,
        "Altitude Readings": context.location_analyzer.altitude_graph,
        "Gradient Curve": context.location_analyzer.gradient_curve,
        "Latitude Readings": context.location_analyzer.latitude_readings,
        "Longitude Readings": context.location_analyzer.longitude_readings,
        "Intervals": context.location_analyzer.significant_intervals,
        "Maximum Power": context.power_analyzer.max_power,
        "Average Power": context.power_analyzer.avg_power,
        "5 Second Power": context.power_analyzer.get_best_power(power_analyzer::BEST_5_SEC_POWER),
        "12 Minute Power": context.power_analyzer.get_best_power(power_analyzer::BEST_12_MIN_POWER),
        "20 Minute Power": context.power_analyzer.get_best_power(power_analyzer::BEST_20_MIN_POWER),
        "1 Hour Power": context.power_analyzer.get_best_power(power_analyzer::BEST_1_HOUR_POWER),
        "Normalized Power": context.power_analyzer.np,
        "Power Readings": context.power_analyzer.readings.clone(),
        "Power Times": context.power_analyzer.time_readings.clone(),
        "Power Intervals": context.power_analyzer.significant_intervals.clone(),
        "Maximum Cadence": context.cadence_analyzer.max_cadence,
        "Average Cadence": context.cadence_analyzer.compute_average(),
        "Cadence Readings": context.cadence_analyzer.readings.clone(),
        "Cadence Times": context.cadence_analyzer.time_readings.clone(),
        "Maximum Heart Rate": context.hr_analyzer.max_hr,
        "Average Heart Rate": context.hr_analyzer.compute_average(),
        "Heart Rate Readings": context.hr_analyzer.readings.clone(),
        "Heart Rate Times": context.hr_analyzer.time_readings.clone(),
        "Temperature Readings": context.temperature_analyzer.readings.clone(),
        "Temperature Times": context.temperature_analyzer.time_readings.clone(),
        "Swim Stroke Readings": context.swim_analyzer.strokes.clone(),
        "Swim Stroke Times": context.swim_analyzer.time_readings.clone(),
        "Events": context.events.clone()
    }).to_string();

    analysis_report_str
}

fn analyze_gpx_route(s: &str) -> String {
    let mut context = analyzer_context::AnalyzerContext::new();
    let mut route_data = BufReader::new(s.as_bytes());
    let route_result = gpx_route_reader::read(&mut route_data);

    match route_result {
        Err(_e) => {
            alert("Error parsing the GPX file.");
        }
        Ok(gpx) => {
            // Iterate through the tracks.
            for track in gpx.tracks {

                // Iterate through the track segments.
                for trackseg in track.segments {
                    let mut fake_time = 0;

                    // Iterate through the points.
                    for point in trackseg.points {
                        context.location_analyzer.append_location(fake_time, point.lat, point.lon, point.ele);
                        fake_time += 1;
                    }
                }
            }

            // For calculations that only make sense once all the points have been added.
            context.location_analyzer.analyze();
        }
    }

    // Copy items to the final report.
    make_final_report(&context)
}

#[wasm_bindgen]
pub fn analyze_gpx(s: &str) -> String {
    utils::set_panic_hook();

    let analysis_report_str;
    let mut context = analyzer_context::AnalyzerContext::new();
    let data = BufReader::new(s.as_bytes());
    let res = gpx::read(data);

    match res {
        Err(_e) => {
            // The GPX parser being used sometimes fails on very simple GPX routes,
            // so try again by running it through our own simple GPX route parser before totally giving up.
            return analyze_gpx_route(s);
        }
        Ok(gpx) => {
            // Iterate through the tracks.
            for track in gpx.tracks {

                // Get the track name.
                if let Some(activity_type) = &track.type_ {
                    context.location_analyzer.set_activity_type(activity_type.to_string())
                }

                // Iterate through the track segments.
                for trackseg in track.segments {

                    // Iterate through the points.
                    for point in trackseg.points {
                        let mut time = 0;
                        match point.time {
                            Some(temp_time) => {
                                let temp: time::OffsetDateTime = temp_time.into();
                                time = temp.unix_timestamp();
                            }
                            _ => { time += 1; }, // data is from a route so just make up a time that is greater than the previous one.
                        }
                        let lat = point.point().y();
                        let lon = point.point().x();
                        let alt = point.elevation.unwrap();

                        context.location_analyzer.append_location((time * 1000) as u64, lat, lon, alt);
                        context.location_analyzer.update_speeds();
                    }
                }
            }

            // For calculations that only make sense once all the points have been added.
            context.location_analyzer.analyze();

            // Copy items to the final report.
            analysis_report_str = make_final_report(&context);
        }
    }

    // Remember this context in case we need it later.
    unsafe {
        CONTEXT_LIST.contexts.push(context);
    }

    analysis_report_str
}

#[wasm_bindgen]
pub fn analyze_tcx(s: &str) -> String {
    utils::set_panic_hook();

    let mut context = analyzer_context::AnalyzerContext::new();
    let mut data = BufReader::new(s.as_bytes());
    let mut error = false;
    let res = tcx::read(&mut data);

    match res {
        Err(_e) => {
            alert("Error parsing the TCX file.");
            error = true;
        }
        Ok(res) => {
            let activities = res.activities;
            match activities {
                None => {
                }
                Some(activities) => {
                    // A file can contain multiple activities.
                    for activity in activities.activities {
                        context.location_analyzer.set_activity_type(activity.sport);

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
                                                    context.location_analyzer.append_location(time as u64, position.latitude, position.longitude, altitude);
                                                    context.location_analyzer.update_speeds();
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
                                            context.hr_analyzer.append_sensor_value(time as u64, hr.value as f64);
                                        }
                                    }

                                    // Get the cadence reading.
                                    let cadence = trackpoint.cadence;
                                    match cadence {
                                        None => {
                                        }
                                        Some(cadence) => {
                                            context.cadence_analyzer.append_sensor_value(time as u64, cadence as f64);
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
                                                            context.power_analyzer.append_sensor_value(time as u64, watts as f64);
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
            context.location_analyzer.analyze();
            context.power_analyzer.analyze();
        }
    }

    let mut analysis_report_str = "".to_string();
    if !error {

        // Copy items to the final report.
        analysis_report_str = make_final_report(&context);

        // Remember this context in case we need it later.
        unsafe {
            CONTEXT_LIST.contexts.push(context);
        }
    }

    analysis_report_str
}

/// Called for each FIT record message as it is processed.
fn callback(timestamp: u32, global_message_num: u16, _local_msg_type: u8, _message_index: u16, fields: Vec<fit_file::fit_file::FitFieldValue>, context: *mut c_void) {
    let callback_context: &mut analyzer_context::AnalyzerContext = unsafe { &mut *(context as *mut analyzer_context::AnalyzerContext) };

    if global_message_num == fit_file::fit_file::GLOBAL_MSG_NUM_SESSION {
        let msg = fit_file::fit_file::FitSessionMsg::new(fields);

        if let Some(sport_id) = msg.sport {
            let sport_names = fit_file::fit_file::init_sport_name_map();
            callback_context.location_analyzer.set_activity_type(sport_names.get(&sport_id).unwrap().to_string());
        }

        if let Some(pool_length) = msg.pool_length {
            callback_context.swim_analyzer.set_pool_length(pool_length);
        }
    }
    else if global_message_num == fit_file::fit_file::GLOBAL_MSG_NUM_RECORD {
        let msg = fit_file::fit_file::FitRecordMsg::new(fields);
        let timestamp_ms = timestamp as u64 * 1000;

        let mut latitude = 0.0;
        let mut longitude = 0.0;
        let mut altitude = 0.0;
        let mut valid_location = false;

        if let Some(lat_semicircles) = msg.position_lat {

            // Make sure we have a valid reading.
            if lat_semicircles != 0x7FFFFFFF {
                latitude = fit_file::fit_file::semicircles_to_degrees(lat_semicircles);

                if let Some(lon_semicircles) = msg.position_long {

                    // Make sure we have a valid reading.
                    if lon_semicircles != 0x7FFFFFFF {
                        longitude = fit_file::fit_file::semicircles_to_degrees(lon_semicircles);
                        valid_location = true;
                    }
                }
            }
        }

        // Some devices don't have altitude data, so just zero it out in that case.
        if let Some(res) = msg.altitude {
    
            // Make sure we have a valid reading.
            if res != 0xFFFF {
                // Apply scaling and offset.
                altitude = (res as f64 / 5.0) - 500.0;
            }
        }

        // Prefer enhanced altitude over regular altitude.
        if let Some(res) = msg.enhanced_altitude {

            // Make sure we have a valid reading.
            if res != 0xFFFF {
                // Apply scaling and offset.
                altitude = (res as f64 / 5.0) - 500.0;
            }
        }

        if let Some(heart_rate) = msg.heart_rate {

            // Make sure we have a valid reading.
            if heart_rate < 255 {
                callback_context.hr_analyzer.append_sensor_value(timestamp_ms, heart_rate as f64);
            }
        }

        if let Some(watts) = msg.power {

            // Make sure we have a valid reading.
            if watts < 65535 {
                callback_context.power_analyzer.append_sensor_value(timestamp_ms, watts as f64);
            }
        }

        if let Some(temp) = msg.temperature {

            // Make sure we have a valid reading.
            if temp < 127 {
                callback_context.temperature_analyzer.append_sensor_value(timestamp_ms, temp as f64);
            }
        }

        if valid_location {
            callback_context.location_analyzer.append_location(timestamp_ms, latitude, longitude, altitude);
            callback_context.location_analyzer.update_speeds();
        }
    }
    else if global_message_num == fit_file::fit_file::GLOBAL_MSG_NUM_EVENT {
        let msg = fit_file::fit_file::FitEventMsg::new(fields);
        let timestamp_ms = timestamp as u64 * 1000;

        if let Some(event_num) = msg.event {
            // Front and rear gear change (42 == rear gear change, 43 == front gear change).
            if event_num == 42 || event_num == 43 {
                let event = event::Event{ timestamp_ms, event_type: event_num, event_data: 0 };
                callback_context.events.push(event);
            }
            // Radar threat alert.
            else if event_num == 75 {
            }
        }
    }
    else if global_message_num == fit_file::fit_file::GLOBAL_MSG_NUM_LENGTH {
        let msg = fit_file::fit_file::FitLengthMsg::new(fields);
        let timestamp_ms = timestamp as u64 * 1000;

        if let Some(total_strokes) = msg.total_strokes {
            if total_strokes < 65535 {
                callback_context.swim_analyzer.append_sensor_value(timestamp_ms, total_strokes);
            }
        }
    }
}

#[wasm_bindgen]
pub fn analyze_fit(s: &[u8]) -> String {
    utils::set_panic_hook();

    let mut context = analyzer_context::AnalyzerContext::new();
    let context_ptr: *mut c_void = &mut context as *mut _ as *mut c_void;

    let mut data = BufReader::new(s);
    let res = fit_file::fit_file::read(&mut data, callback, context_ptr);

    let mut error = false;

    match res {
        Err(_e) => {
            alert("Error parsing the FIT file.");
            error = true;
        }
        Ok(_res) => {
            // For calculations that only make sense once all the points have been added.
            context.location_analyzer.analyze();
            context.power_analyzer.analyze();
        }
    }

    let mut analysis_report_str = "".to_string();
    
    if !error {
        // Copy items to the final report.
        analysis_report_str = make_final_report(&context);

        // Remember this context in case we need it later.
        unsafe {
            CONTEXT_LIST.contexts.push(context);
        }
    }

    analysis_report_str
}

#[wasm_bindgen]
pub fn export_data(format: &str, split_start: u32, split_end: u32) -> String {
    utils::set_panic_hook();

    let mut exported_data = String::new();

    unsafe {
        if !CONTEXT_LIST.contexts.is_empty() {
            let exporter = exporter::Exporter::new();
            exported_data = exporter.export(CONTEXT_LIST.contexts.last().unwrap(), format, (split_start as u64) * 1000, (split_end as u64) * 1000);
        }
        else {
            alert("Nothing to export.");
        }
    }

    exported_data
}

#[wasm_bindgen]
pub fn merge(format: &str) -> String {
    utils::set_panic_hook();

    let mut merged_data = String::new();

    unsafe {
        if  CONTEXT_LIST.contexts.len() > 1 {
            let merge_tool = merge_tool::MergeTool::new();
            let exporter = exporter::Exporter::new();
            let merged_context = merge_tool.merge(&CONTEXT_LIST.contexts[0], &CONTEXT_LIST.contexts[1]);

            merged_data = exporter.export(&merged_context, format, 0, 0);
        }
        else {
            alert("Nothing to merge.");
        }
    }

    merged_data
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::fs::File;
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
            Err(_error) => {
            }
        }

        // Analyze the file and return the results.
        analyze_tcx(&content)
    }

    #[test]
    fn file1_test() {
        let local_file_name = "tests/20180810_zwift_innsbruckring_x2.tcx";
        let remote_file_name = "https://github.com/msimms/TestFilesForFitnessApps/raw/master/tcx/20180810_zwift_innsbruckring_x2.tcx";
        let result = test_file(local_file_name, remote_file_name);

        println!("{}", result);
    }
}
