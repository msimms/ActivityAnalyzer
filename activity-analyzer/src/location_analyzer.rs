// Copyright (c) 2021 Michael J. Simms. All rights reserved.
 #![allow(dead_code)]

use lib_math::{distance, kmeans, peaks, statistics, signals};
use std::collections::HashMap;
use serde::Serialize;

const METERS_PER_KM: f64 = 1000.0;
const METERS_PER_MILE: f64 = 1609.34;
const METERS_PER_HALF_MARATHON: f64 = 13.1 * METERS_PER_MILE;
const METERS_PER_MARATHON: f64 = 26.2 * METERS_PER_MILE;

pub const BEST_1K: &str = "Best 1K";
pub const BEST_MILE: &str = "Best Mile";
pub const BEST_5K: &str = "Best 5K";
pub const BEST_10K: &str = "Best 10K";
pub const BEST_15K: &str = "Best 15K";
pub const BEST_HALF_MARATHON: &str = "Best Half Marathon";
pub const BEST_MARATHON: &str = "Best Marathon";
pub const BEST_METRIC_CENTURY: &str = "Best Metric Century";
pub const BEST_CENTURY: &str = "Best Century";

const TYPE_UNSPECIFIED_ACTIVITY_KEY: &str = "Unknown";
const TYPE_RUNNING_KEY: &str = "Running";
const TYPE_CYCLING_KEY: &str = "Cycling";

#[derive(Clone, Copy, Serialize)]
pub struct IntervalDescription {
    pub start_time: u64,
    pub end_time: u64,
    pub line_length_meters: f64,
    pub line_avg_speed: f64
}

impl IntervalDescription {
    pub fn new() -> Self {
        let interval = IntervalDescription{start_time: 0, end_time: 0, line_length_meters: 0.0, line_avg_speed: 0.0};
        interval
    }
}

struct DistanceNode {
    date_time_ms: u64,
    total_distance: f64, // Distance traveled (in meters)
}

pub struct LocationAnalyzer {
    pub start_time_ms: u64, // First timestamp
    pub last_time_ms: u64, // Most recent timestamp
    last_lat: f64, // Most recent latitude reading
    last_lon: f64, // Most recent longitude reading
    last_alt: f64, // Most recent altitude reading

    distance_buf: Vec<DistanceNode>, // Holds the distance calculations; used for the current speed calcuations. Each item is an array of the form [date_time, meters_traveled, total_distance]
    pub speed_times: Vec<u64>, // Holds the times associated with speed_graph, since the speed graph involves averaging, it might be missing some time values from the beginning of the event
    pub speed_graph: Vec<f64>, // Holds the current speed calculations
    pub total_distance: f64, // Distance traveled (in meters)
    pub total_vertical: f64, // Total ascent (in meters)

    pub times: Vec<u64>, // Holds all timestamps (in milliseconds). Can be used to graph everything except speed/pace data.
    pub lap_times: Vec<u64>, // Holds all timestamps (in milliseconds) at which the lap button was pressed.
    pub latitude_readings: Vec<f64>,
    pub longitude_readings: Vec<f64>,
    pub altitude_graph: Vec<f64>, // Holds all altitude readings
    pub gradient_curve: Vec<f64>, // Holds the gradient calculations
    pub gap_graph: Vec<u64>, // Holds the grade adjusted pace calculations

    pub mile_splits: Vec<f64>, // Mile split times
    pub km_splits: Vec<f64>, // Kilometer split times

    pub avg_speed: f64, // Average speed (in meters/second)
    pub current_speed: f64, // Current speed (in meters/second)
    pub speed_variance: f64,

    pub bests: HashMap<String, u64>,
    pub max_altitude: f64,

    pub activity_type: String,

    pub significant_intervals: Vec<IntervalDescription>,
    pub geo_analyzer: super::geo_json_reader::GeoJsonReader,

    speed_window_size: u64,
    last_speed_buf_update_time: u64,
}

