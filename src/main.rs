extern crate rumqtt;
use rumqtt::*;

extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

use std::time;
use std::ops::Add;
use std::thread;
use std::sync::Arc;

// TODO: CHANGE THESE VARIABLES
static PROJECT_ID: &str = "didrik-test";
static LOCATION: &str = "us-central1";
static REGISTRY_ID: &str = "myregistry";
static DEVICE_ID: &str = "key-test";
static SUBTOPIC: &str = ""; // This don't have to be edited, but if edited it must start with a /. E.g. "/alarms/garage"


static BROKER: &str = "mqtt.googleapis.com:8883";
//static BROKER: &str = "test.mosquitto.org:1883";

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

fn register_address(message: Message) {
    println!("Topic: {}", message.topic.to_string());
    println!("Payload: {}", String::from_utf8(Arc::try_unwrap(message.payload).unwrap()).unwrap());
    //panic!("Received:\n Topic: {}\n Payload: {}", message.topic.to_string(), String::from_utf8(Arc::try_unwrap(message.payload).unwrap()).unwrap())
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
    let token = jwt::encode(&header, &claims, include_bytes!("../../rsa_private.der")).unwrap();

    println!("token: {:?}", token);


    let client_options = MqttOptions::new()
                                      .set_keep_alive(60)
                                      .set_client_id(format!("projects/{}/locations/{}/registries/{}/devices/{}", PROJECT_ID, LOCATION, REGISTRY_ID, DEVICE_ID))
                                      .set_password(token.as_str())
                                      .set_ca("roots.pem")
                                      .set_broker(BROKER);

    let mqtt_callbacks = Some(MqttCallback::new()
                                            .on_message(register_address));


    println!("Starting client");
    
    let mut mqtt_client = MqttClient::start(client_options, mqtt_callbacks).expect("FAILED TO START CLIENT");

    println!("MQTT client started");


    println!("Subscribing to /devices/{}/config", DEVICE_ID);
    mqtt_client.subscribe(vec![(format!("/devices/{}/config", DEVICE_ID).as_str(), QoS::Level1)]).expect("UNABLE TO SUBSCRIBE");

    let mut i = 0;
    loop {
        let data = Data {
            timestamp: time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs(),
            data: i,
        };

        let json_data = serde_json::to_string(&data).expect("UNABLE TO SERIALZE DATA");
        mqtt_client.publish(format!("/devices/{}/events{}", DEVICE_ID, SUBTOPIC).as_str(), QoS::Level1, json_data.into_bytes()).expect("UNABLE TO PUBLISH");
        println!("published {:?}", data);

        thread::sleep(time::Duration::from_secs(3));
        i = (i + 1) % 128;
    }
}
