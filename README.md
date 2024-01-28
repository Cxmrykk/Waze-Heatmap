### Dependencies
- [NodeJS](https://nodejs.org/en) and [NPM](https://nodejs.org/en) (Latest)
- [Rust](https://www.rust-lang.org/) and [Cargo](https://www.rust-lang.org/)

Server (Rust/Cargo)
  - [Diesel](https://diesel.rs/guides/getting-started.html) (Dependencies required, see below)

> By default diesel CLI depends on the following client libraries:
>- **libpq** for the PostgreSQL backend
>- **libmysqlclient** for the Mysql backend
>- **libsqlite3** for the SQLite backend


### Setup
```sh
# Clone the project directory
git clone https://github.com/Cxmrykk/Waze-Heatmap.git
cd Waze-Heatmap/

# Run Diesel setup (Make sure cargo is in $PATH for Diesel to work)
cd server/
cargo build
diesel setup

# Modify Config.toml (Alternative editors: nano, code, etc)
vim Config.toml

# Run the server in a separate process (E.g. using screen)
# Note that the server doesn't need to be running in the background
# for the Web GUI to work, but it does need to be executed at least once
cargo run

# Run the Web GUI
cd ../client
node server.js
```
