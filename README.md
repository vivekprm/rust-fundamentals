# rust-fundamentals
https://www.linkedin.com/learning/rust-programming-from-fundamentals-to-advanced-concepts-with-ai-assisted-development/demo-customizing-the-environment?autoSkip=true&resume=false&u=2284609

## Installing
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Output:
```
Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  $HOME/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  $HOME/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  $HOME/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  $HOME/.profile
  $HOME/.zshenv

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.
```

```
To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, you need to source
the corresponding env file under $HOME/.cargo.

This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
```

Install below vscode extension:
https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

