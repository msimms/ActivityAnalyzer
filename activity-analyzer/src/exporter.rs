// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use crate::analyzer_context::AnalyzerContext;
use crate::gpx_writer::GpxWriter;
use crate::tcx_writer::TcxWriter;

pub struct Exporter {
}

impl Exporter {
    pub fn new() -> Self {
        let exporter = Exporter{};
        exporter
    }

    fn export_gpx(&self, context: &AnalyzerContext) -> String {
        let mut writer = GpxWriter::new();
        writer.open();
        writer.start_track();
        writer.start_track_segment();

        let point_index = 0;
        let loc_data = &context.location_analyzer;
        writer.start_track_point(loc_data.latitude_readings[point_index], loc_data.longitude_readings[point_index], loc_data.altitude_graph[point_index], loc_data.times[point_index]);
        writer.end_track_point();

        writer.end_track_segment();
        writer.end_track();
        let result = writer.close();
        result
    }

    fn export_tcx(&self, context: &AnalyzerContext) -> String {
        let mut writer = TcxWriter::new();
        writer.open();
        writer.start_activities();
        writer.start_activity(&context.location_analyzer.activity_type);
        writer.write_id(context.location_analyzer.start_time_ms);
        writer.start_lap();
        writer.start_track();
        writer.start_trackpoint();

        let loc_index = 0;
        let loc_data = &context.location_analyzer;
        writer.store_time(loc_data.times[loc_index]);
        writer.store_altitude_meters(loc_data.altitude_graph[loc_index]);
        writer.store_position(loc_data.latitude_readings[loc_index], loc_data.longitude_readings[loc_index]);

        writer.end_trackpoint();
        writer.end_track();
        writer.end_lap();
        writer.end_activity();
        let result = writer.close();
        result
    }

    fn export_fit(&self, context: &AnalyzerContext) -> String {
        "".to_string()
    }

    pub fn export(&self, context: &AnalyzerContext, format: &str) -> String {
        let format_lower = format.to_lowercase();

        if format_lower == "gpx" {
            return self.export_gpx(context);
        }
        if format_lower == "tcx" {
            return self.export_tcx(context);
        }
        if format_lower == "fit" {
            return self.export_fit(context);
        }

        format_lower
    }
}
