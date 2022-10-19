# Environment Setup

This file contains some notes from Dylan Lukes (TA) on their process for a fresh minimal Rust installation (based on MacOS).

**Note:** these instructions assume a UNIX-style environemnt (Linux or MacOS, but not Windows).

For simplicity, I am using VSCode.

## 1. Install `asdf` (optional)

See: https://asdf-vm.com/guide/getting-started.html

## 2. Install `rust` and `cargo`

Add the rust plugin and install the latest version.

```bash
asdf plugin add rust        # Install the rust plugin for asdf.
asdf install rust latest    # Install the latest version.
asdf global rust latest     # Activate it globally.
```

After the `install` step you will see a message along the lines of:

``
Rust is installed now. Great!

To get started you need Cargo's bin directory
(~/.asdf/installs/rust/1.64.0/bin) in your PATH
environment variable. This has not been done automatically.

To configure your current shell, run:
source "~/.asdf/installs/rust/1.64.0/env"
```

You can either do this, or you can just close your current terminal and open a new one. Or, you can `source ~/.zhshrc` or `source ~/.bashrc` depending on your preferred shell.

## 3. Check that `rust` and `cargo` work

Check that it works by running the following two commands. 

```bash
rustc
cargo
```

If both emit help messages respectively you're good to go!

## 4. (VSCode): Install `rust-analyzer`

The `rust-analyzer` plugin provides common programming support utilities like auto-complete, "Go To Definition", etc. 

Simply find it and install it. So long as your `cargo` works, it should "just work" in your environment.