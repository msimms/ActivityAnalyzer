// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use crate::analyzer_context::AnalyzerContext;

pub struct MergeTool {
}

impl MergeTool {
    pub fn new() -> Self {
        let merge_tool = MergeTool{};
        merge_tool
    }

    fn merge_locations(&self, context1: &AnalyzerContext, context2: &AnalyzerContext, merged_context: &mut AnalyzerContext) {
        let loc_data1 = &context1.location_analyzer;
        let loc_data2 = &context2.location_analyzer;

        let num_points1 = loc_data1.latitude_readings.len();
        let num_points2 = loc_data2.latitude_readings.len();

        let mut point_index1 = 0;
        let mut point_index2 = 0;

        while point_index1 < num_points1 && point_index2 < num_points2 {
            let ts1 = loc_data1.times[point_index1];
            let ts2 = loc_data2.times[point_index2];

            let time_diff: i64 = (ts1 - ts2) as i64;
            let abs_time_diff = time_diff.abs();

            // If the points are less than one second apart then average them together.
            // Otherwise, use the earliest one.
            if abs_time_diff < 1000 {
                let lat1 = loc_data1.latitude_readings[point_index1];
                let lon1 = loc_data1.longitude_readings[point_index1];
                let alt1 = loc_data1.altitude_graph[point_index1];

                let lat2 = loc_data2.latitude_readings[point_index2];
                let lon2 = loc_data2.longitude_readings[point_index2];
                let alt2 = loc_data2.altitude_graph[point_index2];

                let new_ts = (ts1 + ts2) / 2;
                let new_lat = (lat1 + lat2) / 2.0;
                let new_lon = (lon1 + lon2) / 2.0;
                let new_alt = (alt1 + alt2) / 2.0;

                merged_context.location_analyzer.append_location(new_ts, new_lat, new_lon, new_alt);

                point_index1 = point_index1 + 1;
                point_index2 = point_index2 + 1;
            }
            else if ts1 < ts2 {
                merged_context.location_analyzer.append_location(ts1, loc_data1.latitude_readings[point_index1], loc_data1.longitude_readings[point_index1], loc_data1.altitude_graph[point_index1]);
                point_index1 = point_index1 + 1;
            }
            else {
                merged_context.location_analyzer.append_location(ts2, loc_data2.latitude_readings[point_index2], loc_data2.longitude_readings[point_index2], loc_data2.altitude_graph[point_index2]);
                point_index2 = point_index2 + 1;
            }
        }
    }

    fn merge_hr(&self, context1: &AnalyzerContext, context2: &AnalyzerContext, merged_context: &mut AnalyzerContext) {
        let data1 = &context1.hr_analyzer;
        let data2 = &context2.hr_analyzer;

        let num_points1 = data1.readings.len();
        let num_points2 = data2.readings.len();

        let mut point_index1 = 0;
        let mut point_index2 = 0;

        while point_index1 < num_points1 && point_index2 < num_points2 {
            let ts1 = data1.time_readings[point_index1];
            let ts2 = data2.time_readings[point_index2];

            let time_diff: i64 = (ts1 - ts2) as i64;
            let abs_time_diff = time_diff.abs();

            let value1 = data1.readings[point_index1];
            let value2 = data2.readings[point_index2];

            // If the points are less than one second apart then average them together.
            // Otherwise, use the earliest one.
            if abs_time_diff < 1000 {
                let new_ts = (ts1 + ts2) / 2;
                let new_value = (value1 + value2) / 2.0;

                merged_context.hr_analyzer.append_sensor_value(new_ts, new_value);

                point_index1 = point_index1 + 1;
                point_index2 = point_index2 + 1;
            }
            else if ts1 < ts2 {
                merged_context.hr_analyzer.append_sensor_value(ts1, value1);
                point_index1 = point_index1 + 1;
            }
            else {
                merged_context.hr_analyzer.append_sensor_value(ts2, value2);
                point_index2 = point_index2 + 1;
            }
        }
    }

    fn merge_power(&self, context1: &AnalyzerContext, context2: &AnalyzerContext, merged_context: &mut AnalyzerContext) {
        let data1 = &context1.power_analyzer;
        let data2 = &context2.power_analyzer;

        let num_points1 = data1.readings.len();
        let num_points2 = data2.readings.len();

        let mut point_index1 = 0;
        let mut point_index2 = 0;

        while point_index1 < num_points1 && point_index2 < num_points2 {
            let ts1 = data1.time_readings[point_index1];
            let ts2 = data2.time_readings[point_index2];

            let time_diff: i64 = (ts1 - ts2) as i64;
            let abs_time_diff = time_diff.abs();

            let value1 = data1.readings[point_index1];
            let value2 = data2.readings[point_index2];

            // If the points are less than one second apart then average them together.
            // Otherwise, use the earliest one.
            if abs_time_diff < 1000 {
                let new_ts = (ts1 + ts2) / 2;
                let new_value = (value1 + value2) / 2.0;

                merged_context.power_analyzer.append_sensor_value(new_ts, new_value);

                point_index1 = point_index1 + 1;
                point_index2 = point_index2 + 1;
            }
            else if ts1 < ts2 {
                merged_context.power_analyzer.append_sensor_value(ts1, value1);
                point_index1 = point_index1 + 1;
            }
            else
            {
                merged_context.power_analyzer.append_sensor_value(ts2, value2);
                point_index2 = point_index2 + 1;
            }
        }
    }

    pub fn merge(&self, context1: &AnalyzerContext, context2: &AnalyzerContext) -> AnalyzerContext {
        let mut merged_context = AnalyzerContext::new();

        self.merge_locations(context1, context2, &mut merged_context);
        self.merge_hr(context1, context2, &mut merged_context);
        self.merge_power(context1, context2, &mut merged_context);

        merged_context.location_analyzer.analyze();
        merged_context.power_analyzer.analyze();

        merged_context
    }
}
