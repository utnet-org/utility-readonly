<br />
<br />

<p align="center">
<img src="docs/images/logo.svg" width="240">
</p>

<br />
<br />

## Reference implementation of Utility

![Buildkite](https://img.shields.io/buildkite/0eae07525f8e44a19b48fa937813e2c21ee04aa351361cd851)
![Stable Status][stable-release]
![Prerelease Status][prerelease]
[![codecov][codecov-badge]][codecov-url]
[![Discord chat][discord-badge]][discord-url]
[![Telegram Group][telegram-badge]][telegram-url]

[stable-release]: https://img.shields.io/github/v/release/utnet-org/utility?label=stable
[prerelease]: https://img.shields.io/github/v/release/utnet-org/utility?include_prereleases&label=prerelease
[ci-badge-master]: https://badge.buildkite.com/a81147cb62c585cc434459eedd1d25e521453120ead9ee6c64.svg?branch=master
[ci-url]: https://buildkite.com/utnet-org/utility
[codecov-badge]: https://codecov.io/gh/utnet-org/utility/branch/master/graph/badge.svg
[codecov-url]: https://codecov.io/gh/utnet-org/utility
[discord-badge]: https://img.shields.io/discord/1039485032772947978.svg
[discord-url]: https://discord.gg/x97hvFTv
[telegram-badge]: https://cdn.jsdelivr.net/gh/Patrolavia/telegram-badge@8fe3382b3fd3a1c533ba270e608035a27e430c2e/chat.svg
[telegram-url]: https://t.me/UtilityNetorg

## About Utility

Utility is a decentralized edge computing blockchain network composed of chips defined through the block chain provides resources for AI-directed computing and a variety of heterogeneous computing.

Overall, *Utility* provides a wide range of tools for developers to easily build applications:
 - [JS Client library][js-api] to connect to Utility Network from your applications.
 - [Rust][rust-sdk] and [JavaScript/TypeScript][js-sdk] SDKs to write smart contracts and stateful server-less functions.
 - [Numerous examples][examples-url] with links to hack on them right inside your browser.
 - [Lots of documentation][docs-url], with [Tutorials][tutorials-url] and [API docs][api-docs-url].

[js-api]: https://github.com/utnet-org/unc-api-js
[rust-sdk]: https://github.com/utnet-org/unc-sdk-rs
[js-sdk]: https://github.com/utnet-org/unc-sdk-js
[examples-url]: https://utnet.org
[docs-url]: https://utnet.org/developers/docs
[tutorials-url]: https://utnet.org/developers/docs
[api-docs-url]: https://utnet.org/developers/docs

## Join the Network

The easiest way to join the network, is by using the `make` command, which you can install as follows:

```bash
make release
```

You can join all the active networks:
* mainnet: `./target/debug/uncd mainnet`
* testnet: `./target/debug/uncd testnet`
* betanet: `./target/debug/uncd betanet`

Check the `utility` repository for [more details](https://github.com/utnet-org/utility/) on how to run with or without docker.

To learn how to become validator, checkout [documentation](https://utnet.org/developers/docs).

## Contributing

The workflow and details of setup to contribute are described in [CONTRIBUTING.md](CONTRIBUTING.md), and security policy is described in [SECURITY.md](SECURITY.md).
To propose new protocol changes or standards use [Specification & Standards repository](https://github.com/utnet-org/utility/).

## Getting in Touch

We use Zulip for semi-synchronous technical discussion, feel free to chime in:

https://utility.zulipchat.com/

For non-technical discussion and overall direction of the project, see our Discourse forum:

https://gov.utility.org
