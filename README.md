# ActivityAnalyzer

Analyzes GPX, TCX, and FIT files.  Automatically identifies intervals. Does not store any personal data, all calculations are performed in the browser using WebAssembly. Work in progress. Will eventually connect to Strava and perhaps other services.

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

* 1.4 - Added first version of file merging [Issue 7](https://github.com/msimms/ActivityAnalyzer/issues/7).
* 1.3 - Fixed issues with FIT files that have developer defined fields. Added ability to show gear shift data [Issue 8](https://github.com/msimms/ActivityAnalyzer/issues/8).
* 1.2 - Initial support for file comparison [Issue 2](https://github.com/msimms/ActivityAnalyzer/issues/2).
* 1.1 - Added export and split capabilities [Issue 6](https://github.com/msimms/ActivityAnalyzer/issues/6).
* 1.0 - Fixed FIT file altitude readings and added first cut at power interval analysis [Issue 4](https://github.com/msimms/ActivityAnalyzer/issues/4).
* 0.9 - Added FIT file support [Issue 1](https://github.com/msimms/ActivityAnalyzer/issues/1).
* 0.8 - Added first cut at interval analysis.
* 0.7 - Added split times.
* 0.6 - Added TCX file support as well as power, cadence, and heart rate data from TCX files.
* 0.5 - Basic functionality, minus interval detection and TCX file handling.
