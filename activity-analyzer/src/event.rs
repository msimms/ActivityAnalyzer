// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Event {
    pub timestamp_ms: u64, // Timestamp (in milliseconds) at which the event occurred.
    pub event_type: u8, // Event type, from the file.
    pub event_data: u8, // Event-type dependent data.
}

impl Event {
    pub fn new() -> Self {
        let evt = Event{ timestamp_ms: 0, event_type: 0, event_data: 0 };
        evt
    }
}
