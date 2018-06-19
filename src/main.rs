extern crate rumqtt;
use rumqtt::*;

extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;

use std::time;
use std::ops::Add;
use std::thread;

// TODO: CHANGE THESE VARIABLES
static PROJECT_ID: &str = "didrik-test";
static LOCATION: &str = "us-central1";
static REGISTRY_ID: &str = "myregistry";
static DEVICE_ID: &str = "mylaptop";
static SUBTOPIC: &str = "events";


static BROKER: &str = "mqtt.googleapis.com:8883";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iat: String,
    exp: String,
    aud: String,
}


fn main() {
    println!("Hello, world!");

    let now = time::SystemTime::now();

    let claims = Claims {
        iat: now.duration_since(time::UNIX_EPOCH).unwrap().as_secs().to_string(), 
        exp: now.add(time::Duration::from_secs(12*60*60)).duration_since(time::UNIX_EPOCH).unwrap().as_secs().to_string(), 
        aud: PROJECT_ID.to_owned(),
    };

    let mut header = jwt::Header::new(jwt::Algorithm::RS256);
    header.typ = Some("jwt".to_owned());



    println!("header: {:?}", claims);
    /*
    let mut file = File::open("rsa_private.der").unwrap();
    let mut key = String::new();
    file.read_to_string(&mut key).unwrap();

    //let key = "secret";

    println!("key: {}", key);
    */
    let token = jwt::encode(&header, &claims, include_bytes!("../rsa_private.der")).unwrap();

    println!("token: {:?}", token);


    let client_options = MqttOptions::new()
                                      .set_keep_alive(60)
                                      .set_client_id(format!("projects/{}/locations/{}/registries/{}/devices/{}", PROJECT_ID, LOCATION, REGISTRY_ID, DEVICE_ID))
                                      .set_password(token.as_str())
                                      .set_ca("roots.pem")
                                      .set_broker(BROKER);

    println!("Starting client");
    
    let mut mqtt_client = MqttClient::start(client_options, None).expect("FAILED TO START CLIENT");

    println!("MQTT client started");

    for i in 0.. {
        mqtt_client.publish(format!("/devices/{}/{}", DEVICE_ID, SUBTOPIC).as_str(), QoS::Level1, i.to_string().into_bytes()).expect("UNABLE TO PUBLISH");
        println!("published {}", i);

        thread::sleep(time::Duration::from_secs(3));
    }
}
