# Weather Exporter

 [![Build on Linux](https://github.com/ledyba/weather_exporter/workflows/Build%20on%20Linux/badge.svg)](https://github.com/ledyba/weather_exporter/actions?query=workflow%3A%22Build+on+Linux%22)
[![Build on macOS](https://github.com/ledyba/weather_exporter/workflows/Build%20on%20macOS/badge.svg)](https://github.com/ledyba/weather_exporter/actions?query=workflow%3A%22Build+on+macOS%22)
[![Build on Windows](https://github.com/ledyba/weather_exporter/workflows/Build%20on%20Windows/badge.svg)](https://github.com/ledyba/weather_exporter/actions?query=workflow%3A%22Build+on+Windows%22)  
[![Build single binary on Linux](https://github.com/ledyba/weather_exporter/workflows/Build%20single%20binary%20on%20Linux/badge.svg)](https://github.com/ledyba/weather_exporter/actions?query=workflow%3A%22Build+single+binary+on+Linux%22)

Prometheus exporter for weather. Yeah, the weathers on your earth.

## Building and running

First, please sign up [OpenWetherMap](https://home.openweathermap.org/) and get an app id. Free plan is enough.

### with Cargo

```bash
cargo build --release
```

then run,

```bash
target/release/weather_exporter web \
  --listen '0.0.0.0:8080' \
  --app-id "<app id>" # app-id is optional.
```

### with Docker

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
    command: 
      - 'web'
      - '--listen'
      - '0.0.0.0:8080'
      # app-id is optional
      - '--app-id'
      - '<app id>'
```

then,

```bash
docker-comopse build # It takes long time. Be patient....
docker-comopse up -d
```

## Check from browsers

Please access to `http://localhost:8080/probe?app-id=<app-id>&target=Tokyo,JP`.

`app-id` is not required if you set `--app-id <app-id>` as command line flag.

## Monitoring from Prometheus

### Scraping config

```yaml
  - job_name: 'weather_exporter'
    scrape_interval: '60s'
    metrics_path: '/probe'
    relabel_configs:
      - source_labels: [__address__]
        target_label: __param_target
      - source_labels: [__param_target]
        target_label: instance
      - target_label: __address__
        replacement: 'weather_exporter:8080'
    params:
      app-id: '6f2bbbdbb606f96a81a30961e8cc0d61'
    static_configs:
      - targets:
          - 'Tokyo,JP'
          - 'Saitama,JP'
          - 'Kyoto,JP'
          - 'Osaka,JP'
```

### Alert example

```yaml
---
groups:
  - name: Weather
    rules:
    - alert: HyperHot
      expr: weather_air_temp{location='Tokyo'} >= 308 # in Kelvin.
      for: 60s
      labels:
        severity: 'critical'
      annotations:
        summary: It's too hot for humans.
        description: Stay home. Keep the air conditioner on.
    - alert: HyperHot
      expr: weather_air_temp{location='Tokyo'} >= 303 # in Kelvin.
      for: 60s
      labels:
        severity: 'warning'
      annotations:
        summary: It's too hot to do exercises.
        description: Take a rest. Don't run arround the Imperial Palace.
```
