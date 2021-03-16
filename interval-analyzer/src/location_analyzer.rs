// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use lib_math::{distance};

const METERS_PER_KM: f64 = 1000.0;
const METERS_PER_MILE: f64 = 1609.34;

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
    speed_times: Vec<u64>, // Holds the times associated with speed_graph
    speed_graph: Vec<f64>, // Holds the current speed calculations 
    //speed_blocks: Vec<>, // List of speed/pace blocks, i.e. statistically significant times spent at a given pace
    pub total_distance: f64, // Distance traveled (in meters)
    pub total_vertical: f64, // Total ascent (in meters)

    mile_splits: Vec<f64>, // Mile split times
    km_splits: Vec<f64>, // Kilometer split times

    pub avg_speed: f64, // Average speed (in meters/second)
    pub current_speed: f64 // Current speed (in meters/second)
}

impl LocationAnalyzer {
    pub fn new() -> Self {
        let analyzer = LocationAnalyzer{start_time: 0, last_time: 0, last_lat: 0.0, last_lon: 0.0, last_alt: 0.0, distance_buf: Vec::new(), speed_times: Vec::new(), speed_graph: Vec::new(), total_distance: 0.0, total_vertical: 0.0, mile_splits: Vec::new(), km_splits: Vec::new(), avg_speed: 0.0, current_speed: 0.0};
        analyzer
    }

    fn update_average_speed(&mut self, date_time: u64) {
        // Computes the average speed of the workout. Called by 'append_location'.
        let elapsed_milliseconds = date_time - self.start_time;
        if elapsed_milliseconds > 0 {
            self.avg_speed = self.total_distance / (elapsed_milliseconds as f64 / 1000.0)
        }
    }

    fn get_best_time(&self, record_name: &str) -> u64 {
        // Returns the time associated with the specified record, or None if not found.
        /*if record_name in self.bests {
            return self.bests[record_name]
        }*/
        0
    }

    fn do_record_check(&mut self, record_name: &str, seconds: u64, meters: f64, record_meters: f64) {
        // Looks up the existing record and, if necessary, updates it.
        if (meters as u64) == (record_meters as u64) {
            let old_value = self.get_best_time(record_name);
            if old_value == 0 || seconds < old_value {
//                self.bests[record_name] = seconds
            }
        }
    }

    fn do_split_check(&self, seconds: u64, split_meters: f64, split_buf: &Vec<f64>) {
        let units_traveled = self.total_distance / split_meters;
        let whole_units_traveled = units_traveled as u64;
/*        if len(split_buf) < whole_units_traveled + 1 {
            split_buf.append(seconds)
        }
        else {
            split_buf[whole_units_traveled] = seconds
        } */
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

            // Update totals and averages.
            self.total_distance = self.total_distance + meters_traveled;
            let distance_node = DistanceNode{date_time: date_time, meters_traveled: meters_traveled, total_distance: self.total_distance};
            self.distance_buf.push(distance_node);
            self.total_vertical = self.total_vertical + (altitude - self.last_alt).abs();
            self.update_average_speed(date_time);

            // Update the split calculations.
            self.do_split_check(date_time - self.start_time, METERS_PER_KM, &self.km_splits);
            self.do_split_check(date_time - self.start_time, METERS_PER_MILE, &self.mile_splits);
        }

        // Update the heat map.
        //self.location_heat_map.append(latitude, longitude);

        self.last_time = date_time;
        self.last_lat = latitude;
        self.last_lon = longitude;
        self.last_alt = altitude;
    }
}
