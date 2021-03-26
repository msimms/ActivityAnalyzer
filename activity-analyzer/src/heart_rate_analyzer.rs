// Copyright (c) 2021 Michael J. Simms. All rights reserved.

pub struct HeartRateAnalyzer {
    pub readings: Vec<f64>, // All the readings
    pub max_hr: f64
}

impl HeartRateAnalyzer {
    pub fn new() -> Self {
        let analyzer = HeartRateAnalyzer{readings: Vec::new(), max_hr: 0.0};
        analyzer
    }

    /// Computes the average value.
    pub fn compute_average(&self) -> f64 {
        let count = self.readings.len();
        if count > 0 {
            let sum: f64 = Iterator::sum(self.readings.iter());
            return f64::from(sum) / (count as f64);
        }
        0.0
    }

    /// Adds another reading to the analyzer.
    pub fn append_sensor_value(&mut self, _date_time_ms: u64, value: f64) {

        // Update average heart rate.
        self.readings.push(value);

        // Update max heart rate.
        if value > self.max_hr {
            self.max_hr = value;
        }
    }
}
