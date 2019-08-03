# Rust Actix - Angular SPA PoC Example

An example demonstrating how to use Rust Actix web server to serve an Angular SPA.

### Build Prerequisite

- [Angular CLI](https://cli.angular.io/)
- [Rust and Cargo](https://rustup.rs/)

### How to Build

- Build Angular SPA

```bash
$ cd angular-ui
$ ng build --prod --deploy-url /static/ --output-path dist/
```

- Run Rust Actix Server

```bash
$ cd ..
$ cd actix-backend
$ RUST_LOG=info cargo run
```

- Navigate in browser

> http://127.0.0.1:8080

### Web Server/Angular Route Explained

#### GET /

Angular SPA entry point `index.html` served by Rust server.

#### GET /rust

A simple web page served by Rust server.

#### GET /static

Angular SPA static files (bundled JavaScript) served by Rust server.

#### Other GET routes

All other GET routes are served by Angular.

- If you navigate inside the Angular SPA: it is served directly by Angular.

- If you navigate by URL, the Rust server first redirects all unknown routes to Angular SPA, and then it is further served by Angular.

#### 404 Page Not Found

Similar to all other GET routes, it is served by Angular SPA.
