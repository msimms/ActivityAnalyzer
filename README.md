# ActivityAnalyzer

Extracts and analyzes interval data from a GPX or TCX file. Does not store any personal data, all calculations are performed in the browser using WebAssembly. Work in progress. Will eventually connect to Strava and perhaps other services.

# Building

```
git clone https://github.com/msimms/IntervalAnalyzer
cd IntervalAnalyzer/interval-analyzer
wasm-pack build --target web
```
