extern crate rumqtt;
use rumqtt::*;

//extern crate jsonwebtoken as jwt;
extern crate frank_jwt as fjwt;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::time;
use std::ops::Add;
use std::thread;
use std::io::prelude::*;

// TODO: CHANGE THESE VARIABLES
static PROJECT_ID: &str = "didrik-test";
static LOCATION: &str = "us-central1";
static REGISTRY_ID: &str = "myregistry";
static DEVICE_ID: &str = "mylaptop";
static SUBTOPIC: &str = "events"; // Do not change, unless you know what you are doing


static BROKER: &str = "mqtt.googleapis.com:8883";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iat: String,
    exp: String,
    aud: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    timestamp: u64,
    data: i32,
}


fn main() {
    println!("Hello, world!");

    let now = time::SystemTime::now();

    let claims = Claims {
        iat: now.duration_since(time::UNIX_EPOCH).unwrap().as_secs().to_string(), 
        exp: now.add(time::Duration::from_secs(12*60*60)).duration_since(time::UNIX_EPOCH).unwrap().as_secs().to_string(), 
        aud: PROJECT_ID.to_owned(),
    };

    //let mut header = jwt::Header::new(jwt::Algorithm::RS256);
    //header.typ = Some("jwt".to_owned());

    let header = json!({
        "typ": "jwt",
        "alg": fjwt::Algorithm::ES256.to_string()
    });

    let claims = json!({
        "iat": now.duration_since(time::UNIX_EPOCH).unwrap().as_secs().to_string(),
        "exp": now.add(time::Duration::from_secs(12*60*60)).duration_since(time::UNIX_EPOCH).unwrap().as_secs().to_string(),
        "aud": PROJECT_ID.to_owned()
    });



    println!("header: {:?}", claims);

    let mut key = String::new();
    std::fs::File::open("ec_private.pem").expect("UNABLE TO OPEN KEY").read_to_string(&mut key).unwrap();
    println!("key: {}", key);
    let token = fjwt::encode(header, &key, &claims, fjwt::Algorithm::ES256).unwrap();

    println!("token: {:?}", token);


    let client_options = MqttOptions::new()
                                      .set_keep_alive(60)
                                      .set_client_id(format!("projects/{}/locations/{}/registries/{}/devices/{}", PROJECT_ID, LOCATION, REGISTRY_ID, DEVICE_ID))
                                      .set_password(token.as_str())
                                      //.set_password("eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdWQiOiJkaWRyaWstdGVzdCIsImV4cCI6IjE1Mjk2MzU1ODEiLCJpYXQiOiIxNTI5NTkyMzgxIn0.bsJ_iLueShMAosnkdEEkdzDaQKaUldKjkBpUr971EMmGM0SCQrvB0kXYrRfLJpBGHcm88LMpT1hCh-OPFVAXYg")
                                      .set_ca("roots.pem")
                                      .set_broker(BROKER);

    println!("Starting client");
    
    let mut mqtt_client = MqttClient::start(client_options, None).expect("FAILED TO START CLIENT");

    println!("MQTT client started");

    for i in 0.. {
        let data = Data {
            timestamp: time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs(),
            data: i,
        };

        let json_data = serde_json::to_string(&data).expect("UNABLE TO SERIALZE DATA");
        mqtt_client.publish(format!("/devices/{}/{}", DEVICE_ID, SUBTOPIC).as_str(), QoS::Level1, json_data.into_bytes()).expect("UNABLE TO PUBLISH");
        println!("published {:?}", data);

        thread::sleep(time::Duration::from_secs(3));
    }
}
