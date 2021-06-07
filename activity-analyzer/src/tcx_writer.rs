// Copyright (c) 2021 Michael J. Simms. All rights reserved.

pub struct TcxWriter {
}

impl TcxWriter {
    pub fn new() -> Self {
        let writer = TcxWriter{};
        writer
    }

    pub fn open(&self) {
    }

    pub fn write(&mut self, date_time_ms: u64, value: f64) {
    }
}
