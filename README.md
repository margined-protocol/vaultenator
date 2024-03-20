# Vaultenator

<!-- dprint-ignore-start -->
[![GitHub Actions CI Workflow Status][1]][2]
[![Crates.io Version][3]][4]
<!-- dprint-ignore-end -->

An opinionated base vault implementing CosmWasm Vault Standard using
Tokenfactory.

## Features

- Administration - Handles opening and pausing contracts
- Configuration - Manages contract configuration
- Ownership - Manages contract ownership
- State - Manages contract state
- CW4626 - CosmWasm Vault Standard compliant interface
- Tokenfactory - Tokenfactory denoms as vault share tokens

## Usage

Use the crate in your CosmWasm project

```sh
cargo add vaultenator
```

Implement traits or include default implementations. See the [Example Vault][5]
for an exmaple implementation using Vaulteantor.

[1]: https://img.shields.io/github/actions/workflow/status/margined-protocol/vaultenator/ci.yml?style=for-the-badge
[2]: https://github.com/margined-protocol/vaultenator/actions/workflows/ci.yml
[3]: https://img.shields.io/crates/v/vaultenator?style=for-the-badge
[4]: https://crates.io/crates/vaultenator
[5]: https://github.com/margined-protocol/example-vault
