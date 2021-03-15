// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use lib_math::{distance};

pub struct LocationAnalyzer {
    start_time: u64,
    last_time: u64,
    last_lat: f64,
    last_lon: f64,
    last_alt: f64,

    distance_buf: Vec<f64>, // Holds the distance calculations; used for the current speed calcuations. Each item is an array of the form [date_time, meters_traveled, total_distance]
    speed_times: Vec<u64>, // Holds the times associated with speed_graph
    speed_graph: Vec<f64>, // Holds the current speed calculations 
    //speed_blocks: Vec<>, // List of speed/pace blocks, i.e. statistically significant times spent at a given pace
    total_distance: f64, // Distance traveled (in meters)
    total_vertical: f64, // Total ascent (in meters)

    mile_splits: Vec<f64>, // Mile split times
    km_splits: Vec<f64>, // Kilometer split times

    avg_speed: f64, // Average speed (in meters/second)
    current_speed: f64 // Current speed (in meters/second)
}

impl LocationAnalyzer {
    pub fn new() -> Self {
        let analyzer = LocationAnalyzer{start_time: 0, last_time: 0, last_lat: 0.0, last_lon: 0.0, last_alt: 0.0, distance_buf: Vec::new(), speed_times: Vec::new(), speed_graph: Vec::new(), total_distance: 0.0, total_vertical: 0.0, mile_splits: Vec::new(), km_splits: Vec::new(), avg_speed: 0.0, current_speed: 0.0};
        analyzer
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
            //self.distance_buf.push!([date_time, meters_traveled, self.total_distance]);
            //self.total_vertical = self.total_vertical + (altitude - self.last_alt).abs();
            //update_average_speed(self, date_time);

            // Update the split calculations.
            //do_split_check(self, date_time - self.start_time, 1000, self.km_splits);
            //do_split_check(self, date_time - self.start_time, Units.METERS_PER_MILE, self.mile_splits);
        }

        // Update the heat map.
        //self.location_heat_map.append(latitude, longitude);

        self.last_time = date_time;
        self.last_lat = latitude;
        self.last_lon = longitude;
        self.last_alt = altitude;
    }
}
