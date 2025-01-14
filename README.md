# SolDeer ![Rust][rust-badge] [![License: MIT][license-badge]][license]

[rust-badge]: https://img.shields.io/badge/Built%20with%20-Rust-e43716.svg
[license]: https://opensource.org/licenses/MIT
[license-badge]: https://img.shields.io/badge/License-MIT-blue.svg

<p align="center">
  <img src="./soldeer.jpeg" />
</p>

Soldeer is a package manager for Solidity built in Rust.

Solidity development started to become more and more complex. The need for a package manager was evident.
This project was started to solve the following issues:

- git submodules in foundry are not a good solution for managing dependencies
- npmjs was built for the js ecosystem not for solidity
- github versioning of the releases is a pain and not all the projects are using it correctly

## Version 0.2.13

### Version 0.2.7 introduces the following breaking changes

Save the dependency key as the dependency name to respect the Cargo.toml format. For multiple versions for the same dependency an issue has been created to be added as a feature [#34](https://github.com/mario-eth/soldeer/issues/34). For now the dependency name is the key in the toml file.

### Breaking Changes introduced in 0.2.6

In 0.2.6 the `sdependencies` has been renamed to `dependencies`. Furthermore a dependency now stored in the toml respects Cargo toml format with `version` and `url` included.

### WARNING

#### BETA VERSION - USE AT YOUR OWN RISK

Soldeer has 3 parts:

- soldeer cli - standalone tool that can be used for managing dependencies on project, it is independent and not tied to foundry
- soldeer repository - a central repository used to store various packages. Anyone can push their own packages as public. The repository works like npmjs or crates.io
- soldeer foundry (in progress) - a foundry plugin that will allow you to use soldeer in your foundry projects directly from forge

### HOW TO USE IT

#### Soldeer CLI

The Soldeer cli is a standalone tool that can be used to manage dependencies in your project. It is not tied to foundry and can be used in any project.
The cli can also be used alongside foundry as well by installing dependencies in a new directory called `dependencies` and also it can be used to update the remappings in the `remappings.txt` file.

In order to use the cli you have to install it via cargo:

```bash
cargo install soldeer
```

The above command will install `soldeer` in your `~/.cargo/bin` folder. Ensure it's added to your PATH if it isn't already (usually it is).

Then you have to create a `soldeer.toml` file in the root of your project. The file should look like this:

```toml
[remappings]
enabled = true

[dependencies]
```

The `remappings` option let's you enable or disable the remappings autocompletion. If you set it to `true` then the remappings will be automatically updated when you install a new dependency.
The `dependencies` option is used to store the dependencies that you install via the `soldeer install <dependency>~<version>` command.

If you want to use it with the foundry you can skip the creation of the `soldeer.toml` file and use the `foundry.toml` file instead. You just have to add the `dependencies` option in the `foundry.toml` file and the remappings will be updated automatically.

Example of foundry configuration:

```toml
[profile.default]
auto_detect_solc = false
bytecode_hash = "none"
cbor_metadata = false

.... other foundry config
[dependencies]
```

Even if the `[dependencies]` is empty, this will tell to soldeer to use the `foundry.toml` file for the dependencies management.

#### WARNING

If you do not define a `soldeer.toml` with the `enabled` field or a `foundry.toml` with the `dependencies` field, the remappings will not be updated and you will receive a warning.

### HOW TO INSTALL IT

```bash
cargo install soldeer
```

#### Check if installation was successful

```bash
soldeer help
```

### Install from sources

`cargo build --release` and use the `soldeer` binary from `target/release/`.

### How to use it

`Soldeer` is straightforward. It can work with `foundry.toml` file or you can create a `soldeer.toml`. From version `0.1.5` you can skip the creation of the `soldeer.toml`
if you want to use just the `foundry.toml` file.

### DEPENDENCY INSTALLATION

Add the dependencies by using

```bash
soldeer install <dependency_name>~<version>
```

To search if the dependency is available, visit [https://soldeer.xyz](https://soldeer.xyz).

Additionally, you can install a dependency from any zip file located at any URL:

```bash
soldeer install <dependency_name>~<version> <url>
```

This command will download the zip file of the dependency, unzip it, install it in the `dependencies` directory.

### How to push a new dependency to the repository

In order to push a new dependency to the repository you have create an account on [https://soldeer.xyz](https://soldeer.xyz), create a project that it will match the dependency name.

Example:
Create a project called `my-project` and then use the `soldeer push my-project~v1.0`. This will push the project to the repository and it will be available for everyone to use.
Before using the push command you have to use `soldeer login` to login to the repository.

### Full list of commands

For more commands use `soldeer help`.

### Foundry integration

Once the foundry integration is finished, you will be able to use soldeer directly from foundry by using `forge soldeer ...`.
You will have the same commands as in the standalone version.

### CAVEATS

The "add to remappings" feature only appends to the remappings.txt file and does not delete old dependencies. If you want to remove a dependency from remappings, you must do it manually.

If you use other dependency managers, such as git submodules or npm, ensure you don't duplicate dependencies between soldeer and the other manager.
