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

### `flow commit -m <message>`
This command commits your changes to the Flow repository. The `-m` option allows you to specify a commit message.

```bash
flow commit -m "Your commit message"
```

For more information, see [the tutorials](tutorials.md).