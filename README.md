r-device
========

A sample device in rust for sending data to Google Cloud IoT Core over MQTT.

Prerequisites:
-------------
- openssl
- wget
- [rust compiler](https://www.rust-lang.org)
- linux/bash (for the key generation script)

- Google Cloud SDK (for the gcloud commands, can be done online)

How to run:
===========
1. Run `gen_keys.sh`.
2. Create a Google Cloud project.
3. Lots of configuring of Google Cloud. See next section.
4. Change static variables in `src/main.rs`.
5. Run `cargo run`. This downloads the rest of the dependencies, and compiles the program before it is run, this can take some time.
6. The program counts uppwards, and publishes the counter to Google Cloud every 3rd second.
7. run `gcloud pubsub subscriptions pull --auto-ack projects/PROJECT_ID/subscriptions/SUBSCRIPTION_ID` to pull the messages from the cloud. Add `--limit=n` to pull `n` messages at a time.  

Creation and configuration of a new Google Cloud project:
=========================================================
Most of these steps can also be done online, using the GUI or the cloud console.
Replace words in CAPITAL letters with something apropreate of your choice.
1. `gcloud projects create PROJECT_ID --enable-cloud-apis` (you can add `--set-as-default`)
2. `gcloud pubsub topics create TOPIC_ID`
3. `gcloud pubsub subscriptions create prijects/PROJECT_ID/subscriptions/SUBSCRIPTION_ID --topic=TOPIC_ID`
4. Grant permission to the Cloud IoT Core service account. (This can be done by downloading https://github.com/GoogleCloudPlatform/nodejs-docs-samples.git, navigating to the iot subfolder and run `npm --prefix ./scripts install` and `node scripts/iam.js TOPIC_ID`). TODO: Find out what actually happens here, and give better explanaiton.
5. `gcloud iot registries create my-registry --project=PROJECT_ID --region=REGION --event-notification-config=topic[projects/PROJECT_ID/topics/TOPIC_ID`
6. `gcloud iot devices create DEVICE_ID --project=PROJECT_ID --region=REGION --registry=REGISTRY_ID --public-key path=rsa_cert.pem,type=rs256`