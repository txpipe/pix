# pix

A CLI for managing NFT projects

**Table of Contents**

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Simple Mode](#simple-mode)
  - [Advanced Mode](#advanced-mode)
- [Config](#config)
  - [Example](#example)
  - [Types](#types)

## Features

- Generate unique NFTs from attribute files
- Layer ordering defined in the config
- Output rarity data
- Sets (groups of the same layers with different image files)
- Conditional Layer Rendering (based on sets or traits within a previous layer)
- Starting count at 1 or 0
- Simple or Advanced rarity configurations
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

### Simple Mode

In simple mode, you have some base folder for your images defaulted to `images/`. Then in there you have a folder for each layer who's names match the layers provided in the `pix.json` file.

Then within the layer folders you need to have your trait files organized into **rarity folders**.

```
images/
|__ambience/
     |__common/
     |    |__trait.png
     |__uncommon/
     |__rare/
     |__mythical/
     |__legendary/
```

These have their numeric weights defaulted like so:

- common: 70
- uncommon: 50
- rare: 20
- mythical: 10
- legendary: 5

### Advanced Mode

This mode works much more like what people are used to in hashlips. In advanced mode, you have some base folder for your images defaulted to `images/`. Then in there you have a folder for each layer who's names match the layers provided in the `pix.json` file.

Unlike simple mode, the trait files live in the layer folders and **NOT** within rarity folders. From there you must provide the rarity with a special suffix in the trait file names.

This is the pattern: `name#WEIGHT.png`

```
images/
|__ambience/
     |__trait#30.png
```

> how dos the pix.json tolerance margin works?

This is a number that the tool uses to decide when to stop trying to make unique combinations. The program essentially loops continuously trying to make as many combinations as specified in the `pix.json` file and stops looping when that amount is reached or when the failure tolerance is reached. Without the tolerance number the program could potentially loop infinitely.

## Config

There needs to be a config file at the root of a project.

### Example

```json
{
  "policy_id": "123",
  "name": "BasedBear",
  "display_name": "Based Bear",
  "mode": "simple",
  "start_at_one": false,
  "amount": 10000,
  "tolerance": 50,
  "path": "images",
  "sets": [
    {
      "name": "Head",
      "amount": 20
    },
    {
      "name": "Mohawk",
      "amount": 20
    }
  ],
  "layers": [
    { "name": "background" },
    {
      "name": "eyes",
      "exclude_if_sets": ["Mohawk"],
      "exclude_if_traits": [
        { "layer": "background", "traits": ["clouds", "cardano"] }
      ]
    },
    { "name": "Base" },
    { "name": "Stitch Color" },
    { "name": "belly", "none": 80 },
    { "name": "forehead", "none": 60 },
    { "name": "Stuffing" }
  ],
  "extra": {
    "twitter": "https://twitter.com/_3based",
    "website": "https://3based.com",
    "copyright": "2022 3Based",
  },
  "nft_maker": {
    "network": "mainnet",
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
    mode: "simple" | "advanced",
    start_at_one?: true,
    amount: integer,
    tolerance: integer,
    path: string,
    sets?: { name: string, amount: integer }[],
    layers: {
      name: string,
      none?: integer,
      exclude_if_sets?: string[],
      exclude_if_traits?: {
        layer: string,
        traits: string[]
      }[]
    }[],
    extra: Json,
    nft_maker?: {
        network: string,
        apikey: string,
        nft_project_id: integer
    }
}
```