impl LocationAnalyzer {
    pub fn new() -> Self {
        let analyzer = LocationAnalyzer{start_time_ms: 0, last_time_ms: 0, last_lat: 0.0, last_lon: 0.0, last_alt: 0.0, distance_buf: Vec::new(), speed_times: Vec::new(),
            speed_graph: Vec::new(), total_distance: 0.0, total_vertical: 0.0, times: Vec::new(), lap_times: Vec::new(), latitude_readings: Vec::new(), longitude_readings: Vec::new(),
            altitude_graph: Vec::new(), gradient_curve: Vec::new(), gap_graph: Vec::new(), mile_splits: Vec::new(), km_splits: Vec::new(), avg_speed: 0.0, current_speed: 0.0,
            speed_variance: 0.0, bests: HashMap::new(), max_altitude: 0.0, activity_type: TYPE_UNSPECIFIED_ACTIVITY_KEY.to_string(), significant_intervals: Vec::new(),
            geo_analyzer: super::geo_json_reader::GeoJsonReader::new(), speed_window_size: 1, last_speed_buf_update_time: 0};
        analyzer
    }

    /// Accessor methods for lap metadata.
    pub fn get_lap_start_time(&self, lap_num: u8) -> u64 {
        if lap_num == 1 {
            return self.start_time_ms;
        }
        0
    }
    pub fn get_lap_seconds(&self, lap_num: u8) -> u64 {
        if self.lap_times.len() == 0 {
        }
        0
    }
    pub fn get_lap_calories(&self, lap_num: u8) -> f64 {
        if self.lap_times.len() == 0 {
        }
        0.0
    }
    pub fn get_lap_distance(&self, lap_num: u8) -> f64 {
        if self.lap_times.len() == 0 {
        }
        0.0
    }

    /// Accessor for setting the activity type.
    pub fn set_activity_type(&mut self, activity_type: String) {
        self.activity_type = activity_type;

        // This refers to the number of seconds used when averaging samples together to
        // compute the current speed. The exact numbers were chosen based on experimentation.
        if self.activity_type == TYPE_CYCLING_KEY {
            self.speed_window_size = 7;
        }
        else {
            self.speed_window_size = 11;
        }
    }

    /// Computes the average speed of the workout. Called by 'append_location'.
    fn update_average_speed(&mut self, elapsed_seconds: u64) {
        if elapsed_seconds > 0 {
            self.avg_speed = self.total_distance / (elapsed_seconds as f64)
        }
    }

    /// Returns the time associated with the specified record, or None if not found.
    pub fn get_best_time(&self, record_name: &str) -> u64 {
        match self.bests.get(record_name) {
            Some(&number) => return number,
            _ => return 0,
        }
    }

    /// Looks up the existing record and returns true if it needs updating.
    fn do_record_check(&self, record_name: &str, seconds: u64, meters: f64, record_meters: f64) -> bool {
        let int_meters = meters as u64;
        let int_record_meters = record_meters as u64;

        if int_meters == int_record_meters {
            let old_value = self.get_best_time(record_name);

            if old_value == 0 || seconds < old_value {
                return true;
            }
        }

        false
    }

    fn do_km_split_check(&mut self, seconds: u64) {
        let units_traveled = self.total_distance / METERS_PER_KM;
        let whole_units_traveled = units_traveled as usize;

        if self.km_splits.len() < whole_units_traveled + 1 {
            self.km_splits.push(seconds as f64);
        }
        else {
            self.km_splits[whole_units_traveled] = seconds as f64;
        }
    }

    fn do_mile_split_check(&mut self, seconds: u64) {
        let units_traveled = self.total_distance / METERS_PER_MILE;
        let whole_units_traveled = units_traveled as usize;

        if self.mile_splits.len() < whole_units_traveled + 1 {
            self.mile_splits.push(seconds as f64);
        }
        else {
            self.mile_splits[whole_units_traveled] = seconds as f64;
        }
    }

