# pix

A CLI for managing NFT projects

**Table of Contents**

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Config](#config)
  - [Example](#example)
  - [Types](#types)

## Features

- Generate unique NFTs from attribute files
- Ordering defined in the config
- Integrates with [nft maker](https://nft-maker.io)
  - generate metadata template
  - upload collections

## Installation

For now pix needs to be built from source.

- install [rustup.rs](https://rustup.rs)
- `git clone https://github.com/3based/pix.git`
- `cd pix`
- `cargo install --path .`

> If you decide to make code changes to customize it to your needs you'll have to re-run `cargo install --path .`

## Usage

```
USAGE:
    pix <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    auth        Provide your NFT Maker API Key to use globally
    clean       Clean the output directory
    gen         Generate an NFT collection
    help        Print this message or the help of the given subcommand(s)
    metadata    Output metadata template that can be uploaded to nft-maker.io
    new         Create a new project
    upload      Upload an NFT collection to nft-maker.io
```

## Config

There needs to be a config file at the root of a project.

### Example

```json
{
  "policy_id": "123",
  "name": "BasedBear",
  "display_name": "Based Bear",
  "twitter": "https://twitter.com/_3based",
  "website": "https://3based.com",
  "copyright": "2022 3Based",
  "mode": "simple",
  "start_at_one": false,
  "amount": 10000,
  "tolerance": 50,
  "path": "images",
  "layers": [
    { "name": "background" },
    { "name": "eyes" },
    { "name": "Base" },
    { "name": "Stitch Color" },
    { "name": "belly", "none": 80 },
    { "name": "forehead", "none": 60 },
    { "name": "Stuffing" }
  ],
  "nft_maker": {
    "apikey": "",
    "nft_project_id": 0
  }
}
```

### Types

```
{
    policy_id?: string,
    name: string,
    display_name?: string,
    twitter?: string,
    website?: string,
    copyright?: string,
    mode: "simple" | "advanced",
    start_at_one?: true,
    amount: integer,
    tolerance: integer,
    path: string,
    layers: { name: string, none?: integer }[],
    nft_maker?: {
        apikey: string,
        nft_project_id: integer
    }
}
```
