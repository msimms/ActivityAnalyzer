// Copyright (c) 2021 Michael J. Simms. All rights reserved.

pub struct SwimAnalyzer {
    pub strokes: Vec<u16>, // Strokes per lap
    pub time_readings: Vec<u64>, // All the readings (time)
}

impl SwimAnalyzer {
    pub fn new() -> Self {
        let analyzer = SwimAnalyzer{strokes: Vec::new(), time_readings: Vec::new()};
        analyzer
    }

    /// Adds another reading to the analyzer.
    pub fn append_sensor_value(&mut self, date_time_ms: u64, value: u16) {

        // Update our state.
        self.strokes.push(value);
        self.time_readings.push(date_time_ms);
    }
}
