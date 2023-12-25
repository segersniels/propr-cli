# Propr

[![crates.io](https://img.shields.io/crates/v/propr.svg)](https://crates.io/crates/propr)
[![npm](https://img.shields.io/npm/v/@segersniels/propr)](https://www.npmjs.com/package/@segersniels/propr)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/segersniels/propr-cli/ci.yml)

Generate GitHub PR descriptions from the command line with the help of AI. 
`propr` aims to populate a basic PR description right from your terminal so you can focus on more important things.

<p align="center">
<img src="./resources/logo.png" width="250">

## Install

```bash
sh -c "$(curl -fsSL https://raw.githubusercontent.com/segersniels/propr-cli/master/scripts/install.sh)"
```

### Cargo

```bash
cargo install propr
```

### NPM

```bash
npm install -g @segersniels/propr
```

### Binary

Grab a binary from the [releases](https://github.com/segersniels/propr-cli/releases) page and move it into your desired bin (eg. /usr/local/bin) location.

```bash
mv propr-<os> /usr/local/bin/propr
chmod +x /usr/local/bin/propr
```

## Usage

```
Generate your PRs from the command line with AI

Usage: propr <COMMAND>

Commands:
  init      Initializes propr with a base configuration
  create    Creates a PR with a generated description
  generate  Generates a PR description and outputs it
  config    Configure propr to your liking
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Creating a PR

Creating a PR from the CLI is as easy as running `propr create`. If you want to specify which base branch to target you can provide the `--branch` flag. By default `propr` assumes that the main branch of the repository needs to be targeted.

```
Creates a PR with a generated description

Usage: propr create [OPTIONS]

Options:
  -b, --branch <branch>  The base branch to point your changes to
  -m, --model <model>    Instructs propr to use a specific model [possible values: gpt-3.5-turbo, gpt-3.5-turbo-16k, gpt-4, gpt-4-32k]
  -h, --help             Print help
```

You can regenerate a PR description at any time, even after the PR has already been created using `propr generate`.

#### Automatically generating a  title

There is the option to allow `propr` to automatically generate a PR title based on the generated description. To enable this refer to `propr config generate-title`.

#### Using an assistant

You can configure `propr` to use a custom assistant using `propr config assistant`. This may be interesting if you want to share prompts and templates between devices and/or want to control everything externally.