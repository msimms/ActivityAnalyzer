// Copyright (c) 2021 Michael J. Simms. All rights reserved.

pub struct PowerAnalyzer {
    readings: Vec<f64>, // All the readings
    pub avg_power: f64,
    pub max_power: f64,
    np_buf: Vec<f64>,
    current_30_sec_buf: Vec<f64>,
    current_30_sec_buf_start_time: u64
}

impl PowerAnalyzer {
    pub fn new() -> Self {
        let analyzer = PowerAnalyzer{readings: Vec::new(), avg_power: 0.0, max_power: 0.0, np_buf: Vec::new(), current_30_sec_buf: Vec::new(), current_30_sec_buf_start_time: 0};
        analyzer
    }

    /// Adds another reading to the analyzer.
    pub fn append_sensor_value(&mut self, date_time_ms: u64, value: f64) {

        // Update average power.
        self.readings.push(value);

        // Update max power.
        if value > self.max_power {
            self.max_power = value;
        }
    }
}
