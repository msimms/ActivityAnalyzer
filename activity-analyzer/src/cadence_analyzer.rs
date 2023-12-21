// Copyright (c) 2021 Michael J. Simms. All rights reserved.

pub struct CadenceAnalyzer {
    pub readings: Vec<f64>, // All the readings
    pub time_readings: Vec<u64>, // All the readings (time)
    pub max_cadence: f64,
}

impl CadenceAnalyzer {
    /// Creates a new [`CadenceAnalyzer`].
    pub fn new() -> Self {
        CadenceAnalyzer{ readings: Vec::new(), time_readings: Vec::new(), max_cadence: 0.0 }
    }

    /// Computes the average value.
    pub fn compute_average(&self) -> f64 {
        let count = self.readings.len();
        if count > 0 {
            let sum: f64 = Iterator::sum(self.readings.iter());
            return sum / (count as f64);
        }
        0.0
    }

    /// Adds another reading to the analyzer.
    pub fn append_sensor_value(&mut self, date_time_ms: u64, value: f64) {

        // Update our state.
        self.readings.push(value);
        self.time_readings.push(date_time_ms);

        // Update max power.
        if value > self.max_cadence {
            self.max_cadence = value;
        }
    }
}

impl Default for CadenceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
