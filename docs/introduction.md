---
layout: default
title: Introduction
nav_order: 2
---

# Introduction

## About Flow
Flow is a powerful VCS (Version Control System) designed to help developers manage their codebase efficiently. It utilizes
[git2](https://crates.io/crates/git2) for version control and provides a user-friendly interface for common tasks such as commiting changes, making repositories, etc.

To install Flow, follow the instructions in the [installation guide](installation.md).
For more information on how to use Flow, please refer to the [usage documentation](usage.md), or explore the [tutorials](tutorials.md).

## Features

- **Version Control**: Flow uses git2 for robust version control, allowing you to track changes, revert to previous versions, and collaborate with others.
- **User-Friendly Interface**: Flow provides a simple and intuitive interface for managing your codebase, making it easy to perform common tasks without needing to remember complex git commands.
- **Extensible**: Flow is designed to be extensible, allowing you to add custom commands and functionality as needed.
- **Open Source**: Flow is open source and available on [GitHub](https://github.com/Pjdur/Flow).

## Current Commands

- `flow init`: Initialize a new Flow repository with a specified directory.
- `flow add`: Add files to the staging area for commit.
- `flow commit`: Commit changes to the repository.
- `flow clone`: Clone a remote repository.
- `flow fetch`: Fetch changes from a remote repository.
- `flow merge`: Merge branches.

> **Note**: Currently, Flow is in its early stages of development, and more commands will be added in the future. The current commands are designed to cover the most common use cases for version control, but additional functionality will be added as needed.
---