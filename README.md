# About

This repository contains a CLI tool for accessing the
[Murdock](https://github.com/murdock-ng/murdock) API.

`murdock-cli` is in very early stages. Feel free to help out!

## Supported API calls

Currently, only the "delete jobs" API endpoint is exposed. More will follow soon!
Feel free to contribute. ;)

## Installation

Check out this repo, then install via

    cargo install --path .

Alternatively, the latest released version can be installed from crates.io with

    cargo install murdock-cli

## Getting started

Create a Github personal access token with at least the "user.email" scope.
Then, add a server configuration, e.g.,

    murdock-cli servers add <some_name> https://server/url --token <your token> --default

Then you can use the API, e.g.,

    murdock-cli jobs delete --age 30

## Generated Openapi code

`murdock-api` has been generated with

    openapi-generator-cli generate -i openapi.json -g rust --additional-properties library=reqwest,packageName=murdock-api -o murdock-api --skip-validate-spec
