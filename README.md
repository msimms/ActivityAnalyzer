# ActivityAnalyzer

Analyzes GPX and TCX files.  Automatically identifies intervals. Does not store any personal data, all calculations are performed in the browser using WebAssembly. Work in progress. Will eventually connect to Strava and perhaps other services.

# Building

```
git clone https://github.com/msimms/ActivityAnalyzer
git submodule update --init
cd ActivityAnalyzer/activity-analyzer
wasm-pack build --target web
```

# Example

An example implementation is available at https://activity-analyzer.app.

# Version History

0.5 - Basic functionality, minus interval detection and TCX file handling.
