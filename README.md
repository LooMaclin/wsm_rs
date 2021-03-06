### wsm_rs

WebSphere monitoring utility written on Rust.

### Requirements

You need to install `PerfServlet` to `WebSphere`.

### TODO

- [X] Getting XML from `PerfServlet` (with `Hyper`)
- [X] Parsing and collect statistics from XML.
- [X] Pushing the collected data to `InfluxDB`.
- [X] Docker container.
- [X] Create CLI with customising parameters.
- [ ] Fixed pushing interval (now i'ts only N-sec wait, without correlation)
- [ ] Generating basic dashboard for `Grafana` (basics on IBM `WebSphere` Monitoring [book](http://www.redbooks.ibm.com/redpapers/pdfs/redp4579.pdf)).
- [ ] Anything else?

### Information

This tool uses PerfServlet to get specific metrics from WebSphere, parses them and pushes to the InfluxDB.

### Build

Run `cargo run` command.

### Visualising

This tool uses `PerfServlet` to get specific metrics from `WebSphere` and pushes the data to the `InfluxDB` after it's parsed.

Samples:

![Sample 1](https://sc-cdn.scaleengine.net/i/2faa47f637e83b354b6c6341e1b98181.png)

![Sample 2](https://sc-cdn.scaleengine.net/i/99a7a32346291383260f435e6fa27c7e.png)

![Sample 3](https://sc-cdn.scaleengine.net/i/3f09504bdddc9767c4089006e5827899.png)
