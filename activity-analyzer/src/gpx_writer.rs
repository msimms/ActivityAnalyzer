// Copyright (c) 2021 Michael J. Simms. All rights reserved.

extern crate chrono;

use chrono::*;
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
        self.writer.write_attribute("lat", &lat);
        self.writer.write_attribute("lon", &lon);
        self.writer.write_attribute("ele", &alt);
        self.writer.write_attribute("time", &GpxWriter::format_timestamp(time_ms));
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

    pub fn store_heart_rate_bpm(&mut self, heart_rate_bpm: u8) {
        self.writer.write_attribute("gpxtpx:hr", &heart_rate_bpm);
    }
    pub fn store_cadence_rpm(&mut self, cadence_rpm: u8) {
        self.writer.write_attribute("gpxtpx:cad", &cadence_rpm);
    }
    pub fn store_power_in_watts(&mut self, power_in_watts: u32) {
        self.writer.write_attribute("power", &power_in_watts);
    }

    fn format_timestamp(t: u64) -> String {
        let sec  = t / 1000;
        let ms = t % 1000;

        let naive = NaiveDateTime::from_timestamp(sec as i64, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

        let buf1 = datetime.format("%Y-%m-%dT%H:%M:%S");
        let buf2 = format!("{}.{:03}Z", buf1, ms);
        buf2
    }
}
