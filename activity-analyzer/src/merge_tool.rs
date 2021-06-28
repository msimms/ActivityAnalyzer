// Copyright (c) 2021 Michael J. Simms. All rights reserved.

use crate::analyzer_context::AnalyzerContext;

pub struct MergeTool {
}

impl MergeTool {
    pub fn new() -> Self {
        let merge_tool = MergeTool{};
        merge_tool
    }

    pub fn merge(&self, context1: &AnalyzerContext, context2: &AnalyzerContext) -> AnalyzerContext {
        let mut merged_context = AnalyzerContext::new();
        merged_context
    }
}
