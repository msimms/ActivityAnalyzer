// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use xmlwriter::*;

pub struct TcxWriter {
    writer: XmlWriter,
}

impl TcxWriter {
    pub fn new() -> Self {
        let opt = Options { use_single_quote: true, attributes_indent: Indent::Spaces(2), indent: Indent::Spaces(2) };
        let writer = TcxWriter{ writer: XmlWriter::new(opt) };
        writer
    }

    pub fn open(&mut self) {
        self.writer.start_element("TrainingCenterDatabase");
    }

    pub fn start_activities(&mut self) {
        self.writer.start_element("Activities");
    }

    pub fn end_activities(&mut self) {
        self.writer.end_element();
    }

    pub fn start_activity(&mut self, description: &str) {
        self.writer.start_element("Activity");
        self.writer.write_attribute("Sport", description);
    }

    pub fn end_activity(&mut self) {
        self.writer.end_element();
    }

    pub fn start_lap(&mut self) {
        self.writer.start_element("Lap");
    }
    pub fn end_lap(&mut self) {
        self.writer.end_element();
    }

    pub fn start_track(&mut self){
        self.writer.start_element("Track");
    }
    pub fn end_track(&mut self) {
        self.writer.end_element();
    }

    pub fn start_trackpoint(&mut self) {
        self.writer.start_element("TrackPoint");
    }
    pub fn end_trackpoint(&mut self) {
        self.writer.end_element();
    }

    pub fn start_trackpoint_extensions(&mut self) {
        self.writer.start_element("Extensions");
    }
    pub fn end_trackpoint_extensions(&mut self) {
        self.writer.end_element();
    }

    pub fn store_time(&mut self, date_time_ms: u64) {
        self.writer.write_attribute("Time", &date_time_ms);
    }
    pub fn store_altitude_meters(&mut self, altitude_meters: f64) {
        self.writer.write_attribute("AltitudeMeters", &altitude_meters);
    }
    pub fn store_distance_meters(&mut self, distance_meters: f64) {
        self.writer.write_attribute("DistanceMeters", &distance_meters);
    }
    pub fn store_heart_rate_bpm(&mut self, heart_rate_bpm: u8) {
        self.writer.start_element("HeartRateBpm");
        self.writer.write_attribute("Value", &heart_rate_bpm);
        self.writer.end_element();
    }
    pub fn store_cadence_rpm(&mut self, cadence_rpm: u8) {
        self.writer.write_attribute("Cadence", &cadence_rpm);
    }
    pub fn store_power_in_watts(&mut self, power_watts: u32) {
    }
    pub fn store_position(&mut self, lat: f64, lon: f64) {
        self.writer.start_element("Position");
        self.writer.write_attribute("LatitudeDegrees", &lat);
        self.writer.write_attribute("LongitudeDegrees", &lon);
        self.writer.end_element();
    }

    pub fn close(self) -> String {
        let result = self.writer.end_document();
        result
    }
}
