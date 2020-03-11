use rumq_client::{MqttEventLoop, eventloop, subscribe, QoS, MqttOptions, Request, Notification};
use tokio::time::Duration;
use tokio::sync::mpsc::{Sender, Receiver};
use futures_util::stream::StreamExt;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::base::Config;

pub struct Mqtt {
    config:       Config,
    eventloop: MqttEventLoop,
    actions_tx: Sender<Notification>,
    mqtt_tx: Sender<Request>
}


impl Mqtt {
    pub fn new(config: Config, actions_tx: Sender<Notification>, mqtt_tx: Sender<Request>, mqtt_rx: Receiver<Request>) -> Mqtt {
        // create a new eventloop and reuse it during every reconnection
        let options = mqttoptions(&config);
        let eventloop = eventloop(options, mqtt_rx);

        Mqtt {
            config,
            eventloop,
            mqtt_tx,
            actions_tx 
        }
    }

    pub async fn start(&mut self) {
        let actions_subscription = format!("/devices/{}/actions", self.config.device_id);
        loop {
            let mut stream = self.eventloop.stream();
            while let Some(notification) = stream.next().await {
                match notification {
                    Notification::Connected => {
                        let subscription = subscribe(actions_subscription.clone(), QoS::AtLeastOnce);
                        let subscribe = Request::Subscribe(subscription);
                        if let Err(e) = self.mqtt_tx.send(subscribe).await {
                            error!("Failed to send subscription. Error = {:?}", e);
                        }
                    }
                    Notification::Publish(publish) if publish.topic_name == actions_subscription => {
                        let notification = Notification::Publish(publish);
                        if let Err(e) = self.actions_tx.send(notification).await {
                            error!("Failed to forward action. Error = {:?}", e);
                        }
                    }
                    _ => {
                        debug!("Notification = {:?}", notification);
                    }
                }
            }

            tokio::time::delay_for(Duration::from_secs(5)).await;
        }
    }
}

fn mqttoptions(config: &Config) -> MqttOptions {
    // let (rsa_private, ca) = get_certs(&config.key.unwrap(), &config.ca.unwrap());
    let mut mqttoptions = MqttOptions::new(&config.device_id, &config.broker, config.port);
    mqttoptions.set_keep_alive(30);
    mqttoptions
}

fn _get_certs(key_path: &Path, ca_path: &Path) -> (Vec<u8>, Vec<u8>) {
    println!("{:?}", key_path);
    let mut key = Vec::new();
    let mut key_file = File::open(key_path).unwrap();
    key_file.read_to_end(&mut key).unwrap();

    let mut ca = Vec::new();
    let mut ca_file = File::open(ca_path).unwrap();
    ca_file.read_to_end(&mut ca).unwrap();

    (key, ca)
}