    fn compute_grade_adjusted_pace(gradient: f64, pace: f64) -> f64 {
        let cost = (155.4 * (f64::powf(gradient, 5.0))) - (30.4 * f64::powf(gradient, 4.0)) - (43.4 * f64::powf(gradient, 3.0)) - (46.3 * (gradient * gradient)) - (19.5 * gradient) + 3.6;
        let gap = pace + (cost - 3.6) / 3.6;
        gap
    }

    fn examine_interval_peak(&mut self, start_index: usize, end_index: usize) -> Option<IntervalDescription> {
        // Examines a line of near-constant pace/speed.
        if start_index >= end_index {
            let result: Option::<IntervalDescription> = None;
            return result;
        }

        // How long (in seconds) was this block?
        let start_time = self.speed_times[start_index];
        let end_time = self.speed_times[end_index];

        // Don't consider anything less than ten seconds.
        if end_time - start_time < 10 {
            let result: Option::<IntervalDescription> = None;
            return result;
        }

        let speeds = &self.speed_graph[start_index..end_index - 1];

        let mut start_distance_rec: Option<&DistanceNode> = None;
        let mut end_distance_rec: Option<&DistanceNode> = None;

        let mut line_length_meters = 0.0;

        for rec in self.distance_buf.iter() {
            if rec.date_time_ms == start_time {
                start_distance_rec = Some(&rec);
            }
            if rec.date_time_ms == end_time {
                end_distance_rec = Some(&rec);
                break;
            }
        }

        match start_distance_rec {
            Some(start_rec) => {
                match end_distance_rec {
                    Some(end_rec) => {
                        line_length_meters = end_rec.total_distance - start_rec.total_distance;
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        let line_avg_speed = statistics::average_f64(&speeds.to_vec());
        let desc = IntervalDescription{start_time: start_time, end_time: end_time, line_length_meters: line_length_meters, line_avg_speed: line_avg_speed};
        let result: Option::<IntervalDescription> = Some(desc);

        result
    }

    /// Performs a k-means analysis on peaks extracted from the speed/pace data to look for intervals.
    fn search_for_intervals(&mut self) {

        if self.speed_graph.len() > 1 {

            // Compute the speed/pace variation. This will tell us how consistent the pace was.
            self.speed_variance = statistics::variance_f64(&self.speed_graph, self.avg_speed);

            // Don't look for peaks unless the variance was high. Cutoff selected via experimentation.
            // Also, don't search if the feature has been disabled.
            if self.speed_variance > 0.25 {

                // Smooth the speed graph to take out some of the GPS jitter.
                let smoothed_graph = signals::smooth(&self.speed_graph, 4);
                if smoothed_graph.len() > 1 {

                    // Find peaks in the speed graph. We're looking for intervals.
                    let peak_list = peaks::find_peaks(&smoothed_graph, 0.3);

                    // Examine the lines between the peaks. Extract pertinant data, such as avg speed/pace and set it aside.
                    // This data is used later when generating the report.
                    let mut filtered_interval_list = Vec::new();
                    for peak in peak_list {
                        let interval = self.examine_interval_peak(peak.left_trough.x, peak.right_trough.x);
                        match interval {
                            Some(interval) => {
                                filtered_interval_list.push(interval);
                            }
                            _ => {},
                        }
                    }

                    // Do a k-means analysis on a 2D the computed speed/pace blocks so we can get rid of any outliers.
                    let num_possible_intervals = filtered_interval_list.len();
                    if num_possible_intervals >= 2 {

                        // Convert the interval description into something k-means can work with.
                        let sample_dimensions = 2;
                        let mut samples = vec![0.0 as f64; sample_dimensions * num_possible_intervals];
                        let mut sample_index = 0;
                        for interval in &filtered_interval_list {
                            samples[sample_index] = interval.line_avg_speed;
                            sample_index = sample_index + 1;
                            samples[sample_index] = interval.line_length_meters;
                            sample_index = sample_index + 1;
                        }

                        // Determine the maximum value of k.
                        let mut max_k = 10;
                        if num_possible_intervals < max_k {
                            max_k = num_possible_intervals;
                        }

                        // Run k means for each possible k.
                        let mut best_k = 0;
                        let mut best_labels = Vec::<usize>::new();
                        let mut steepest_slope = 0.0;
                        let mut distortions = Vec::<f64>::new();
                        let max_error = 1.0;
                        let max_iter = 100;
                        for k in 1..max_k {
                            let (labels, distortion) = kmeans::kmeans_equally_space_centroids(samples.to_vec(), sample_dimensions, k, max_error, max_iter);

                            // Store the error.
                            distortions.push(distortion);

                            // Use the elbow method to find the best value for k.
                            if distortions.len() >= 2 && k >= 2 {
                                let slope = (distortions[k-1] + distortions[k-2]) / 2.0;

                                if best_k == 0 || slope > steepest_slope {
                                    best_k = k;
                                    best_labels = labels;
                                    steepest_slope = slope;
                                }
                            }
                        }

                        // Save off the significant peaks.
                        let mut interval_index = 0;
                        for label in best_labels {
                            if label >= 1 {
                                let interval = filtered_interval_list[interval_index];
                                self.significant_intervals.push(interval);
                            }
                            interval_index = interval_index + 1;
                        }
                    }
                }
            }
        }
    }

    /// Called after all data is loaded.
    pub fn analyze(&mut self) {
        self.search_for_intervals();
    }

    /// Computes the average speed over the last mile. Called by 'append_location'.
    pub fn update_speeds(&mut self) {

        // This will be recomputed here, so zero it out.
        self.current_speed = 0.0;

        // Loop through the list, in reverse order, updating the current speed, and all "bests".
        let time_distance_iter = self.distance_buf.iter().rev();
        for time_distance_node in time_distance_iter {

            // Convert time from ms to seconds - seconds from this point to the end of the activity.
            let current_time_ms = time_distance_node.date_time_ms;
            let total_seconds = (self.last_time_ms - current_time_ms) / 1000;
            if total_seconds <= 0 {
                continue;
            }

            // Distance travelled from this point to the end of the activity.
            let current_distance = time_distance_node.total_distance;
            let total_meters = self.total_distance - current_distance;

            // Current speed is the average of the last ten seconds.
            if total_seconds == self.speed_window_size {
                self.current_speed = total_meters / total_seconds as f64;

                if current_time_ms > self.last_speed_buf_update_time {
                    self.speed_times.push(current_time_ms);
                    self.speed_graph.push(self.current_speed);
                    self.last_speed_buf_update_time = current_time_ms;
                }
            }

            // Is this a new kilometer record for this activity?
            if total_meters < 1000.0 {
                continue;
            }
            if self.do_record_check(BEST_1K, total_seconds, total_meters, 1000.0) {
                self.bests.entry(BEST_1K.to_string()).or_insert(total_seconds);
            }

            // Is this a new mile record for this activity?
            if total_meters < METERS_PER_MILE {
                continue;
            }
            if self.do_record_check(BEST_MILE, total_seconds, total_meters, METERS_PER_MILE) {
                self.bests.entry(BEST_MILE.to_string()).or_insert(total_seconds);
            }

            // Is this a new 5K record for this activity?
            if total_meters < 5000.0 {
                continue;
            }
            if self.do_record_check(BEST_5K, total_seconds, total_meters, 5000.0) {
                self.bests.entry(BEST_5K.to_string()).or_insert(total_seconds);
            }

            // Is this a new 10K record for this activity?
            if total_meters < 10000.0 {
                continue;
            }
            if self.do_record_check(BEST_10K, total_seconds, total_meters, 10000.0) {
                self.bests.entry(BEST_10K.to_string()).or_insert(total_seconds);
            }

            // Running-specific records:
            if self.activity_type == TYPE_RUNNING_KEY {

                // Is this a new 15K record for this activity?
                if total_meters < 15000.0 {
                    continue;
                }
                if self.do_record_check(BEST_15K, total_seconds, total_meters, 15000.0) {
                    self.bests.entry(BEST_15K.to_string()).or_insert(total_seconds);
                }

                // Is this a new half marathon record for this activity?
                if total_meters < METERS_PER_HALF_MARATHON {
                    continue;
                }
                if self.do_record_check(BEST_HALF_MARATHON, total_seconds, total_meters, METERS_PER_HALF_MARATHON) {
                    self.bests.entry(BEST_HALF_MARATHON.to_string()).or_insert(total_seconds);
                }

                // Is this a new marathon record for this activity?
                if total_meters < METERS_PER_MARATHON {
                    continue;
                }
                if self.do_record_check(BEST_MARATHON, total_seconds, total_meters, METERS_PER_MARATHON) {
                    self.bests.entry(BEST_MARATHON.to_string()).or_insert(total_seconds);
                }
            }

            // Cycling-specific records:
            if self.activity_type == TYPE_CYCLING_KEY {

                // Is this a new metric century record for this activity?
                if total_meters < 100000.0 {
                    continue;
                }
                if self.do_record_check(BEST_METRIC_CENTURY, total_seconds, total_meters, 100000.0) {
                    self.bests.entry(BEST_METRIC_CENTURY.to_string()).or_insert(total_seconds);
                }

                // Is this a new century record for this activity?
                if total_meters < METERS_PER_MILE * 100.0 {
                    continue;
                }
                if self.do_record_check(BEST_CENTURY, total_seconds, total_meters, METERS_PER_MILE * 100.0) {
                    self.bests.entry(BEST_CENTURY.to_string()).or_insert(total_seconds);
                }
            }
        }
    }

    pub fn append_location(&mut self, date_time_ms: u64, latitude: f64, longitude: f64, altitude: f64) {

        // Not much we can do with the first location other than note the start time.
        if self.start_time_ms == 0 {
            self.start_time_ms = date_time_ms;
        }

        // Update the total distance calculation.
        else if self.last_time_ms != 0 {

            // How far since the last point?
            let meters_traveled = distance::haversine_distance(latitude, longitude, altitude, self.last_lat, self.last_lon, self.last_alt);

            // How long has it been?
            let elapsed_seconds = (date_time_ms - self.start_time_ms) / 1000;

            // Compute the grade adjusted pace.
            let num_alts = self.altitude_graph.len();
            if num_alts > 0 {
                let prev_alt = self.altitude_graph[num_alts - 1];
                let gradient = (altitude - prev_alt) / meters_traveled;

                self.gradient_curve.push(gradient);
            }

            // Update totals and averages.
            let new_distance = self.total_distance + meters_traveled;
            let distance_node = DistanceNode{ date_time_ms: date_time_ms, total_distance: new_distance };
            self.distance_buf.push(distance_node);
            self.total_distance = new_distance;
            let vertical = altitude - self.last_alt;
            if vertical > 0.0 {
                self.total_vertical = self.total_vertical + vertical;
            }
            self.times.push(date_time_ms);
            self.latitude_readings.push(latitude);
            self.longitude_readings.push(longitude);
            self.altitude_graph.push(altitude);
            self.update_average_speed(elapsed_seconds);

            // Update the split calculations.
            self.do_km_split_check(elapsed_seconds);
            self.do_mile_split_check(elapsed_seconds);

            // Update max altitude.
            if altitude > self.max_altitude {
                self.max_altitude = altitude;
            }
        }

        self.last_time_ms = date_time_ms;
        self.last_lat = latitude;
        self.last_lon = longitude;
        self.last_alt = altitude;
    }
}
