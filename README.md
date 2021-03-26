# light

a light file server made in rust

# feature set

- users
- fully customizable
- fast
- low memory footprint

# running

Quick setup with docker-compose

```sh
docker-compose up --build -d
```

# building

Build `light` by just running

```sh
cargo build --release
```

# prebuilt binaries

Prebuilt binaries can be found in GitHub Releases. Only for linux.

And run the executable found in `target/release/light`

# configuration

Provided there is a `light.example.toml` which you should rename to `light.toml`. All values do have [default values](#default-config-values).

# default config values

These default values should be perfect to run in docker as you only need to run one command (docker compose) and you will have it running.

```toml
host = "0.0.0.0:8000"
postgres_uri = "postgresql://light:light@postgres/light"
admin_key = "1234"
uploads_dir = "./uploads"
uploads_route = "/i"
token_length = 12
file_length = 5
```
