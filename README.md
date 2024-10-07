<div align="center">

![Shuru Logo](shuru.svg)

# Shuru

A task runner and version manager for Node.js and Python, written in Rust!

![Version](https://img.shields.io/badge/version-0.0.17-blue)
![License](https://img.shields.io/badge/license-MIT-lightgrey)

**Join us in
[Hacktoberfest](https://github.com/shuru-project/shuru/discussions/10) and
contribute to open source!**

</div>

## Installation

### Linux and macOS

Run the following command to install the `shuru` CLI on your system:

```bash
sh -c "$(curl -fsSL https://raw.githubusercontent.com/shuru-project/shuru/main/install.sh)"
```

## Features

- Basic task runner
- Command Auto-completions for Bash, Zsh, and Fish shell
- Built-in Node Version Manager
- Built-in Python Version Manager

## Usage

1. Create a `shuru.toml` file in the root of your project to define tasks.
2. Run tasks using the following command:

```bash
shuru <COMMAND>
```

Replace `<COMMAND>` with the name of the task you've defined in your
`shuru.toml` file.

## Examples

You can explore the `examples` directory for more examples. Below is a simple
example for a Node.js project:

```toml
[versions]
node = "v16.14.0" # You can use any Node Version
# You can also specify the platform:
# node = { version = "v16.14.0", platform = "darwin-arm64" }

[tasks.setup]
command = "npm i"

[tasks.dev]
command = "npm run dev"

[tasks.build]
command = "npm run build"

[tasks.version]
command = "node --version"
```

## Join Us on Discord

Join our community on Discord to discuss, share feedback, and get support:
[https://discord.gg/EtZn7EdDdS](https://discord.gg/EtZn7EdDdS)
