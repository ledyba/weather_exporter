# Weather Exporter

Prometheus exporter for weather. Yeah, the weathers on your earth.

## Building and running

First, please sign up [OpenWetherMap](https://home.openweathermap.org/) and get an app id. Free plan is enough.

### Manyally

```bash
cargo build --release
```

then run,

```bash
target/release/weather_exporter web \
  --listen '0.0.0.0:8080' \
  --app-id "<app id>" \
  'Tokyo,JP' 'Saitama,JP' 'Kyoto,JP' 'Osaka,JP'
```

### Using docker

Write a docker-compose.yml like:

```yaml
---
version: '3.7'

services:
  weather_exporter:
    image: weather_exporter
    build:
      context: ./
    expose:
      - '8080'
    restart: always
    command: "web --listen '0.0.0.0:8080' --app-id <app id> 'Tokyo,JP' 'Saitama,JP' 'Kyoto,JP' 'Osaka,JP'"
```

then,

```bash
docker-comopse build
docker-comopse up -d
```

## Monitoring from Prometheus

### Scraping config

```yaml
scrape_configs:
  - job_name: 'weather_exporter'
    scrape_interval: '60s'
    metrics_path: '/'
    static_configs:
      - targets:
        - 'weather_exporter:8080'
```

### Alert example

```yaml
---
groups:
  - name: Weather
    rules:
    - alert: HyperHot
      expr: weather_temp{location='Tokyo'} >= 303 # in Kelvin.
      for: 60s
      labels:
        severity: warning
      annotations:
        summary: It's too hot for humans.
        description: Take a rest. Don't run arround the Imperial Palace.
```
