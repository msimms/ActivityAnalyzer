// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use std::collections::HashMap;

pub const BEST_5_SEC_POWER: &str = "5 Second Power";
pub const BEST_12_MIN_POWER: &str = "12 Minute Power";
pub const BEST_20_MIN_POWER: &str = "20 Minute Power";
pub const BEST_1_HOUR_POWER: &str = "1 Hour Power";

pub struct PowerAnalyzer {
    pub power_readings: Vec<f64>, // All the readings (power)
    pub time_readings: Vec<u64>, // All the readings (time)
    pub max_power: f64,
    pub np_buf: Vec<f64>,
    pub np: f64, // Normalized power
    pub vi: f64, // Variability index
    current_30_sec_buf: Vec<f64>,
    current_30_sec_buf_start_time: u64,
    pub bests: HashMap<String, f64>,
    start_time_ms: u64,
    end_time_ms: u64
}

impl PowerAnalyzer {
    pub fn new() -> Self {
        let analyzer = PowerAnalyzer{power_readings: Vec::new(), time_readings: Vec::new(), max_power: 0.0, np_buf: Vec::new(), np: 0.0, vi: 0.0,
            current_30_sec_buf: Vec::new(), current_30_sec_buf_start_time: 0, bests: HashMap::new(), start_time_ms: 0, end_time_ms: 0};
        analyzer
    }

    /// Computes the average value.
    pub fn compute_average(&self) -> f64 {
        let count = self.power_readings.len();
        if count > 0 {
            let sum: f64 = Iterator::sum(self.power_readings.iter());
            return f64::from(sum) / (count as f64);
        }
        0.0
    }

    /// Returns the time associated with the specified record, or None if not found.
    pub fn get_best_power(&self, record_name: &str) -> f64 {
        match self.bests.get(record_name) {
            Some(&number) => return number,
            _ => return 0.0,
        }
    }

    /// Looks up the existing record and, if necessary, updates it.
    fn do_power_record_check(&self, record_name: &str, watts: f64) -> bool {
        let old_value = self.get_best_power(record_name);
        if old_value <= 0.1 || watts > old_value {
            return true;
        }
        false
    }

    /// Adds another reading to the analyzer.
    pub fn append_sensor_value(&mut self, date_time_ms: u64, value: f64) {

        // Update our state.
        if self.start_time_ms == 0 {
            self.start_time_ms = date_time_ms;
        }
        self.end_time_ms = date_time_ms;
        self.time_readings.push(date_time_ms);
        self.power_readings.push(value);

        // Calculate the current activity duration.
        let duration_ms = self.end_time_ms - self.start_time_ms;

        // Update max power.
        if value > self.max_power {
            self.max_power = value;
        }

        // Update the buffers needed for the normalized power calculation.
        if date_time_ms - self.current_30_sec_buf_start_time > 30000 {
            if self.current_30_sec_buf.len() > 0 {
                let sum_norm_power: f64 = Iterator::sum(self.current_30_sec_buf.iter());
                let avg_norm_power = f64::from(sum_norm_power) / self.current_30_sec_buf.len() as f64;
                self.np_buf.push(avg_norm_power);
                self.current_30_sec_buf = Vec::new();
            }
            self.current_30_sec_buf_start_time = date_time_ms;
        }
        self.current_30_sec_buf.push(value);

        // Search for best efforts.
        let readings_iter = self.time_readings.iter().rev();
        for reading in readings_iter {
            let curr_time_diff = (self.end_time_ms - reading) / 1000;

            if curr_time_diff == 5 {
                let average_power = self.compute_average();
                if self.do_power_record_check(BEST_5_SEC_POWER, average_power) {
                    self.bests.entry(BEST_5_SEC_POWER.to_string()).or_insert(average_power);
                }
                if duration_ms < 720 {
                    return;
                }
            }
            else if curr_time_diff == 720 {
                let average_power = self.compute_average();
                if self.do_power_record_check(BEST_12_MIN_POWER, average_power) {
                    self.bests.entry(BEST_12_MIN_POWER.to_string()).or_insert(average_power);
                }
                if duration_ms < 1200 {
                    return;
                }
            }
            else if curr_time_diff == 1200 {
                let average_power = self.compute_average();
                if self.do_power_record_check(BEST_20_MIN_POWER, average_power) {
                    self.bests.entry(BEST_20_MIN_POWER.to_string()).or_insert(average_power);
                }
                if duration_ms < 3600 {
                    return;
                }
            }
            else if curr_time_diff == 3600 {
                let average_power = self.compute_average();
                if self.do_power_record_check(BEST_1_HOUR_POWER, average_power) {
                    self.bests.entry(BEST_1_HOUR_POWER.to_string()).or_insert(average_power);
                }
            }
            else if curr_time_diff > 3600 {
                return;
            }
        }
    }

    pub fn analyze(&mut self) {

        // Compute normalized power.
        if self.np_buf.len() > 1 {

            // Throw away the first 30 second average.
            self.np_buf.pop();

            // Needs this for the variability index calculation.
            let sum_norm_power: f64 = Iterator::sum(self.np_buf.iter());
            let avg_norm_power = f64::from(sum_norm_power) / self.np_buf.len() as f64;

            // Raise all items to the fourth power.
            let mut sum_norm_power2 = 0.0;
            for item in self.np_buf.iter() {
                sum_norm_power2 = sum_norm_power2 + item.powf(4.0);
            }

            // Average the values that were raised to the fourth.
            let avg_norm_power2 = f64::from(sum_norm_power2) / self.np_buf.len() as f64;

            // Take the fourth root.
            self.np = avg_norm_power2.powf(0.25);

            // Compute the variability index (VI = NP / AP).
            self.vi  = self.np / avg_norm_power;
        }
    }
}
