# Rust SpaceAPI Implementation of dezentrale

This is an implementation of the [SpaceAPI](https://spaceapi.io/) v14 in Rust. It contains following parts

- `spaceapi-dezentrale`: Serialization and deserialization to/from JSON using Serde
- `spaceapi-dezentrale-client`: Client to access the server via API
- `spaceapi-dezentrale-server`: Server which provides the API

Supported architectures with static linked binaries with [*musl libc*](https://musl.libc.org/):

* x86-64
* Aarch64
* ARMv6 + FPU
* ARMv7 + FPU

## Build

```
cargo build --release
```

## Usage

### Server

Create a config file (see `config.sample.yml`) which describes some basic information about the space, see [below](#Configuration).

Start the server.

```
CONFIG_FILE=config.sample.yml RUST_LOG=INFO \
    spaceapi-dezentrale-server
```

#### Configuration file

The `publish` section is a representation of the [`Status` struct of the SpaceAPI](https://spaceapi.io/docs/), which will be used as a template for publishing the status.

The server doesn't use much custom logic. See [Rocket documentation](https://rocket.rs/v0.5-rc/guide/configuration/#configuration) how to change parts like ports, limits, etc.

The log level can be changed with the default mechanism of [`RUST_LOG` of `env_logger`](https://docs.rs/env_logger/0.10.0/env_logger/#enabling-logging).

### Client

Open the space

```
SPACEAPI_URL=http://localhost:8000 API_KEY=not-very-secure \
    spaceapi-dezentrale-client open
```

Close the space

```
SPACEAPI_URL=http://localhost:8000 API_KEY=not-very-secure \
    spaceapi-dezentrale-client close
```

Check if the space is open

```
SPACEAPI_URL=http://localhost:8000 API_KEY=not-very-secure \
    spaceapi-dezentrale-client is-open
```

Keep open call

```
SPACEAPI_URL=http://localhost:8000 API_KEY=not-very-secure \
    spaceapi-dezentrale-client keep-open
```

### Containter images

Due the support for static linked binaries the container images are based on the `scratch` image so the image only contains the binary.

To build the images for *x86-64*:

```sh
# Build the server image
docker build -t spaceapi-dezentrale-server .

# Build the client image
docker build \
    --build-arg PROJECT=spaceapi-dezentrale-client \
    --build-arg BINARY=target/x86_64-unknown-linux-musl/release/spaceapi-dezentrale-client \
    --tag spaceapi-dezentrale-client .
```

To build other configurations and architectures take a look into [Dockerfile](Dockerfile) and the [GitHub release workflow](.github/workflows/release.yml).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
