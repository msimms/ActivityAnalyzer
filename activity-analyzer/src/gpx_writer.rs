// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use xmlwriter::*;

pub struct GpxWriter {
    writer: XmlWriter,
}

impl GpxWriter {
    pub fn new() -> Self {
        let opt = Options { use_single_quote: true, attributes_indent: Indent::Spaces(2), indent: Indent::Spaces(2) };
        let writer = GpxWriter{ writer: XmlWriter::new(opt) };
        writer
    }

    pub fn open(&mut self) {
        self.writer.start_element("gpx");
    }
    pub fn close(self) -> String {
        let result = self.writer.end_document();
        result
    }

    pub fn write_metadata(&mut self, start_time: u64) {
    }
    pub fn write_name(&mut self, name: &str) {
    }
    
    pub fn start_track(&mut self) {
        self.writer.start_element("trk");
    }
    pub fn end_track(&mut self) {
        self.writer.end_element();
    }

    pub fn start_track_segment(&mut self) {
        self.writer.start_element("trkseg");
    }
    pub fn end_track_segment(&mut self) {
        self.writer.end_element();
    }

    pub fn start_track_point(&mut self, lat: f64, lon: f64, alt: f64, time_ms: u64) {
        self.writer.start_element("trkpt");
    }
    pub fn end_track_point(&mut self) {
        self.writer.end_element();
    }

    pub fn start_extensions(&mut self) {
        self.writer.start_element("extensions");
    }
    pub fn end_extensions(&mut self) {
        self.writer.end_element();
    }

    pub fn start_track_point_extensions(&mut self) {
        self.writer.start_element("gpxtpx:TrackPointExtension");
    }
    pub fn end_track_point_extensions(&mut self) {
        self.writer.end_element();
    }

    pub fn StoreHeartRateBpm(&mut self, heart_rate_bpm: u8) {
    }
    pub fn StoreCadenceRpm(&mut self, cadence_rpm: u8) {
    }
    pub fn StorePowerInWatts(&mut self, power_in_watts: u32) {
    }
}
