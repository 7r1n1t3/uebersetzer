# uebersetzer

uebersetzer is a fast daemon written in Rust that manages your dotfiles' parameters according to your env, following template files you set.

## Motivation

I wanted that my dotfiles reference my env variables for a uniform style for my desktop and so that a change in the .env file results in changing all configs that use these variables.

## Features

- **Speed**
- **Universal**: works on any file in your config
- **Configurable**: edit uebersetzer.toml

## preparing ueber files

For each file containing `.ueber` under `config_path`, uebersetzer writes the rendered output to the same path with the `.ueber` extension removed.  
For example, `config.ueber.yaml` -> `config.yaml`.

1. copy config files and add ueber extension (example: `config.yaml` -> `config.yaml.ueber`)
2. set env variable placeholders as `@@var_name@@` in the new ueber file

### Note

If the target file already exists, uebersetzer skips it by default. Set `force_write = true` to replace existing target files.
If an error happens during the writing, a `bk` backup file of the target file will be found in the same path

## Configuration

Configuration is done by editing `uebersetzer.toml`. The file is searched under `XDG_CONFIG_DIRS`.

```sh
mkdir ~/.config/uebersetzer
cp uebersetzer.default.toml ~/.config/uebersetzer/uebersetzer.toml
```
