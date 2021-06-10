// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use crate::location_analyzer::LocationAnalyzer;
use crate::heart_rate_analyzer::HeartRateAnalyzer;
use crate::cadence_analyzer::CadenceAnalyzer;
use crate::power_analyzer::PowerAnalyzer;

/// Context structure. An instance of this will be passed to the parser and ultimately to the callback function so we can use it for whatever.
pub struct AnalyzerContext {
    pub name: String,
    pub location_analyzer: LocationAnalyzer,
    pub hr_analyzer: HeartRateAnalyzer,
    pub cadence_analyzer: CadenceAnalyzer,
    pub power_analyzer: PowerAnalyzer,
}

impl AnalyzerContext {
    pub fn new() -> Self {
        let context = AnalyzerContext{
            name: "Unnamed".to_string(),
            location_analyzer: LocationAnalyzer::new(),
            hr_analyzer: HeartRateAnalyzer::new(),
            cadence_analyzer: CadenceAnalyzer::new(),
            power_analyzer: PowerAnalyzer::new() };
        context
    }
}