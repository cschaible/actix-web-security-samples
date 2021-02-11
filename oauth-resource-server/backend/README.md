## OAuth2 Resource Server

### Dependencies

* The sample application uses the `jwk-default-loader` feature of `actix-web-security` which uses the `reqwest` lib which required `openssl` to be installed on the system.  
  The documentation about how to install it can be found [here](https://docs.rs/openssl/0.10.32/openssl/#automatic).
* Docker and Docker-Compose

  
### Build

Debug-Build
```bash
cargo build
```

Release-Build
```bash
cargo build --release
```

### Run

Start the database and Keycloak server by running the [up.sh](../docker/up.sh) script.  
This starts and initializes the app database and the Keycloak Authentication Provider.

Start the application by running (as debug build):
```bash
cargo run
```

The application automatically runs the database migrations on startup if not already applied.

### Stop the database and Keycloak

The database and the Keycloak server by running the [down.sh](../docker/down.sh) script.