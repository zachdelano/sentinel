# Sentinel

## Dependencies

Built on Windows

* [Install SQLite3](https://gist.github.com/zeljic/d8b542788b225b1bcb5fce169ee28c55)
* [Install GStreamer](https://stackoverflow.com/questions/63026758/cannot-compile-gstreamer-on-windows-because-it-is-missing-glib-2-0)
* [Diesel CLI](https://lib.rs/crates/diesel_cli)
  - Make sure `sqlite3.lib` is in the directory where you are building the Diesel CLI.
  - `cargo install diesel_cli --no-default-features --features"sqlite-bundled"`

## Set up SQLite with Diesel

* [Guide](https://erwabook.com/intro/set-up-orm-database.html)