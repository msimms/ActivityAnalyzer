// Copyright (c) 2021 Michael J. Simms. All rights reserved.

extern crate chrono;

use chrono::*;
use xmlwriter::*;

pub struct TcxWriter {
    writer: XmlWriter,
}

impl TcxWriter {
    pub fn new() -> Self {
        let opt = Options { use_single_quote: true, attributes_indent: Indent::Spaces(2), indent: Indent::Spaces(2) };
        TcxWriter{ writer: XmlWriter::new(opt) }
    }

    pub fn open(&mut self) {
        self.writer.start_element("TrainingCenterDatabase");
        self.writer.write_attribute("xmlns", "http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2");
        self.writer.write_attribute("xmlns:xsd", "http://www.w3.org/2001/XMLSchema");
        self.writer.write_attribute("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance");
        self.writer.write_attribute("xmlns:tc2", "http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2");
        self.writer.write_attribute("targetNamespace", "http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2");
        self.writer.write_attribute("elementFormDefault", "qualified");
    }
    pub fn close(self) -> String {
        self.writer.end_document()
    }

    pub fn write_id(&mut self, start_time_ms: u64) {
        self.writer.start_element("Id");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text(&TcxWriter::format_timestamp(start_time_ms));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
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

    pub fn start_lap(&mut self, start_time_ms: u64) {
        self.writer.start_element("Lap");
        self.writer.write_attribute("StartTime", &TcxWriter::format_timestamp(start_time_ms));
    }
    pub fn end_lap(&mut self) {
        self.writer.end_element();
    }
    pub fn store_lap_seconds(&mut self, time_ms: u64) {
        let time_sec = time_ms as f64 / 1000.0;
        self.writer.start_element("TotalTimeSeconds");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &time_sec));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
    }
    pub fn store_lap_distance(&mut self, distance_meters: f64) {
        self.writer.start_element("DistanceMeters");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &distance_meters));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
    }
    pub fn store_lap_calories(&mut self, calories: u16) {
        self.writer.start_element("Calories");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &calories));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
    }
    pub fn store_lap_max_speed(&mut self, max_speed: f64) {
        self.writer.start_element("MaximumSpeed");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &max_speed));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
    }

    pub fn start_track(&mut self){
        self.writer.start_element("Track");
    }
    pub fn end_track(&mut self) {
        self.writer.end_element();
    }

    pub fn start_trackpoint(&mut self) {
        self.writer.start_element("Trackpoint");
    }
    pub fn end_trackpoint(&mut self) {
        self.writer.end_element();
    }

    pub fn start_trackpoint_extensions(&mut self) {
        self.writer.start_element("Extensions");
        self.writer.start_element("TPX");
    }
    pub fn end_trackpoint_extensions(&mut self) {
        self.writer.end_element();
        self.writer.end_element();
    }

    pub fn store_time(&mut self, date_time_ms: u64) {
        self.writer.start_element("Time");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text(&TcxWriter::format_timestamp(date_time_ms));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
    }
    pub fn store_altitude_meters(&mut self, altitude_meters: f64) {
        self.writer.start_element("AltitudeMeters");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &altitude_meters));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
    }
    pub fn store_distance_meters(&mut self, distance_meters: f64) {
        self.writer.start_element("DistanceMeters");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &distance_meters));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
    }
    pub fn store_heart_rate_bpm(&mut self, heart_rate_bpm: u8) {
        self.writer.start_element("HeartRateBpm");
        self.writer.start_element("Value");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &heart_rate_bpm));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
        self.writer.end_element();
    }
    pub fn store_cadence_rpm(&mut self, cadence_rpm: u8) {
        self.writer.start_element("Cadence");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &cadence_rpm));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
    }
    pub fn store_power_in_watts(&mut self, power_watts: u32) {
        self.writer.start_element("Watts");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &power_watts));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
    }
    pub fn store_position(&mut self, lat: f64, lon: f64) {
        self.writer.start_element("Position");

        self.writer.start_element("LatitudeDegrees");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &lat));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
 
        self.writer.start_element("LongitudeDegrees");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &lon));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);

        self.writer.end_element();
    }

    fn format_timestamp(t: u64) -> String {
        let sec  = t / 1000;
        let ms = t % 1000;

        let datetime = DateTime::<Utc>::from_timestamp(sec as i64, 0).unwrap();
        let buf1 = datetime.format("%Y-%m-%dT%H:%M:%S");
        let buf2 = format!("{}.{:03}Z", buf1, ms);
        buf2
    }
}
