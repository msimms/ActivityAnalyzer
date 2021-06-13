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

    fn export_gpx(&self, context: &AnalyzerContext, split_start_offset_ms: u64, split_end_offset_ms: u64) -> String {
        let mut writer = GpxWriter::new();
        let loc_data = &context.location_analyzer;

        let lap_num = 1;
        let num_hr_readings = context.hr_analyzer.time_readings.len();
        let num_cad_readings = context.cadence_analyzer.time_readings.len();
        let num_power_readings = context.power_analyzer.time_readings.len();
        let mut hr_index = 0;
        let mut cad_index = 0;
        let mut power_index = 0;

        writer.open();
        writer.write_metadata(loc_data.start_time_ms);
        writer.start_track();
        writer.write_name(&context.name);
        writer.write_type(&loc_data.activity_type);
        writer.start_track_segment();

        let num_points = loc_data.latitude_readings.len();
        for point_index in 0..num_points - 1 {
            let ts = loc_data.times[point_index];
            let mut use_data_point;

            if split_start_offset_ms == 0 && split_end_offset_ms == 0 {
                use_data_point = true;
            }
            else if split_end_offset_ms == 0 {
                use_data_point = ts >= loc_data.start_time_ms + split_start_offset_ms;
            }
            else {
                use_data_point = ts >= loc_data.start_time_ms + split_start_offset_ms && ts < loc_data.start_time_ms + split_end_offset_ms;
            }

            if use_data_point {

                writer.start_track_point(loc_data.latitude_readings[point_index], loc_data.longitude_readings[point_index], loc_data.altitude_graph[point_index], ts);

                while hr_index < num_hr_readings && context.hr_analyzer.time_readings[hr_index] < ts {
                    hr_index = hr_index + 1;
                }
                while cad_index < num_cad_readings && context.cadence_analyzer.time_readings[cad_index] < ts {
                    cad_index = cad_index + 1;
                }
                while power_index < num_power_readings && context.power_analyzer.time_readings[power_index] < ts {
                    power_index = power_index + 1;
                }

                let mut has_extensions = false;

                if num_hr_readings > 0 && hr_index < num_hr_readings {
                    writer.start_extensions();
                    writer.start_track_point_extensions();
                    writer.store_heart_rate_bpm(context.hr_analyzer.readings[hr_index] as u8);
                    has_extensions = true;
                }
                if num_cad_readings > 0 && cad_index < num_cad_readings {
                    if !has_extensions {
                        writer.start_extensions();
                        writer.start_track_point_extensions();
                        has_extensions = true;
                    }
                    writer.store_cadence_rpm(context.cadence_analyzer.readings[cad_index] as u8);
                }
                if num_power_readings > 0 && power_index < num_power_readings {
                    if !has_extensions {
                        writer.start_extensions();
                        writer.start_track_point_extensions();
                        has_extensions = true;
                    }
                    writer.store_power_in_watts(context.power_analyzer.power_readings[power_index] as u32);
                }

                if has_extensions {
                    writer.end_track_point_extensions();
                    writer.end_extensions();
                }

                writer.end_track_point();
            }
        }

        writer.end_track_segment();
        writer.end_track();

        let result = writer.close();
        result
    }

    fn export_tcx(&self, context: &AnalyzerContext, split_start_offset_ms: u64, split_end_offset_ms: u64) -> String {
        let loc_data = &context.location_analyzer;
        let mut writer = TcxWriter::new();

        let lap_num = 1;
        let num_hr_readings = context.hr_analyzer.time_readings.len();
        let num_cad_readings = context.cadence_analyzer.time_readings.len();
        let num_power_readings = context.power_analyzer.time_readings.len();
        let mut hr_index = 0;
        let mut cad_index = 0;
        let mut power_index = 0;

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
            let ts = loc_data.times[loc_index];
            let mut use_data_point;

            if split_start_offset_ms == 0 && split_end_offset_ms == 0 {
                use_data_point = true;
            }
            else if split_end_offset_ms == 0 {
                use_data_point = ts >= loc_data.start_time_ms + split_start_offset_ms;
            }
            else {
                use_data_point = ts >= loc_data.start_time_ms + split_start_offset_ms && ts < loc_data.start_time_ms + split_end_offset_ms;
            }

            if use_data_point {

                writer.start_trackpoint();
                writer.store_time(ts);
                writer.store_position(loc_data.latitude_readings[loc_index], loc_data.longitude_readings[loc_index]);
                writer.store_altitude_meters(loc_data.altitude_graph[loc_index]);
                writer.store_distance_meters(0.0);

                while hr_index < num_hr_readings && context.hr_analyzer.time_readings[hr_index] < ts {
                    hr_index = hr_index + 1;
                }
                while cad_index < num_cad_readings && context.cadence_analyzer.time_readings[cad_index] < ts {
                    cad_index = cad_index + 1;
                }
                while power_index < num_power_readings && context.power_analyzer.time_readings[power_index] < ts {
                    power_index = power_index + 1;
                }

                if num_hr_readings > 0 && hr_index < num_hr_readings {
                    writer.store_heart_rate_bpm(context.hr_analyzer.readings[hr_index] as u8);
                }
                if num_cad_readings > 0 && cad_index < num_cad_readings {
                    writer.store_cadence_rpm(context.cadence_analyzer.readings[cad_index] as u8);
                }
                if num_power_readings > 0 && power_index < num_power_readings {
                    writer.store_power_in_watts(context.power_analyzer.power_readings[power_index] as u32);
                }

                writer.end_trackpoint();
            }
        }
        writer.end_track();
        writer.end_lap();
        writer.end_activity();
        writer.end_activities();

        let result = writer.close();
        result
    }

    fn export_fit(&self, context: &AnalyzerContext, split_start_offset_ms: u64, split_end_offset_ms: u64) -> String {
        "".to_string()
    }

    pub fn export(&self, context: &AnalyzerContext, format: &str, split_start_offset_ms: u64, split_end_offset_ms: u64) -> String {
        let format_lower = format.to_lowercase();

        if format_lower == "gpx" {
            return self.export_gpx(context, split_start_offset_ms, split_end_offset_ms);
        }
        if format_lower == "tcx" {
            return self.export_tcx(context, split_start_offset_ms, split_end_offset_ms);
        }
        if format_lower == "fit" {
            return self.export_fit(context, split_start_offset_ms, split_end_offset_ms);
        }

        format_lower
    }
}
