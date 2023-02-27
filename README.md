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

```bash

curl 'https://api.render.com/graphql' \
  -H 'authority: api.render.com'   -H 'accept: */*'   -H 'accept-language: en-US,en;q=0.9'   -H 'authorization: Bearer rnd_5k7jv9AD3Pv7-type: application/json'   -H 'cookie: __render=%7B%22originalReferrer%22%3A%22%22%2C%22id%22%3A%22usr-c16ine3jbvm8u5ep1jsg%22%7D; intercom-device-id-wf6otxqc=06392965-5eb5-4902-9df0-32081d0675ba; __cf_bm=vvkr2dtrLnvXt8MBnJ5BDRxNhxw3q84NAzH0l0xwKQU-1677468020-0-AZcjNeUyUHayPZgmZOe4Fe3ZhELo4wcexKWGoPF7SIATYO5ZW5y3C2K5H86TlqgrWrgIa4cnB1Xn3LPxvvLG2pc='   -H 'dnt: 1'   -H 'origin: https://dashboard.render.com'   -H 'referer: https://dashboard.render.com/'   -H 'render-request-id: 34ee5f8d-f5f5-423f-9341-62191d388fea'   -H 'sec-ch-ua: "Chromium";v="110", "Not A(Brand";v="24", "Google Chrome";v="110"'   -H 'sec-ch-ua-mobile: ?0'   -H 'sec-ch-ua-platform: "macOS"'   -H 'sec-fetch-dest: empty'   -H 'sec-fetch-mode: cors'   -H 'sec-fetch-site: same-site'   -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36'   --data-raw $'{"operationName":"servicesForOwner","variables":{"ownerId":"tea-cckh5qmn6mpvodhffrc0"},"query":"query servicesForOwner($ownerId: String\u0021) {\\n  servicesForOwner(ownerId: $ownerId) {\\n    id\\n    type\\n    userFacingType\\n    userFacingTypeSlug\\n    name\\n    slug\\n    env {\\n      ...envFields\\n      __typename\\n    }\\n    repo {\\n      ...repoFields\\n      __typename\\n    }\\n    updatedAt\\n    createdAt\\n    lastDeployedAt\\n    state\\n    suspenders\\n    owner {\\n      id\\n      __typename\\n    }\\n    maintenanceScheduledAt\\n    pendingMaintenanceBy\\n    region {\\n      id\\n      description\\n      __typename\\n    }\\n    __typename\\n  }\\n}\\n\\nfragment envFields on Env {\\n  id\\n  name\\n  language\\n  isStatic\\n  sampleBuildCommand\\n  sampleStartCommand\\n  __typename\\n}\\n\\nfragment repoFields on Repo {\\n  id\\n  provider\\n  providerId\\n  name\\n  ownerName\\n  webURL\\n  isPrivate\\n  __typename\\n}\\n"}'   --compressed

```