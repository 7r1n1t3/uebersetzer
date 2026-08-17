# uebersetzer

uebersetzer is a daemon written in Rust that manages your configuration files'
parameters according to your environment, using [Tera](https://keats.github.io/tera)
templates that you define.

## Motivation

I wanted my dotfiles to reference environment variables for a consistent style
across my desktop, so that a single change in my .env file propagates to every
config that uses those variables.

## Features

- **Universal**: works on any file in your config
- **Configurable**: edit uebersetzer.toml
- **Capable templating engine**: thanks to [Tera](https://keats.github.io/tera)

## preparing ueber files

For each file containing `.ueber` under `config_path`, uebersetzer writes
the rendered output to the same path with the `.ueber` extension removed.  
For example, `config.ueber.yaml` -> `config.yaml`.

1. copy config files and add ueber extension (example: `config.yaml` -> `config.yaml.ueber`)
2. set env variable placeholders as `{{ var_name }}` in the new ueber file

## Configuration

Configuration is done by editing `uebersetzer.toml`.
The file is searched under `XDG_CONFIG_DIRS`.

```sh
cp uebersetzer.default.toml ~/.config/uebersetzer.toml
```

### Note

- If the target file already exists, uebersetzer skips it by default.
Set `force_write = true` to replace existing target files.

- Set `recursive = true` to search in all files unders `config_path`
