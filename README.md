Golem X
=======

Golem X is a project meant to run on [Golem Cloud](https://golem.cloud) that aims to implement some basic social
networking features like posting, following, fetch user profile, tweets and timeline.

Each component in the architecture exposes a WebAssembly interface that can be called by other components.

Setup
-----

The project requires:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo Component](https://github.com/bytecodealliance/cargo-component)
  ```bash
  cargo install cargo-component
  ```
- [Golem](https://github.com/golemcloud/golem/releases)
    - You can download the latest binary from the GitHub releases page
- [Golem CLI](https://github.com/golemcloud/golem/releases)
    - You can download the latest binary from the GitHub releases page
    - Or you can build it:
      ```bash
      cargo install golem-cli
      ```
      Note: Requires protobuf installed on your system

Run
---

1. Spin up Golem OSS (optional, to run locally):

    ```bash
    golem start
    ```

2. Building is as simple as:

    ```bash
    golem-cli app -b release build
    ```

   but you can use the provided script to simplify also the loading steps:

    ```bash
    ./update_components.sh
    ```

   The script will streamline the process of building and creating the components into Golem,
   as well as loading and deploying the API definition.

3. Call the API to:
    1. Get the user profile:

   ```bash
   source ./api_calls.sh

   get_profile bob
   { "status": 200, "user": { "username": "bob", "followers": [], "followings": [] } }
   ```

Test
----

To run unit tests, execute:

```bash
cargo test --all
```