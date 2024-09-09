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
- [Cargo Make](https://github.com/sagiegurari/cargo-make)
  ```bash
  cargo install cargo-make
  ```
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Golem CLI](https://github.com/golemcloud/golem/releases)
    - You can download the latest binary from the GitHub releases page
    - Or you can build it:
      ```bash
      cargo install golem-cloud-cli --features universal
      ```
      Note: Requires protobuf installed on your system

Run
---

1. Spin up Golem OSS (optional, to run locally):

    ```bash
    docker compose --env-file docker.env up -d
    ```

2. Building is as simple as:

    ```bash
    ./update_components.sh build
    ```
   This script will make sure to build, create the components and the router worker and load the API definition.

3. Call the API to:
    1. Create a user:

        ```bash
        source ./api_calls.sh
 
        create_user "Bob"
        { "message": "Success", "status": 200 }
        ```
    2. Get the user profile:

        ```bash
        source ./api_calls.sh
 
        get_profile 0
        { "status": 200, "user": { "user-id": 0, "username": "Bob", "followers": [], "following": [] } }
        ```

Test
----

To run unit tests, execute:

```bash
cargo test --all
```