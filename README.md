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

# creating a user

Make a **POST** request to /api/create_user with a JSON body of the following contents below. You will need to also include an `Authorization` header with your `admin_key` from the config.

```json
{
  "username": "asdf"
}
```

If the user already exists, then you will receieve an error.
If the user doesn't exist, then you will receieve your authentication token. Remember to save this, as of now there is no way to regen a token.

# uploading a file

You can upload a file by using `multipart/form-data`, an example using cURL:

```sh
curl -H "Content-Type: multipart/form-data" -H "authorization: user token" -F file=@"pog.png" "localhost:8000/upload"
```

# deleting a file

You can delete an image by sending the `DELETE` method instead to the file route, usually being `/i/{file}`

# where are files?

Files are located at `/i/{file}` by default, yet you can change `/i` to anything you want in the config.

# api routes

`/upload` -> POST `Content-Type: multipart/form-data`
`/api/create_user` -> POST `Content-Type: application/json`
`/{upload view route}/{file}` -> GET
`/{upload view route}/{file}` -> DELETE
