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

    pub fn write(&mut self, date_time_ms: u64, value: f64) {
    }

    pub fn close(&mut self) -> String {
        //self.writer.end_document();
        "".to_string()
    }
}
