# About

This repository contains a CLI tool for accessing the
[Murdock](https://github.com/murdock-ng/murdock) API.

`murdock-cli` is in very early stages. Feel free to help out!

## Installation

Check out this repo, then install via

    cargo install --path .

## Generated Openapi code

`murdock-api` has been generated with

    openapi-generator-cli generate -i openapi.json -g rust --additional-properties library=reqwest,packageName=murdock-api -o murdock-api --skip-validate-spec
