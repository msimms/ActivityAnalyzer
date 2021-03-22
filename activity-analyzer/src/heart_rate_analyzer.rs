// Copyright (c) 2021 Michael J. Simms. All rights reserved.

pub struct HeartRateAnalyzer {
    readings: Vec<f64>, // All the readings
    pub avg_hr: f64,
    pub max_hr: f64
}

impl HeartRateAnalyzer {
    pub fn new() -> Self {
        let analyzer = HeartRateAnalyzer{readings: Vec::new(), avg_hr: 0.0, max_hr: 0.0};
        analyzer
    }

    /// Adds another reading to the analyzer.
    pub fn append_sensor_value(&mut self, date_time_ms: u64, value: f64) {

        // Update average heart rate.
        self.readings.push(value);

        // Update max heart rate.
        if value > self.max_hr {
            self.max_hr = value;
        }
    }
}
