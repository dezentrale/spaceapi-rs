# Changelog

## v0.10.0

* Refactoring of state handling
* Refactored module structure
* Moved API-Key checking into guard implementation
* Template for publishing state is handled as extra state
* Added CORS handling
* Added a minimalistic index page
* Added support for TLS for client
* Added text output on path `/status/text`
* Added html output on path `/status/html`
* Added configuration for text/html outputs
* Added static build configuration for ARMv6, ARMv7, AArch64, x86-64 for libmusl
* Added container builds

## v0.9

* Restructured project as single repository with api, server and client crate
* Added a client implementation based on reqwest
* Added a server implementation based on rocket

## v0.8.999

* Forked of [spaceapi-rs Project](https://github.com/spaceapi-community/spaceapi-rs/)
