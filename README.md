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

# slog-term  - Unix terminal drain for [slog-rs]

[slog-rs]: //github.com/slog-rs/slog

## Development

### Running integration test suite in docker

Run testsuite on live syslog instances

```
docker-compose -f docker/docker-compose.yml up --abort-on-container-exit
```

Parse syslog-ng messages.json for all messages

```
docker-compose -f docker/docker-compose.yml run rust cat /syslog-ng/messages.json | jq --slurp '.[] | .MESSAGE'
```
