<p align="center">

  <a href="https://github.com/slog-rs/slog">
  <img src="https://cdn.rawgit.com/slog-rs/misc/master/media/slog.svg" alt="slog-rs logo">
  </a>
  <br>

  <a href="https://travis-ci.org/slog-rs/term">
      <img src="https://img.shields.io/travis/slog-rs/term/master.svg" alt="Travis CI Build Status">
  </a>

  <a href="https://crates.io/crates/slog-term">
      <img src="https://img.shields.io/crates/d/slog-term.svg" alt="slog-term on crates.io">
  </a>

  <a href="https://gitter.im/slog-rs/slog">
      <img src="https://img.shields.io/gitter/room/slog-rs/slog.svg" alt="slog-rs Gitter Chat">
  </a>
</p>

# syslog-ng  - Syslog drain for [slog-rs](http://github.com/slog-rs/slog)

## Development

### Running integration test suite in docker

All the Docker files are under `docker/` directory

```
cd docker
```

Full clean integration test run (reubilds all the crates),
display parsed JSON syslog output

```
docker-compose run rust
```

Full clean run

```
docker-compose  up --abort-on-container-exit
```

Run testsuite and inspect parsed JSON output

```
docker-compose run rust test-inspect
```

Inspect failed tests with backtraces (doesn't rebuild dependencies)

```
docker-compose run rust test-debug
```
