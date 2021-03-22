# ActivityAnalyzer

Extracts and analyzes interval data from a GPX or TCX file. Does not store any personal data, all calculations are performed in the browser using WebAssembly. Work in progress. Will eventually connect to Strava and perhaps other services.

# Building

```
git clone https://github.com/msimms/ActivityAnalyzer
git submodule update --init
cd ActivityAnalyzer/activity-analyzer
wasm-pack build --target web
```
