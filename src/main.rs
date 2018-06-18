extern crate rumqtt;
use rumqtt::*;

extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;

use std::time;
use std::ops::Add;

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
        aud: "didrik-test".to_owned(),
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
                                      .set_client_id("projects/didrik-test/locations/us-central1/registries/myregistry/devices/mylaptop")
                                      .set_password(token.as_str()) //TODO: add jwt
                                      .set_broker(BROKER);

    println!("Starting client");
    
    let mqtt_client = MqttClient::start(client_options, None).expect("FAILED TO START CLIENT");

    println!("MQTT client started");
}
