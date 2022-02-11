# pix

A CLI for managing NFT projects

## Features

- Generate unique NFTs from attribute files
- Ordering defined in the config
- Integrates with <a href="https://nft-maker.io" target="_blank">nft maker</a>
  - generate metadata template
  - upload collections

## Usage

```shell
USAGE:
    pix <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    clean       Clean the output directory
    gen         Generate an NFT collection
    help        Print this message or the help of the given subcommand(s)
    metadata    Output metadata template that can be uploaded to nft-maker.io
    new         Create a new project
    upload      Upload an NFT collection to nft-maker.io
```

## Config

There needs to be a config file at the root of a project.

## Example

```json
{
  "name": "BasedBears",
  "twitter": "https://twitter.com/_3based",
  "website": "https://3based.com",
  "copyright": "2022 3Based",
  "mode": "simple",
  "amount": 10000,
  "tolerance": 50,
  "path": "images",
  "attributes": [
    "background",
    "eyes",
    "Base",
    "Stitch Color",
    "belly",
    "forehead",
    "Stuffing"
  ],
  "nft_maker": {
    "apikey": "",
    "nft_project_id": 0
  }
}
```

## Types

```
{
    name: string,
    twitter?: string,
    website?: string,
    copyright?: string,
    mode: "simple" | "advanced",
    amount: integer,
    tolerance: integer,
    path: string,
    attributes: string[],
    nft_maker?: {
        apikey: string,
        nft_project_id: integer
    }
}
```