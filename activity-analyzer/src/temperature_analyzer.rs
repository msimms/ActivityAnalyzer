// Copyright (c) 2021 Michael J. Simms. All rights reserved.

pub struct TemperatureAnalyzer {
    pub readings: Vec<f64>, // All the readings
    pub time_readings: Vec<u64>, // All the readings (time)
    pub max_temp: f64
}

impl TemperatureAnalyzer {
    pub fn new() -> Self {
        let analyzer = TemperatureAnalyzer{readings: Vec::new(), time_readings: Vec::new(), max_temp: 0.0};
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
    pub fn append_sensor_value(&mut self, date_time_ms: u64, value: f64) {

        // Update our state.
        self.readings.push(value);
        self.time_readings.push(date_time_ms);

        // Update max temperature.
        if value > self.max_temp {
            self.max_temp = value;
        }
    }
}
