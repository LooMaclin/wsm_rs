### wsm_rs

WebSphere monitoring utility written on Rust.

### Requirements

You need to install `PerfServlet` to `WebSphere`.

### Information

This tool uses PerfServlet to get specific metrics from WebSphere, parses them and pushes to the InfluxDB.

### Build

Run `cargo run` command.

### Visualising

You can check results with `Grafana` or use `InfluxDB CLI` or `InfluxDB` admin dashboard.

Samples:

![Sample 1](https://sc-cdn.scaleengine.net/i/2faa47f637e83b354b6c6341e1b98181.png)

![Sample 2](https://sc-cdn.scaleengine.net/i/99a7a32346291383260f435e6fa27c7e.png)

![Sample 3](https://sc-cdn.scaleengine.net/i/3f09504bdddc9767c4089006e5827899.png)
