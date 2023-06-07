## Linux / WSL2 Pre-install Instructions

See [eframe docs](https://github.com/emilk/egui/tree/master/crates/eframe).

```sh
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

## Project Setup

Copied stuff from [`eframe` project Template](https://github.com/emilk/eframe_template/).

## Running

To run as a native app:

```sh
cargo run --bin search_viz --release
```

To run as a web app:

```sh
(cd search_viz && trunk serve --port 8083 --release)
```
