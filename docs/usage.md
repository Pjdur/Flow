---
layout: default
title: Usage
nav_order: 4
---

# Usage

## Help

For basic help from Flow, run:

```bash
flow --help
```
This will show you the available commands and options.
## Commands

### `flow init <name>`
This command initializes a new Flow repository. It initializes an empty repository (unless a template is specified).

```bash
flow init repository_name
```

### `flow init <name> --template <template>`
This command initializes a new Flow repository using a template. A template is a pre-defined structure for your repository, which can include files, directories, and configurations.

```bash
flow init repository_name --template template_name
```

### `flow merge <branch>`
This command merges the specified branch into the current branch. It is useful for integrating changes from one branch into another.

```bash
flow merge branch_name
```

### `flow fetch`
This command fetches changes from a remote repository. It updates your local repository with the latest changes from the remote, without merging them into your current branch.

```bash
flow fetch
```

### `flow clone <repository>`
This command clones a remote repository to your local machine. It creates a copy of the repository, including all its history and branches.

```bash
flow clone repository_url
```

### `flow add <file>`
This command adds a file to the Flow repository. It stages the file for commit, meaning it will be included in the next commit.

```bash
flow add file_name
```

### `flow commit -m <message>`
This command commits your changes to the Flow repository. The `-m` option allows you to specify a commit message.

```bash
flow commit -m "Your commit message"
```

For more information, see [the tutorials](tutorials.md).