# xcstringstool

This tool is a reimplementation of the string catalog compiler `xcstringstool` found in Xcode.

## Overview

Apple introduced the `.xcstrings` format in Xcode 15 as a structured, JSON-based
way to manage localizations.

In Objective-C projects, `xcstringstool` compiles the `.xcstrings` file into
language-specific `.strings` and `.stringsdict` files.

## Options

```
$ xcstringstool --help
Work with .xcstrings files

Usage: xcstringstool <COMMAND>

Commands:
  print    Prints all string keys represented in an xcstrings file
  compile  Produces build products for an .xcstrings file
  sync     Updates an .xcstrings file based on .stringsdata files
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Building

Install Rust (stable), and build the project with cargo: `cargo build --release`.
