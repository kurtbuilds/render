<div id="top"></div>

<p align="center">
<a href="https://github.com/kurtbuilds/render/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/kurtbuilds/render.svg?style=flat-square" alt="GitHub Contributors" />
</a>
<a href="https://github.com/kurtbuilds/render/stargazers">
    <img src="https://img.shields.io/github/stars/kurtbuilds/render.svg?style=flat-square" alt="Stars" />
</a>
<a href="https://github.com/kurtbuilds/render/actions">
    <img src="https://img.shields.io/github/workflow/status/kurtbuilds/render/test?style=flat-square" alt="Build Status" />
</a>
<a href="https://crates.io/crates/render-cli">
    <img src="https://img.shields.io/crates/d/render-cli?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/render-cli">
    <img src="https://img.shields.io/crates/v/render-cli?style=flat-square" alt="Crates.io" />
</a>

</p>

# Render

This is a CLI tool for Render.com actions. Right now it only supports triggering deploys (useful for monorepos) and
updating a service environment environment.

Pull Requests and formal adoption by the render.com team are more than welcome and will be responded to fairly quickly.

# Usage

These are the only two commands currently:

    render put-env <service_name> <env_fpath...>
    render deploy <service_name>

It authorizes your requests by looking for `RENDER_TOKEN` in your environment. 
Generate a token on the Render.com web interface, and then add it to your environment.

You can add it to your environment like below. 
I recommend against adding secrets to `~/.bash_profile`, so it's stored in `~/.renderrc`.

    # ~/.renderrc
    export RENDER_TOKEN=<your token>

    # ~/.bash_profile
    source ~/.renderrc

The rest of the documentation is available from `--help`:

    render-cli 0.1.0

    Kurt Wolf <kurtwolfbuilds@gmail.com>

    Command line actions for Render.com

    USAGE:
    render [OPTIONS] <SUBCOMMAND>

    OPTIONS:
    -h, --help             Print help information
    --token <TOKEN>    The API key. Can be set with env var RENDER_TOKEN
    -V, --version          Print version information

    SUBCOMMANDS:
    deploy
    help       Print this message or the help of the given subcommand(s)
    put-env    Update the service environment variables 
      
# Installation

    cargo install render-cli

Alternatively, install from source:

    git clone https://github.com/kurtbuilds/render
    cd render
    just install

# Contributing

Bug reports are very welcome, as are pull requests for new features. Requests for new features will likely be ignored.

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request
