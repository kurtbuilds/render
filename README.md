<div id="top"></div>

<p align="center">
<a href="https://github.com/kurtbuilds/render/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/kurtbuilds/render.svg?style=flat-square" alt="GitHub Contributors" />
</a>
<a href="https://github.com/kurtbuilds/render/stargazers">
    <img src="https://img.shields.io/github/stars/kurtbuilds/render.svg?style=flat-square" alt="Stars" />
</a>
<a href="https://github.com/kurtbuilds/render/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/kurtbuilds/render/test.yaml?style=flat-square" alt="Build Status" />
</a>
<a href="https://crates.io/crates/render-cli">
    <img src="https://img.shields.io/crates/d/render-cli?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/render-cli">
    <img src="https://img.shields.io/crates/v/render-cli?style=flat-square" alt="Crates.io" />
</a>

</p>

# Render

This is a CLI tool for Render.com actions. 

Pull requests and adoption or sponsorship by the render.com team are more than welcome.

# Usage

These commands exist:

    render put-env <service_name> <env_fpath...>
    render put-env <env_group_name> <env_fpath...>
    render create-env-group <service_name>
    render deploy <service_name>
    render list
    render suspend <service_name...>

It depends on two environment variables, which can also be passed in as flags:

    RENDER_TOKEN
    RENDER_OWNER # this is optional. Assumes your user account otherwise

For the token, generate a token on the Render.com web interface, and then add it to your environment.

I recommend against adding secrets to `~/.bash_profile`. Instead, store it in `~/.renderrc`:

    # ~/.renderrc
    export RENDER_TOKEN=<your token>

    # ~/.bash_profile
    source ~/.renderrc

# Installation

    cargo install render-cli

