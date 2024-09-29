<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/render-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/render-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/render-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/libninjacom/render-rs/ci?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/render_api">
    <img src="https://img.shields.io/crates/d/render_api?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/render_api">
    <img src="https://img.shields.io/crates/v/render_api?style=flat-square" alt="Crates.io" />
</a>

</p>

This repository also contains a Render command line tool, in the `cli` directory. You can install it with `cargo install render_cli`.

Render client, generated from the OpenAPI spec.

# Usage

```rust
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let response = client
        .list_authorized_users_and_teams()
        .cursor("your cursor")
        .email("your email")
        .limit("your limit")
        .name("your name")
        .await
        .unwrap();
    println!("{:#?}", response);
}
```

This example loads configuration from environment variables, specifically:

* `RENDER_API_KEY_AUTH`



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
render_api = ".."
```

# Documentation

* [Client Library Documentation](https://docs.rs/render-api)

You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*