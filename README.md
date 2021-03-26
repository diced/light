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

And run the executable found in `target/release/light`

# configuration

Provided there is a `light.example.toml` which you should rename to `light.toml`. All values do have [default values](#default-config-values).

# default config values

```toml
host = "0.0.0.0:8000"
postgres_uri = "postgresql://light:light@postgres/light"
admin_key = "1234"
uploads_dir = "./uploads"
uploads_route = "/i"
token_length = 12
file_length = 5
```
