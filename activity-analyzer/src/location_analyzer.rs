// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use lib_math::{distance, peaks, statistics, signals};
use std::collections::HashMap;
use kmeans::*;

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

struct DistanceNode {
    date_time_ms: u64,
    total_distance: f64, // Distance traveled (in meters)
}

pub struct LocationAnalyzer {
    pub start_time_ms: u64,
    pub last_time_ms: u64,
    last_lat: f64,
    last_lon: f64,
    last_alt: f64,

    distance_buf: Vec<DistanceNode>, // Holds the distance calculations; used for the current speed calcuations. Each item is an array of the form [date_time, meters_traveled, total_distance]
    pub speed_times: Vec<u64>, // Holds the times associated with speed_graph
    pub speed_graph: Vec<f64>, // Holds the current speed calculations
    speed_blocks: Vec<f64>, // List of speed/pace blocks, i.e. statistically significant times spent at a given pace
    pub total_distance: f64, // Distance traveled (in meters)
    pub total_vertical: f64, // Total ascent (in meters)
    pub altitude_graph: Vec<f64>, // Holds all altitude readings
    pub gradient_curve: Vec<f64>, // Holds the gradient calculations
    pub gap_graph: Vec<u64>, // Holds the grade adjusted pace calculations

    pub mile_splits: Vec<f64>, // Mile split times
    pub km_splits: Vec<f64>, // Kilometer split times

    pub avg_speed: f64, // Average speed (in meters/second)
    pub current_speed: f64, // Current speed (in meters/second)
    pub speed_variance: f64,

    pub bests: HashMap<String, u64>,
    pub activity_type: String,

    pub significant_intervals: Vec<f64>,

    speed_window_size: u64,
    last_speed_buf_update_time: u64
}

impl LocationAnalyzer {
    pub fn new() -> Self {
        let analyzer = LocationAnalyzer{start_time_ms: 0, last_time_ms: 0, last_lat: 0.0, last_lon: 0.0, last_alt: 0.0, distance_buf: Vec::new(), speed_times: Vec::new(),
            speed_graph: Vec::new(), speed_blocks: Vec::new(), total_distance: 0.0, total_vertical: 0.0, altitude_graph: Vec::new(), gradient_curve: Vec::new(),
            gap_graph: Vec::new(), mile_splits: Vec::new(), km_splits: Vec::new(), avg_speed: 0.0, current_speed: 0.0, speed_variance: 0.0, bests: HashMap::new(),
            activity_type: TYPE_UNSPECIFIED_ACTIVITY_KEY.to_string(), significant_intervals: Vec::new(), speed_window_size: 1, last_speed_buf_update_time: 0};
        analyzer
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

    fn examine_interval_peak(&mut self, start_index: usize, end_index: usize) -> bool {
        // Examines a line of near-constant pace/speed.
        if start_index >= end_index {
            return false;
        }

        // How long (in seconds) was this block?
        let start_time = self.speed_times[start_index];
        let end_time = self.speed_times[end_index];
        let line_duration_seconds = end_time - start_time;

        // Don't consider anything less than ten seconds.
        if line_duration_seconds > 10 {
            let speeds = &self.speed_graph[start_index..end_index - 1];

            let mut start_distance_rec: Option<&DistanceNode> = None;
            let mut end_distance_rec: Option<&DistanceNode> = None;

            for rec in self.distance_buf.iter() {
                if rec.date_time_ms == start_time {
                    start_distance_rec = Some(&rec);
                }
                if rec.date_time_ms == end_time {
                    end_distance_rec = Some(&rec);
                    break;
                }
            }

            let mut line_length_meters = 0.0;

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
            self.speed_blocks.push(line_avg_speed);
        }

        false
    }

    pub fn analyze(&mut self) {
        // Do a speed/pace analysis.
        if self.speed_graph.len() > 1 {

            // Compute the speed/pace variation. This will tell us how consistent the pace was.
            self.speed_variance = statistics::variance_f64(&self.speed_graph, self.avg_speed);

            // Don't look for peaks unless the variance was high. Cutoff selected via experimentation.
            if self.speed_variance > 0.25 {

                // Smooth the speed graph to take out some of the GPS jitter.
                let smoothed_graph = signals::smooth(&self.speed_graph, 4);
                if smoothed_graph.len() > 1 {

                    // Find peaks in the speed graph. We're looking for intervals.
                    let peak_list = peaks::find_peaks(&smoothed_graph, 0.3);

                    // Examine the lines between the peaks. Extract pertinant data, such as avg speed/pace and set it aside.
                    // This data is used later when generating the report.
                    let mut all_intervals = Vec::new();
                    for peak in peak_list {
                        let interval = self.examine_interval_peak(peak.left_trough.x, peak.right_trough.x);
                        if interval {
                            all_intervals.push(interval);
                        }
                    }

                    // Do a k-means analysis on the computed speed/pace blocks so we can get rid of any outliers.
                    // let significant_intervals = Vec::new();
                    let num_speed_blocks = self.speed_blocks.len();
                    if num_speed_blocks > 1 {

                        // Determine the maximum value of k.
                        let mut max_k = 10;
                        if num_speed_blocks < max_k {
                            max_k = num_speed_blocks;
                        }

                        // Run k means for each possible k.
                        let mut best_k = 0;
                        let best_labels = Vec::<usize>::new();
                        let mut steepest_slope = 0.0;
                        let distortions = Vec::<f64>::new();
                        for k in 1..max_k {
                            /*let kmean = KMeans::new(self.speed_blocks.to_vec(), num_speed_blocks, 1);
                            let result = kmean.kmeans_lloyd(k, 100, KMeans::init_kmeanplusplus, &KMeansConfig::default());
                            // let distances = self.speed_blocks / result.centroids;
                            // let distances_sum = sum(np.min(distances, axis = 1));
                            // let distortion = distances_sum / num_speed_blocks;
                            // distortions.push(distortion);*/

                            // Use the elbow method to find the best value for k.
                            if distortions.len() > 1 {
                                let slope = (distortions[k-1] + distortions[k-2]) / 2.0;
                                if best_k == 0 || slope > steepest_slope {
                                    best_k = k;
                                    steepest_slope = slope;
                                }
                            }
                        }

                        // Save off the significant peaks.
                        let mut interval_index = 0;
                        for label in best_labels {
                            if label >= 1 {
                                //self.significant_intervals.append(all_intervals[interval_index]);
                            }
                            interval_index = interval_index + 1;
                        }
                    }
                }
            }
        }
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

                if self.total_distance < 1000.0 {
                    break;
                }
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
            let distance_node = DistanceNode{date_time_ms: date_time_ms, total_distance: new_distance};
            self.distance_buf.push(distance_node);
            self.total_distance = new_distance;
            self.total_vertical = self.total_vertical + (altitude - self.last_alt).abs();
            self.altitude_graph.push(altitude);
            self.update_average_speed(elapsed_seconds);

            // Update the split calculations.
            self.do_km_split_check(elapsed_seconds);
            self.do_mile_split_check(elapsed_seconds);
        }

        self.last_time_ms = date_time_ms;
        self.last_lat = latitude;
        self.last_lon = longitude;
        self.last_alt = altitude;
    }
}
