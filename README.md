# vmod_httpstats

Exposes HTTP response code aggregates for backend and frontend responses via Varnish Statistics Counters (VSC).

## Requirements

You'll need:
- `cargo` (and the accompanying `rust` package)
- a recent `varnish` installed, with its development libraries/headers

## Build and test

``` bash
cargo build --release
cargo test --release
```

The vmod file will be found at `target/release/libvmod_httpstats.so`.

## VCL Usage

```vcl
import httpstats;

sub vcl_init {
    new s = httpstats.stats();
}

sub vcl_backend_response {
    s.record_backend(beresp.status);
}

sub vcl_deliver {
    s.record_frontend(resp.status);
}
```

## Statistics

The following counters are exposed via `varnishstat`:

### Backend (origin server responses)

| Stat Name | Description |
|-----------|-------------|
| `httpstats.backend.default.resp_2xx` | Successful responses (200-299) |
| `httpstats.backend.default.resp_3xx` | Redirect responses (300-399) |
| `httpstats.backend.default.resp_4xx` | Client error responses (400-499) |
| `httpstats.backend.default.resp_5xx` | Server error responses (500-599) |

### Frontend (client-facing responses)

| Stat Name | Description |
|-----------|-------------|
| `httpstats.frontend.default.resp_2xx` | Successful responses (200-299) |
| `httpstats.frontend.default.resp_3xx` | Redirect responses (300-399) |
| `httpstats.frontend.default.resp_4xx` | Client error responses (400-499) |
| `httpstats.frontend.default.resp_5xx` | Server error responses (500-599) |

## API Reference

See [API.md](API.md) for the full API documentation.
