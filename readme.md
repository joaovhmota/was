# Write and Structure (WAS)

A simple project for me to learn and implement a database migration CLI tool in Rust.

## Requirements 

The following tools are required to install the CLI.

| Tool  | Version |
| ----- | ------- |
| cargo | 1.88.0  |
| rustc | 1.88.0  |

## Instalation Guide

Open the root folder and execute the following command:

```powershell
cargo install --path .
```

This will install the tool in the system and after that can be called using the `was` word.
Using the `--help` flag will show all the avaliable commands to use.

```powershell
was --help
```

## Simple usage

### Initialize a project

Run the following command in the terminal to generate the migrations project in the running repository.

```powershell
was init project_name
```

The generated structure will look like this:

```
📁 migrations_project
 ├─ 📁 migrations
 └─ 📄 config.json
```
