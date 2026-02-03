# TRMNL Wingspan plugin recipe & server

A TRMNL plugin to display bird cards from the game Wingspan.

Powered by [Wingsearch](https://github.com/navarog/wingsearch).

| todo: preview image and link will go here

## Fork with your own images

To fork this to host your own server, with your own images:

1. Clone this repo
2. Change the api/config.toml

My personal `config.toml` looks like this:

```toml
wingsearch = "../wingsearch" # Point this to the wingsearch directory. If you clone this repo with submodules, keep as-is
default_images = "default"
new_bird_interval = 3600 # How often to generate new birds. Feel free to set this as low as you want. This is 1h.

[images.default]
# Set to a path where with all the blurred bird images from Wingsearch, but with the padding around the outlines
# removed
path = "../blurred_bird_images"

[images.full]
# My personal collection of full color bird images, protected with a password. Falls back to `default`.
path = "../bird_images"
token = "..."
```

3. Run the API server. The easiest way to do this is to use Docker via the Dockerfile & docker-compose.yml in `api/`
    - It listens on port 8080
4. Host the API server somewhere public
5. Configure the plugin to use your API server
6. If you use your own image set, enter the name and the token. In the example above, the image set would be `full` and
   the token `...`.

## Server

Public server: https://wingspan.capypara.de - this only has one image set publicly available, which serves blurred
images.

Routes:

- `GET /api/current?locale=<locale>&allowed_set=<set>` - current bird -
    - `locale` is optional and defaults to `en`. Can be: `en,de,dk,es,fr,jp,lt,nl,pl,pt,tr,uk`.
    - `allowed_set` is an optional, repeatable argument, if any is given, the cards are filtered
      to be only from these sets.
- `GET /images/<image set>/<id>.png` - get bird image
    - `GET /images/_/<id>.png`: uses default image set
    - some sets may need token authentication to use (`?token=<token>`)
    - image sets configurable, see `api/config.toml`
- `GET /icons/<icon_name>.png`
    - Icon names, see https://github.com/navarog/wingsearch/tree/master/src/assets/icons/png
- `GET /fonts/<font>`
    - Font names, see https://github.com/navarog/wingsearch/tree/master/src/assets/fonts
