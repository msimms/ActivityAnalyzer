// Copyright (c) 2021 Michael J. Simms. All rights reserved.

pub struct CadenceAnalyzer {
    pub readings: Vec<f64>, // All the readings
    pub max_cadence: f64,
}

impl CadenceAnalyzer {
    pub fn new() -> Self {
        let analyzer = CadenceAnalyzer{readings: Vec::new(), max_cadence: 0.0};
        analyzer
    }

    /// Computes the average value.
    pub fn compute_average(&self) -> f64 {
        let count = self.readings.len();
        if count > 0 {
            let sum: f64 = Iterator::sum(self.readings.iter());
            return f64::from(sum) / (self.readings.len() as f64);
        }
        0.0
    }

    /// Adds another reading to the analyzer.
    pub fn append_sensor_value(&mut self, _date_time_ms: u64, value: f64) {

        // Update average power.
        self.readings.push(value);

        // Update max power.
        if value > self.max_cadence {
            self.max_cadence = value;
        }
    }
}
