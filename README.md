# xcstringstool

This tool is a reimplementation of the string catalog compiler `xcstringstool` found in Xcode.

## Overview

Apple introduced the `.xcstrings` format in Xcode 15 as a structured, JSON-based
way to manage localizations.

In Objective-C projects, `xcstringstool` compiles the `.xcstrings` file into
language-specific `.strings` and `.stringsdict` files.
