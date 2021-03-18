// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use lib_math::{distance};
use std::collections::HashMap;

const METERS_PER_KM: f64 = 1000.0;
const METERS_PER_MILE: f64 = 1609.34;
const METERS_PER_HALF_MARATHON: f64 = 13.1 * METERS_PER_MILE;
const METERS_PER_MARATHON: f64 = 26.2 * METERS_PER_MILE;

const BEST_1K: &str = "Best 1K";
const BEST_MILE: &str = "Best Mile";
const BEST_5K: &str = "Best 5K";
const BEST_10K: &str = "Best 10K";
const BEST_15K: &str = "Best 15K";
const BEST_HALF_MARATHON: &str = "Best Half Marathon";
const BEST_MARATHON: &str = "Best Marathon";
const BEST_METRIC_CENTURY: &str = "Best Metric Century";
const BEST_CENTURY: &str = "Best Century";

const TYPE_UNSPECIFIED_ACTIVITY_KEY: &str = "Unknown";
const TYPE_RUNNING_KEY: &str = "Running";
const TYPE_CYCLING_KEY: &str = "Cycling";

struct DistanceNode {
    date_time: u64,
    meters_traveled: f64, // Meters traveled so far
    total_distance: f64, // Distance traveled (in meters)
}

pub struct LocationAnalyzer {
    pub start_time: u64,
    pub last_time: u64,
    last_lat: f64,
    last_lon: f64,
    last_alt: f64,

    distance_buf: Vec<DistanceNode>, // Holds the distance calculations; used for the current speed calcuations. Each item is an array of the form [date_time, meters_traveled, total_distance]
    pub speed_times: Vec<u64>, // Holds the times associated with speed_graph
    pub speed_graph: Vec<f64>, // Holds the current speed calculations 
    //speed_blocks: Vec<>, // List of speed/pace blocks, i.e. statistically significant times spent at a given pace
    pub total_distance: f64, // Distance traveled (in meters)
    pub total_vertical: f64, // Total ascent (in meters)

    pub mile_splits: Vec<f64>, // Mile split times
    pub km_splits: Vec<f64>, // Kilometer split times

    pub avg_speed: f64, // Average speed (in meters/second)
    pub current_speed: f64, // Current speed (in meters/second)

    pub bests: HashMap<String, u64>,

    pub activity_type: String,
    speed_window_size: u64
}

impl LocationAnalyzer {
    pub fn new() -> Self {
        let analyzer = LocationAnalyzer{start_time: 0, last_time: 0, last_lat: 0.0, last_lon: 0.0, last_alt: 0.0, distance_buf: Vec::new(), speed_times: Vec::new(), speed_graph: Vec::new(), total_distance: 0.0, total_vertical: 0.0, mile_splits: Vec::new(), km_splits: Vec::new(), avg_speed: 0.0, current_speed: 0.0, bests: HashMap::new(), activity_type: TYPE_UNSPECIFIED_ACTIVITY_KEY.to_string(), speed_window_size: 1};
        analyzer
    }

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

    fn update_average_speed(&mut self, elapsed_seconds: u64) {
        // Computes the average speed of the workout. Called by 'append_location'.
        if elapsed_seconds > 0 {
            self.avg_speed = self.total_distance / (elapsed_seconds as f64)
        }
    }

    fn get_best_time(&self, record_name: &str) -> u64 {
        // Returns the time associated with the specified record, or None if not found.
        match self.bests.get(record_name) {
            Some(&number) => return number,
            _ => return 0,
        }
    }

    fn do_record_check(&self, record_name: &str, seconds: u64, meters: f64, record_meters: f64) -> bool {
        // Looks up the existing record and, if necessary, updates it.
        if (meters as u64) == (record_meters as u64) {
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

    pub fn update_speeds(&mut self) {
        // Computes the average speed over the last mile. Called by 'append_location'.

        // This will be recomputed here, so zero it out.
        self.current_speed = 0.0;

        // Loop through the list, in reverse order, updating the current speed, and all "bests".
        let time_distance_iter = self.distance_buf.iter().rev();
        for time_distance_node in time_distance_iter {

            // Convert time from ms to seconds - seconds from this point to the end of the activity.
            let current_time = time_distance_node.date_time;
            let total_seconds = (self.last_time - current_time) / 1000;
            if total_seconds <= 0 {
                continue;
            }

            // Distance travelled from this point to the end of the activity.
            let current_distance = time_distance_node.total_distance;
            let total_meters = self.total_distance - current_distance;

            // Current speed is the average of the last ten seconds.
            if total_seconds == self.speed_window_size {
                self.current_speed = total_meters / total_seconds as f64;
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

    pub fn append_location(&mut self, date_time: u64, latitude: f64, longitude: f64, altitude: f64) {
        // Not much we can do with the first location other than note the start time.
        if self.start_time == 0 {
            self.start_time = date_time;
        }

        // Update the total distance calculation.
        else if self.last_time != 0 {

            // How far since the last point?
            let meters_traveled = distance::haversine_distance(latitude, longitude, altitude, self.last_lat, self.last_lon, self.last_alt);

            // How long has it been?
            let elapsed_seconds = date_time - self.start_time;

            // Update totals and averages.
            self.total_distance = self.total_distance + meters_traveled;
            let distance_node = DistanceNode{date_time: date_time, meters_traveled: meters_traveled, total_distance: self.total_distance};
            self.distance_buf.push(distance_node);
            self.total_vertical = self.total_vertical + (altitude - self.last_alt).abs();
            self.update_average_speed(elapsed_seconds);

            // Update the split calculations.
            self.do_km_split_check(elapsed_seconds);
            self.do_mile_split_check(elapsed_seconds);
        }

        // Update the heat map.
        //self.location_heat_map.append(latitude, longitude);

        self.last_time = date_time;
        self.last_lat = latitude;
        self.last_lon = longitude;
        self.last_alt = altitude;
    }
}
