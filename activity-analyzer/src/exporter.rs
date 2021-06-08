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
        writer.close();
        "".to_string()
    }

    fn export_tcx(&self, context: &AnalyzerContext) -> String {
        let mut writer = TcxWriter::new();
        writer.open();
        writer.start_activities();
        writer.start_activity("");
        writer.start_lap();
        writer.start_track();
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
        if format == "gpx" {
            return self.export_gpx(context);
        }
        else if format == "tcx" {
            return self.export_tcx(context);
        }
        else if format == "fit" {
            return self.export_fit(context);
        }

        "".to_string()
    }
}
