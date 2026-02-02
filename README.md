# TRMNL Wingspan plugin recipe & server

Server: https://wingspan.capypara.de

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
