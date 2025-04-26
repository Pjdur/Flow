# Flow Installation Guide

Flow is a version control system written in Rust. Follow the steps below to install it on your system.

> Flow will soon have available install scripts.

## Prerequisites

- Ensure you have **Rust** and **Cargo** installed. You can install them using [Rustup](https://rustup.rs/):
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

- Verify the installation:
    ```bash
    rustc --version
    cargo --version
    ```

## Installation Steps

1. Clone the Flow repository:
     ```bash
     git clone https://github.com/your-org/flow.git
     cd flow
     ```

2. Build and install Flow using Cargo:
     ```bash
     cargo install --path .
     ```

3. Verify the installation:
     ```bash
     flow --version
     ```

## Updating Flow

To update Flow to the latest version, navigate to the cloned repository and run:
```bash
git pull origin main
cargo install --path .
```

## Uninstallation

To uninstall Flow, use Cargo:
```bash
cargo uninstall flow
```

## Additional Resources

- [Flow Documentation](https://github.com/Pjdur/flow/wiki)
- [Rust Programming Language](https://www.rust-lang.org/)
