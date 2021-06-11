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
        self.writer.write_attribute("creator", "activity-analyzer.app");
        self.writer.write_attribute("version", "1.1");
        self.writer.write_attribute("xsi:schemaLocation", "http://www.topografix.com/GPX/1/1 http://www.topografix.com/GPX/1/1/gpx.xsd http://www.garmin.com/xmlschemas/GpxExtensions/v3 http://www.garmin.com/xmlschemas/GpxExtensionsv3.xsd http://www.garmin.com/xmlschemas/TrackPointExtension/v1 http://www.garmin.com/xmlschemas/TrackPointExtensionv1.xsd");
        self.writer.write_attribute("xmlns", "http://www.topografix.com/GPX/1/1");
        self.writer.write_attribute("xmlns:gpxtpx", "http://www.garmin.com/xmlschemas/TrackPointExtension/v1");
        self.writer.write_attribute("xmlns:gpxx", "http://www.garmin.com/xmlschemas/GpxExtensions/v3");
        self.writer.write_attribute("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance");
    }
    pub fn close(self) -> String {
        let result = self.writer.end_document();
        result
    }

    pub fn write_metadata(&mut self, start_time_ms: u64) {
        self.writer.start_element("metadata");
        self.writer.start_element("time");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text(&GpxWriter::format_timestamp(start_time_ms));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
        self.writer.end_element();
    }
    pub fn write_name(&mut self, name: &str) {
        self.writer.start_element("name");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text(name);
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
    }
    pub fn write_type(&mut self, activity_type: &str) {
        self.writer.start_element("type");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text(activity_type);
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
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
        self.writer.start_element("ele");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &alt));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
        self.writer.start_element("time");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text(&GpxWriter::format_timestamp(time_ms));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
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
        self.writer.start_element("gpxtpx:hr");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &heart_rate_bpm));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
    }
    pub fn store_cadence_rpm(&mut self, cadence_rpm: u8) {
        self.writer.start_element("gpxtpx:cad");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &cadence_rpm));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
    }
    pub fn store_power_in_watts(&mut self, power_in_watts: u32) {
        self.writer.start_element("power");
        self.writer.set_preserve_whitespaces(true);
        self.writer.write_text_fmt(format_args!("{:?}", &power_in_watts));
        self.writer.end_element();
        self.writer.set_preserve_whitespaces(false);
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
