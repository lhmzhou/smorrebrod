# smorrebrod

`smorrebrod` runs a shell script prompt to assist with the automatic installation of useful Mac tools for frontend React developers. `smorrebrod` is built in thread-safe Rust because users can easily download the compiled file and hit the ground running within a space of minutes. The following common React tool chain is installed with a simple execution: 

```
Homebrew
    * zsh
    * zsh-completions
    * coreutils
    * corkscrew
    * gdbm
    * htop-osx
    * libtool
    * pcre
    * openssl
    * libyaml
    * nginx
    * readline
    * ruby 
    * wget
    * mongodb
    * docker
    * docker-compose
    * p7zip
    * couchdb
    * erlang
    * geoip
    * nettle
    * redis
nvm
    * Node version: 10.15.3
ohmyzsh
```

## Prerequisites

```
target the latest stable release of Rust and Cargo

use nvm or node installer to install Node.js and npm

git, github access and corresponding ssh key

bash

configure the OS proxy variables in the shell
```

## Usage

Install Rust
```
$ curl https://sh.rustup.rs -sSf | sh
```

Confirm the `PATH` environment variable contains the Cargo's binary directory: `~/.cargo/bin`

Build
```
$ cargo build
```

Run
```
$ cargo run
```

## TODOs

- [ ] Set up interactive terminal for implementing specific package installation.
- [ ] Moving and organizing code into blocks and moving each main function into a library as to not pollute the main.rs file
- [ ] Add testing (function, integration, e2e, etc)
- [ ] Add complete e2e installation

## Helpful Links

[ohmyzsh](https://github.com/ohmyzsh/ohmyzsh)
</br>
[homebrew](https://brew.sh/)
</br>
[nodejs](https://nodejs.org/en/download/)
</br>
[nvm](https://github.com/nvm-sh/nvm)
</br>
[Git download](https://git-scm.com/downloads)
</br>
[Rust Installation](https://www.rust-lang.org/tools/install)
</br>
[Rust Community](https://users.rust-lang.org/)
