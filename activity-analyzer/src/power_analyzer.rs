// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use lib_math::{kmeans, peaks, statistics, signals};
use std::collections::HashMap;
use serde::Serialize;

pub const BEST_5_SEC_POWER: &str = "5 Second Power";
pub const BEST_12_MIN_POWER: &str = "12 Minute Power";
pub const BEST_20_MIN_POWER: &str = "20 Minute Power";
pub const BEST_1_HOUR_POWER: &str = "1 Hour Power";

#[derive(Clone, Copy, Serialize)]
pub struct PowerIntervalDescription {
    pub start_time: u64,
    pub end_time: u64,
    pub avg_power: f64
}

impl PowerIntervalDescription {
    pub fn new() -> Self {
        PowerIntervalDescription{ start_time: 0, end_time: 0, avg_power: 0.0 }
    }
}

impl Default for PowerIntervalDescription {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PowerAnalyzer {
    pub readings: Vec<f64>, // All the readings (power)
    pub time_readings: Vec<u64>, // All the readings (time)
    pub max_power: f64,
    pub avg_power: f64,
    pub np_buf: Vec<f64>,
    pub np: f64, // Normalized power
    pub vi: f64, // Variability index
    current_30_sec_buf: Vec<f64>,
    current_30_sec_buf_start_time: u64,
    pub bests: HashMap<String, f64>,
    pub significant_intervals: Vec<PowerIntervalDescription>,
    start_time_ms: u64,
    end_time_ms: u64
}

impl PowerAnalyzer {
    pub fn new() -> Self {
        PowerAnalyzer{ readings: Vec::new(), time_readings: Vec::new(), max_power: 0.0, avg_power: 0.0, np_buf: Vec::new(), np: 0.0, vi: 0.0,
            current_30_sec_buf: Vec::new(), current_30_sec_buf_start_time: 0, bests: HashMap::new(), significant_intervals: Vec::new(), start_time_ms: 0, end_time_ms: 0 }
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

    /// Returns the time associated with the specified record, or None if not found.
    pub fn get_best_power(&self, record_name: &str) -> f64 {
        match self.bests.get(record_name) {
            Some(&number) => number,
            _ => 0.0,
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
        self.readings.push(value);

        // Calculate the current activity duration.
        let duration_ms = self.end_time_ms - self.start_time_ms;

        // Update max power.
        if value > self.max_power {
            self.max_power = value;
        }

        // Update the buffers needed for the normalized power calculation.
        if date_time_ms - self.current_30_sec_buf_start_time > 30000 {
            if !self.current_30_sec_buf.is_empty() {
                let sum_norm_power: f64 = Iterator::sum(self.current_30_sec_buf.iter());
                let avg_norm_power = sum_norm_power / self.current_30_sec_buf.len() as f64;

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

    fn compute_normalized_power(&mut self) {

        if self.np_buf.len() > 1 {

            // Throw away the first 30 second average.
            self.np_buf.pop();

            // Needs this for the variability index calculation.
            let sum_norm_power: f64 = Iterator::sum(self.np_buf.iter());
            let avg_norm_power = sum_norm_power / self.np_buf.len() as f64;

            // Raise all items to the fourth power.
            let mut sum_norm_power2 = 0.0;
            for item in self.np_buf.iter() {
                sum_norm_power2 += item.powf(4.0);
            }

            // Average the values that were raised to the fourth.
            let avg_norm_power2 = sum_norm_power2 / self.np_buf.len() as f64;

            // Take the fourth root.
            self.np = avg_norm_power2.powf(0.25);

            // Compute the variability index (VI = NP / AP).
            self.vi  = self.np / avg_norm_power;
        }
    }

    fn examine_interval_peak(&mut self, start_index: usize, end_index: usize) -> Option<PowerIntervalDescription> {
        // Examines a line of near-constant pace/speed.
        if start_index >= end_index {
            let result: Option::<PowerIntervalDescription> = None;
            return result;
        }

        // How long (in seconds) was this block?
        let start_time = self.time_readings[start_index];
        let end_time = self.time_readings[end_index];

        // Don't consider anything less than ten seconds.
        if end_time - start_time < 10 {
            let result: Option::<PowerIntervalDescription> = None;
            return result;
        }

        let powers = &self.readings[start_index..end_index - 1];
        let avg_power = statistics::average_f64(&powers.to_vec());
        let desc = PowerIntervalDescription{start_time, end_time, avg_power};
        Some(desc)
    }

    /// Performs a k-means analysis on peaks extracted from the power data to look for intervals.
    fn search_for_intervals(&mut self) {

        if self.readings.len() > 1 {

            // Compute the speed/pace variation. This will tell us how consistent the pace was.
            let power_variance = statistics::variance_f64(&self.readings, self.avg_power);

            // Don't look for peaks unless the variance was high. Cutoff selected via experimentation.
            // Also, don't search if the feature has been disabled.
            if power_variance > 0.50 {

                // Smooth the speed graph to take out some of the GPS jitter.
                let smoothed_graph = signals::smooth(&self.readings, 4);
                if smoothed_graph.len() > 1 {

                    // Find peaks in the speed graph. We're looking for intervals.
                    let peak_list = peaks::find_peaks(&smoothed_graph, 1.0);

                    // Examine the lines between the peaks. Extract pertinant data, such as avg speed/pace and set it aside.
                    // This data is used later when generating the report.
                    let mut filtered_interval_list = Vec::new();
                    for peak in peak_list {
                        let interval = self.examine_interval_peak(peak.left_trough.x, peak.right_trough.x);
                        if let Some(interval) = interval {
                            filtered_interval_list.push(interval);
                        }
                    }

                    // Do a k-means analysis on a 2D the computed speed/pace blocks so we can get rid of any outliers.
                    let num_possible_intervals = filtered_interval_list.len();
                    if num_possible_intervals >= 2 {

                        // Convert the interval description into something k-means can work with.
                        let sample_dimensions = 2;
                        let mut samples = vec![0.0_f64; sample_dimensions * num_possible_intervals];
                        let mut sample_index = 0;
                        for interval in &filtered_interval_list {
                            samples[sample_index] = interval.avg_power;
                            sample_index += 1;
                            samples[sample_index] = (interval.end_time - interval.start_time) as f64;
                            sample_index += 1;
                        }

                        // Determine the maximum value of k.
                        let mut max_k = 10;
                        if num_possible_intervals < max_k {
                            max_k = num_possible_intervals;
                        }

                        // Run k means for each possible k.
                        let mut best_k = 0;
                        let mut best_labels = Vec::<usize>::new();
                        let mut steepest_slope = 0.0;
                        let mut distortions = Vec::<f64>::new();
                        let max_error = 1.0;
                        let max_iter = 100;
                        for k in 1..max_k {
                            let (labels, distortion) = kmeans::kmeans_equally_space_centroids(samples.to_vec(), sample_dimensions, k, max_error, max_iter);

                            // Store the error.
                            distortions.push(distortion);

                            // Use the elbow method to find the best value for k.
                            if distortions.len() >= 2 && k >= 2 {
                                let slope = (distortions[k-1] + distortions[k-2]) / 2.0;

                                if best_k == 0 || slope > steepest_slope {
                                    best_k = k;
                                    best_labels = labels;
                                    steepest_slope = slope;
                                }
                            }
                        }

                        // Save off the significant peaks.
                        let mut interval_index = 0;
                        for label in best_labels {
                            if label >= 1 {
                                let interval = filtered_interval_list[interval_index];
                                self.significant_intervals.push(interval);
                            }
                            interval_index += 1;
                        }
                    }
                }
            }
        }
    }

    /// Called after all data is loaded.
    pub fn analyze(&mut self) {
        self.avg_power = self.compute_average();
        self.compute_normalized_power();
        self.search_for_intervals();
    }
}

impl Default for PowerAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
