// Copyright (c) 2021 Michael J. Simms. All rights reserved.

pub struct SwimAnalyzer {
    pub pool_length: u16, // Pool length
    pub pool_length_units: u8, // Pool length, in meters / 100
    pub strokes: Vec<u16>, // Strokes per lap
    pub time_readings: Vec<u64>, // All the readings (time)
}

impl SwimAnalyzer {
    pub fn new() -> Self {
        let analyzer = SwimAnalyzer{pool_length: 0, pool_length_units: 0, strokes: Vec::new(), time_readings: Vec::new()};
        analyzer
    }

    pub fn get_start_time_ms(&self) -> u64 {
        if self.time_readings.len() > 0 {
            return self.time_readings[0];
        }
        0
    }
    pub fn get_last_time_ms(&self) -> u64 {
        if self.time_readings.len() > 0 {
            return *self.time_readings.last().unwrap();
        }
        0
    }
    pub fn get_total_distance(&self) -> u64 {
        let distance = (self.time_readings.len() * self.pool_length as usize) as u64;
        distance / 100
    }

    pub fn set_pool_length(&mut self, pool_length: u16) {
        self.pool_length = pool_length;
    }
    pub fn set_pool_length_units(&mut self, pool_length_units: u8) {
        self.pool_length_units = pool_length_units;
    }

    /// Adds another reading to the analyzer.
    pub fn append_sensor_value(&mut self, date_time_ms: u64, value: u16) {

        // Update our state.
        self.strokes.push(value);
        self.time_readings.push(date_time_ms);
    }
}
