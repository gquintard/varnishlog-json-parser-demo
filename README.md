# JSON isn't dirty, it just feels that way

This a demo tool showing how you can easily deserialize the output of [varnishlog-json](https://github.com/varnish/varnishlog-json/blob/main/README.md) (meaning you should probably head over there and install it first).

# Build

You just need `cargo` and `rust`:

``` shell
cargo build
```

# Run

Just pipe `varnishlog-json`'s output into `varnishlog-json-parser-demo`:

``` shell
/path/to/varnishlog-json -d | target/debug/varnishlog-json-parser-demo
503 GET /admin/index.html
200 GET /
...
```
