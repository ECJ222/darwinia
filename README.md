<div align="center">

![Logo](https://infura-ipfs.io/ipfs/QmWm8Fdvjnu1afHGiyXQusGrBhTdZRyviNJNa6Dyx7Ujud)

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Substrate version](https://img.shields.io/badge/Substrate-3.0.0-brightgreen?logo=Parity%20Substrate)](https://substrate.io)
[![CI](https://github.com/darwinia-network/darwinia/workflows/CI/badge.svg?branch=master)](https://github.com/darwinia-network/darwinia/actions/workflows/ci.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/darwinia-network/darwinia)](https://github.com/darwinia-network/darwinia/tags)
[![Quay.io](https://img.shields.io/badge/quay-latest-blue.svg?logo=docker&logoColor=white)](https://quay.io/repository/darwinia-network/darwinia)
[![GitHub code lines](https://tokei.rs/b1/github/darwinia-network/darwinia)](https://github.com/darwinia-network/darwinia)
[![GitHub last commit](https://img.shields.io/github/last-commit/darwinia-network/darwinia?color=red&style=plastic)](https://github.com/darwinia-network/darwinia)

[![Twitter URL](https://img.shields.io/twitter/follow/DarwiniaNetwork?style=social)](https://twitter.com/DarwiniaNetwork)
[![Telegram](https://img.shields.io/endpoint?color=neon&style=flat-square&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2FDarwiniaNetwork)](https://t.me/DarwiniaOfficial)
[![Medium](https://badgen.net/badge/icon/medium?icon=medium&label)](https://darwinianetwork.medium.com)
[![Discord](https://img.shields.io/badge/Discord-gray?logo=discord)](https://discord.gg/uqa3snSGTj)

</div>

## Introduction
Implementation of a **https://darwinia.network** node in **Rust** based on the **Substrate** framework.

This repository contains runtimes for the **[Darwinia](https://darwinia.network)** and **[Crab](https://crab.network)** networks.

## Resources

### Ecosystem
|                                    Project                                     |                                        Introduction                                         |                                                                       Code                                                                       |                                                                                         Activity                                                                                         |
| :----------------------------------------------------------------------------: | :-----------------------------------------------------------------------------------------: | :----------------------------------------------------------------------------------------------------------------------------------------------: | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
|     [Darwinia Common](https://github.com/darwinia-network/darwinia-common)     | Darwinia bridge core protocol implementation as runtime pallet and Pangolin/Pangoro testnet |     [![GitHub code lines](https://tokei.rs/b1/github/darwinia-network/darwinia-common)](https://github.com/darwinia-network/darwinia-common)     |     [![GitHub last commit](https://img.shields.io/github/last-commit/darwinia-network/darwinia-common?color=red&style=plastic)](https://github.com/darwinia-network/darwinia-common)     |
|  [Darwinia Parachain](https://github.com/darwinia-network/darwinia-parachain)  |                           Darwinia's parachain on Polkadot/Kusama                           |  [![GitHub code lines](https://tokei.rs/b1/github/darwinia-network/darwinia-parachain)](https://github.com/darwinia-network/darwinia-parachain)  |  [![GitHub last commit](https://img.shields.io/github/last-commit/darwinia-network/darwinia-parachain?color=red&style=plastic)](https://github.com/darwinia-network/darwinia-parachain)  |
| [Darwinia Bridge Sol](https://github.com/darwinia-network/darwinia-bridge-sol) |                          Darwinia bridge solidity smart contracts                           | [![GitHub code lines](https://tokei.rs/b1/github/darwinia-network/darwinia-bridge-sol)](https://github.com/darwinia-network/darwinia-bridge-sol) | [![GitHub last commit](https://img.shields.io/github/last-commit/darwinia-network/darwinia-bridge-sol?color=red&style=plastic)](https://github.com/darwinia-network/darwinia-bridge-sol) |
|             [Bridger](https://github.com/darwinia-network/bridger)             |        Client for header relayers and message relayers in Darwinia's bridge network         |             [![GitHub code lines](https://tokei.rs/b1/github/darwinia-network/bridger)](https://github.com/darwinia-network/bridger)             |             [![GitHub last commit](https://img.shields.io/github/last-commit/darwinia-network/bridger?color=red&style=plastic)](https://github.com/darwinia-network/bridger)             |
|           [Smart App](https://github.com/darwinia-network/smart-app)           | DVM, smart contract, and transfer RING/KTON between Substrate address and Ethereum address  |           [![GitHub code lines](https://tokei.rs/b1/github/darwinia-network/smart-app)](https://github.com/darwinia-network/smart-app)           |           [![GitHub last commit](https://img.shields.io/github/last-commit/darwinia-network/smart-app?color=red&style=plastic)](https://github.com/darwinia-network/smart-app)           |

### Documents
- [Darwinia Network Docs](https://docs.darwinia.network)
- [Crab Network Docs](https://docs.crab.network)

### Technical Support
- Telegram
	- [Official Technical Group](https://t.me/DarwiniaDev)
- Matrix
	- [Official Technical Room](https://matrix.to/#/#darwinia:matrix.org)

## Installation
- Downloading pre-built binary from **[releases](https://github.com/darwinia-network/darwinia/releases)** page.
- Using the docker image on **[releases](https://github.com/darwinia-network/darwinia/releases)** page.
- Building from source follow this **[guide](#build-from-source)**.

## Building
> Make sure that you have all the required dependencies.
>
> Follow [substrate-getting-started](https://substrate.dev/docs/en/knowledgebase/getting-started).

### Installing via Cargo
```sh
cargo install --git https://github.com/darwinia-network/darwinia --tag <version> --locked
```

### Building from Source
```sh
# with github-cli
gh repo clone darwinia-network/darwinia
# with git
git clone https://github.com/darwinia-network/darwinia.git
git checkout <version>
cargo build --release --locked
```

## Networks
This repository supports runtimes for Darwinia and Crab.

### Connecting to Darwinia Mainnet
Connecting to the global Darwinia network by running:
```sh
./darwinia --chain darwinia
```
You can see your node on [telemetry](https://telemetry.polkadot.io/#list/0x729cb8f2cf428adcf81fe69610edda32c5711b2ff17de747e8604a3587021db8) (set a custom name with --name "my custom name").

### Connecting to Crab Canary Network
Connecting to the global Crab Canary Network by running:
```sh
./darwinia --chain crab
```
You can see your node on [telemetry](https://telemetry.polkadot.io/#list/0x34f61bfda344b3fad3c3e38832a91448b3c613b199eb23e5110a635d71c13c65) (set a custom name with --name "my custom name").

## Contributing

### Roadmap
[Roadmap](https://itering.notion.site/9617e154ec884b07a7cee9a056374e42?v=0c3e4d9f257646c486a32a0425ee3a93)

### Contributing Guidelines
[Contributing Guidelines](docs/CONTRIBUTING.adoc)

### Contributor Code of Conduct
[Code of Conduct](docs/CODE_OF_CONDUCT.md)

### Security
[Security](docs/SECURITY.md)
