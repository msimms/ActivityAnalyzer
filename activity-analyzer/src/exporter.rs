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
        let loc_data = &context.location_analyzer;

        writer.open();
        writer.write_metadata(loc_data.start_time_ms);
        writer.start_track();
        writer.write_name(&context.name);
        writer.write_type(&loc_data.activity_type);
        writer.start_track_segment();

        let num_points = loc_data.latitude_readings.len();
        for point_index in 0..num_points - 1 {
            writer.start_track_point(loc_data.latitude_readings[point_index], loc_data.longitude_readings[point_index], loc_data.altitude_graph[point_index], loc_data.times[point_index]);
            writer.end_track_point();
        }

        writer.end_track_segment();
        writer.end_track();

        let result = writer.close();
        result
    }

    fn export_tcx(&self, context: &AnalyzerContext) -> String {
        let loc_data = &context.location_analyzer;
        let mut writer = TcxWriter::new();
        let lap_num = 1;

        writer.open();
        writer.start_activities();
        writer.start_activity(&loc_data.activity_type);
        writer.write_id(loc_data.start_time_ms);

        writer.start_lap(loc_data.get_lap_start_time(lap_num));
        writer.store_lap_seconds(loc_data.get_lap_seconds(lap_num));
        writer.store_lap_distance(loc_data.get_lap_distance(lap_num));
        writer.store_lap_calories(loc_data.get_lap_calories(lap_num) as u16);

        writer.start_track();

        let num_locs = loc_data.latitude_readings.len();
        for loc_index in 0..num_locs - 1 {
            writer.start_trackpoint();
            writer.store_time(loc_data.times[loc_index]);
            writer.store_position(loc_data.latitude_readings[loc_index], loc_data.longitude_readings[loc_index]);
            writer.store_altitude_meters(loc_data.altitude_graph[loc_index]);
            writer.store_distance_meters(0.0);
            writer.end_trackpoint();
        }
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
