# ❄️ kola-application 🚀

 [![NixOS](https://img.shields.io/badge/Flakes-Nix-informational.svg?logo=nixos&style=for-the-badge)](https://nixos.org) ![License](https://img.shields.io/github/license/mordragt/nix-templates?style=for-the-badge)

DevOps/GitOps Application Repository using Continuos Integration and Continuous Delivery

## About

- uses GitHub Actions

## Github Actions

### Events

- Trigger for a workflow (e.g. someone pushes new code)

### Jobs

- collection of steps to perform

### Runners

- where to run jobs on (basically container environment)

### Steps

### Actions

## Usage

- `nix develop`: opens up a `bash` shell with the necessary tooling
- `nix build` : builds the package to the symbolic `result` directory
- `nix run`: runs the package

## Reference

1. [Github Actions](https://docs.github.com/en/actions)
