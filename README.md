# Grid

Grid is a platform for building supply chain solutions that include
distributed ledger components. It provides a growing set of tools that
accelerate development for supply chain smart contracts and client interfaces.

This project is not an implementation of a distributed ledger or a client
application. Instead, Grid provides supply-chain-focused libraries,
data models, and software development kits (SDKs) as modular, reusable
components.

The Grid project includes several repositories:

- [This repository](https://github.com/splintercommunity/grid/) contains core
  components such as supply-chain-centric data types and smart permissioning
  code.

- The [grid-docs](https://github.com/splintercommunity/grid-docs) repository
  contains the source files for the Grid documentation and website
  hosted at [grid.splinter.dev](https://grid.splinter.dev/).

## How to Participate

We welcome contributors, both organizations and individuals, to help shape
project direction, contribute ideas, provide use cases, and work on specific
tools and examples. Please [join the
discussion](https://grid.splinter.dev/community/join_the_discussion.html).

## Building Grid

Grid is built using latest stable [rust](https://www.rust-lang.org/), which
you should install via [rustup](https://rustup.rs/).

To install the remaining dependencies using a package manager, run one of the
following commands.

Homebrew (OS X):
```bash
brew install openssl zeromq pkg-config protobuf libpq
```

APT (Ubuntu):
```bash
apt install \
    build-essential \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    libsqlite3-dev \
    libpq-dev \
    libsasl2-dev \
    libxml2-dev \
    libzmq3-dev \
    openssl
```

Once you have the prerequisites installed, run `cargo build` from the root
directory. This command builds all of the Grid components, including `gridd`
(the grid daemon), the CLI, and all of the smart contracts in the `contracts`
directory.

To build individual components, run `cargo build` in the component directories.
For example, to build only the grid-cli, navigate to `cli`, then run
`cargo build`.

### Building with Docker

To build Grid using Docker, run `docker-compose build` from the root directory.
This command builds Docker images for all of the Grid components, including
`gridd` (the grid daemon), the CLI, and all of the smart contracts in the
`contracts` directory.

To build individual components using Docker, run
`docker-compose build <component>` from the root directory. For example, to
build only the grid-cli, run `docker-compose build grid-cli`.

To use Docker to build Grid with experimental features enabled, set an
environment variable in your shell before running the build commands. For
example: `export 'CARGO_ARGS= --features experimental'`. To go back to
building with default features, unset the environment variable:
`unset CARGO_ARGS`

## More Information

- [Grid website](https://grid.splinter.dev/)
- [Documentation](https://grid.splinter.dev/docs/)
- [#grid discussion channel](https://discord.gg/BAVpP73NjW)


## License

Grid software is licensed under the [Apache License Version2.0](LICENSE)
software license.

The Grid documentation in the
[grid-docs](https://github.com/splintercommunity/grid-docs) repository is licensed
under a Creative Commons Attribution 4.0 International License (CC BY 4.0).
You may obtain a copy of the license at
<http://creativecommons.org/licenses/by/4.0/>.
