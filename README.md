# API Starknet Quest
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-3-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

API for Starknet Quest Client project built in Rust

## About

API Starknet Quest provides the backend infrastructure for Starknet Quest Client, an app which helps protocols attract and retain users by creating gamified quests experiences on Starknet.

## Prerequisites

### Install Rust

To run the project without issues you need to have a Rust version >= 1.73.0. To check your rust version run the following command in a terminal.

```bash
rustc --version
```
If you don't have Rust installed, please go to the [Rust installation page](https://doc.rust-lang.org/book/ch01-01-installation.html) for further instructions.

### Install Git

Go to the [Git installation page](https://git-scm.com/downloads) and follow the instructions for your operating system to install Git.

### Install Docker (optional)

Docker may be useful for advanced setups but is not required for basic usage since we’re now leveraging MongoDB Atlas for database setup. If you wish to use Docker for other purposes, please ensure you have Docker engine version >= 1.13.0. Check your version with.

```bash
docker --version
```
If you don't have Docker installed, please go to the [Docker installation page](https://docs.docker.com/get-started/get-docker/) for further instructions.

## Installation Instructions

Fork the repository and clone the forked repository to your local system

```bash
git clone https://github.com/<your-user>/api.starknet.quest.git
```

## Build Instructions

To build the project use the following command in a terminal

```bash
cargo build
```

The command above will run `cargo build` with the `--debug` flag, which compiles faster, includes debug symbols for easier debugging. However it produces a larger binary, for development purposes the command above is fine.

If you wish to create an optimized binary without debug information run the following command in a terminal

```bash
cargo build --release
```

## Running the Project

To run the project successfully you'll need to do the following steps:
1. Set Up a MongoDB Atlas Database:

  - Go to [MongoDB Atlas](https://www.mongodb.com/products/platform/atlas-database) and create a free account if you don't have one.

  - Create a new cluster by following the instructions provided on the Atlas dashboard.

  - Once your cluster is ready, click on "Connect," then choose "Connect your application."

  - Copy the connection string provided, which should look something like this:
    [Example:]
     ( mongodb+srv://<username>:<password>@cluster0.mongodb.net/<database>?retryWrites=true&w=majority )
  - Make sure your IP address is whitelisted in the Atlas security settings.

2. Create `config.toml` file using the `config.template.toml` file.
Create a `config.toml` file by copying and modifying the `config.template.toml` file. Make sure you update the following fields as required to run the project successfully:

- `connection_string`, this is the string to connect to the database. Replace with the MongoDB Atlas connection string here.
- `secret_key`, this is the secret used for the JWT token. You can change it or leave as is.
- `expiry_duration`, this is the expiry duration of the JWT token. You should change it according to your needs the time is stored in miliseconds.
- `rpc_url`, this is to interact with the blockchain you can use a public RPC such as [Lava](https://www.lavanet.xyz/get-started/starknet) or a private node provider such as [Alchemy](https://www.alchemy.com) or [Infura](https://www.infura.io). Alchemy and Infura require an account to get a private RPC, while Lava is completely public.
- In the section of `[watchtower]`, set `enabled` to false. If you wish to setup the watchtower correctly, you can check the Watchtower repositories for further information. [Watchtower frontend](https://github.com/starknet-id/watchtower.starknet.id) and [Watchtower backend](https://github.com/starknet-id/watchtower_server) 

3. Run the project. 
Once the `config.toml` file is created properly, you're going to be able to run the project using the following command

```bash
cargo run
```
If you've setup everything correctly, you should see the following output

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 59.57s
     Running `target/debug/quest_server`
quest_server: starting v0.1.0
database: connected
server: listening on http://0.0.0.0:8080
```
If you have a different output, refer the to the Troubleshooting guide below.

If you wish to test admin endpoints, you need to add the admin manually to the database.

## Troubleshooting

If your expected output doesn't includes the following text:
```bash
database: connected
server: listening on http://0.0.0.0:8080
```
This means you didn't add MongoDB database. To fix this, you'll need to follow first step of the section Running the Project.

If you get the following output:

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 59.57s
     Running `target/debug/quest_server`
quest_server: starting v0.1.0
thread 'main' panicked at src/config.rs:212:9:
error: unable to read file with path "config.toml"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This means you didn't create the `config.toml` file. To fix this, you'll need to create the `config.toml` file with the steps mentioned in the second step of the section Running the Project.

If you get the following output:

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
     Running `target/debug/quest_server`
quest_server: starting v0.1.0
thread 'main' panicked at src/main.rs:34:49:
called `Result::unwrap()` on an `Err` value: RelativeUrlWithoutBase
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This means you didn't add the `rpc_url` in the `config.toml` file. To fix this, you'll need to add the `rpc_url` to the `config.toml` file. Please refer the second step of the section Running the Project for further instructions on how to add the `rpc_url`.

If you get the following output:

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/quest_server`
quest_server: starting v0.1.0
thread 'main' panicked at src/main.rs:29:10:
called `Result::unwrap()` on an `Err` value: Error { kind: InvalidArgument { message: "connection string contains no scheme" }, labels: {}, wire_version: None, source: None }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This means you didn't add the `connection_string` in the `config.toml` file. To fix this, you'll need to add the `connection_string` to the `config.toml` file. Please refer to the second step of the section Running the Project for further instructions on how to add the `connection_string`.

If you get the following output:

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.41s
     Running `target\debug\quest_server.exe`
quest_server: starting v0.1.0
thread 'main' panicked at src\config.rs:218:13:
error: unable to deserialize config. newline in string found at line 6 column 63
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\quest_server.exe` (exit code: 101)
```

This means you probably forgot the following character in the `config.toml` file: ". To fix this, you'll need to check that the fields you modified while creating the `config.toml` file have their opening and closing character. As an example, it should look like this:

`connection_string = "mongodb+srv://<username>:<password>@cluster0.mongodb.net/<database>?retryWrites=true&w=majority"`

and NOT like this

`connection_string = "mongodb+srv://<username>:<password>@cluster0.mongodb.net/<database>?retryWrites=true&w=majority`

If you get the following output:

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/quest_server`
INFO: quest_server: starting v0.1.0
Failed to post log: "Invalid token or token expired"
```

This means that you didn't setup the credentials for Watchtower. To fix this, you'll need to set the `enabled` field in `[watchtower]` to false in the `config.toml` file. Please refer the second step of the section Running the Project for further instructions if you wish to keep the `[watchtower]` enabled.
## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Gerson2102"><img src="https://avatars.githubusercontent.com/u/71728860?v=4?s=100" width="100px;" alt="Gerson"/><br /><sub><b>Gerson</b></sub></a><br /><a href="https://github.com/lfglabs-dev/api.starknet.quest/commits?author=Gerson2102" title="Code">💻</a> <a href="#business-Gerson2102" title="Business development">💼</a> <a href="#ideas-Gerson2102" title="Ideas, Planning, & Feedback">🤔</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Th0rgal"><img src="https://avatars.githubusercontent.com/u/41830259?v=4?s=100" width="100px;" alt="Thomas Marchand"/><br /><sub><b>Thomas Marchand</b></sub></a><br /><a href="https://github.com/lfglabs-dev/api.starknet.quest/commits?author=Th0rgal" title="Code">💻</a> <a href="#business-Th0rgal" title="Business development">💼</a> <a href="#ideas-Th0rgal" title="Ideas, Planning, & Feedback">🤔</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Marchand-Nicolas"><img src="https://avatars.githubusercontent.com/u/60229704?v=4?s=100" width="100px;" alt="Nico"/><br /><sub><b>Nico</b></sub></a><br /><a href="https://github.com/lfglabs-dev/api.starknet.quest/commits?author=Marchand-Nicolas" title="Code">💻</a> <a href="#business-Marchand-Nicolas" title="Business development">💼</a> <a href="#ideas-Marchand-Nicolas" title="Ideas, Planning, & Feedback">🤔</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
