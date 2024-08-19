[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)](https://stand-with-ukraine.pp.ua)

# tanque-website

small web-app for my personal website, written in Rust, powered by [tokio](https://github.com/tokio-rs/tokio) runtime, [axum](https://github.com/tokio-rs/axum) webframework, [maud](https://github.com/lambda-fairy/maud) templating macro and [htmx](https://github.com/bigskysoftware/htmx), deployed using [Docker](https://www.docker.com/)

crate structure plus some other ideas & patterns have been heavily inspired by:

- [Axum-Rust-Api-Template](https://github.com/thanipro/Axum-Rust-Rest-Api-Template/) - crate structure and modules organization

- [A Practical Guide To Containerize Your Rust Application With Docker](https://itnext.io/a-practical-guide-to-containerize-your-rust-application-with-docker-77e8a391b4a8) - basics of Rust containerization

- [Rust Dockerfile Boilerplate](https://peterprototypes.com/blog/rust-dockerfile-boilerplate/) - lightweight multi-stage Docker builds

- [rust-axum-sqlx-htmx-boilerplate](https://github.com/scottjmaddox/rust-axum-sqlx-htmx-boilerplate/) repo - simple boilerplate using both [sqlx](https://github.com/launchbadge/sqlx) and [htmx](https://github.com/bigskysoftware/htmx)



## Usage/Examples

clone this repo and `cd` into it and use `cargo run` to run it

```bash
RUST_ENV=debug cargo run
```

and `cargo test` to run tests

```bash
RUST_ENV=testing cargo test
```

### Using `Makefile`

alternatively, if on Linux, you can use `make` to run it

```bash
make run
```

and to test it

```bash
make test
```

### Hot-realoading

you can watch for changes live as you develop

```bash
RUST_ENV=development cargo watch -x run
```

and for tests also

```bash
RUST_ENV=testing cargo watch -x test
```

or using `Makefile`

```bash
make watch
```

and

```bash
make watch-test
```

## License

[GPL-3.0](https://choosealicense.com/licenses/gpl-3.0/)

## Authors

written with rusty 💛️💙️ by Tanque

- [@TheShooter89](https://www.github.com/TheShooter89)

## `tanque` Stands With Ukraine 🇺🇦️

    "Freedom doesn't come cheap"

`tanque` stands with people of Ukraine in their fight against the brutal russian aggression and unrightful occupation of their homeland

`tanque` stands with people of Ukraine in their fight for **Freedom**, for **Peace**, for **Self-Determination**, for **Happiness**

`tanque` stands with this generation of young ukrainians robbed away of their youth by the war, who will have to find the strength to get up once again and rebuild from the rubbles

####

_By your side, for as long as it takes_ 💪️

        Slava Ukraini 🇺🇦️

### Donate

Please contribute and donate through official government channels or globally-know remarkable institutions:

- **UNITED24**: Institutinal fundraising, charity and media platform of Ukrainian Government. It's possible to donate for food, medicine, medical assistance, refugees support and more

  [U24 official site](https://u24.gov.ua/)

- **Medecins Sans Frontieres**: Life-saving medical assistance both in war and peace time, all over the world

  [MSF official site](https://www.msf.org/ukraine)

- **Protect A Volunteer**: Independent matching platform to support a Volunteer on the frontline
  [Protect a Volunteer site](https://protectavolunteer.com/)

Or use below badge:

[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)](https://stand-with-ukraine.pp.ua)

---

    humans die, but IDEAS are bulletproof
    🇺🇦️ ️🇪🇺️ 🏳️‍🌈️
