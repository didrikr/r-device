r-device
========

A sample device in rust for sending data to Google Cloud IoT Core over MQTT.

Prerequisites:
-------------
- openssl
- [rust compiler](https://www.rust-lang.org)
- linux/bash (for the key generation script)

- Google Cloud SDK (for the gcloud commands, can be done online)

How to run:
===========
1. Run `generate_keys.sh`
2. Create a Google Cloud project
3. Lots of configuring of Google Cloud
4. Change static variables in `src/main.rs`
5. Run `cargo run`
