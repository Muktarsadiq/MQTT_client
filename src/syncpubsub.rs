/*Imports */
use rumqttc::{Client, LastWill, MqttOptions, QoS};
use std::thread;
use std::time::Duration;


fn main() {
		
		//initialize the logger
		pretty_env_logger::init();
		
		//Set Mqtt connection options and last will message
		let mut mqttoptions = MqttOptions::new("test-1", "broker.emqx.io", 1883);
		let will = LastWill::new("hello/world", "good bye", QoS::AtMostOnce, false);
		
		//expect commnication at least every  seconds
		mqttoptions
				.set_keep_alive(Duration::from_secs(5))
				.set_last_will(will);
				
		//create mqtt client and connection, and call the publish function in a new thread
		let (client, mut connection) = Client::new(mqttoptions, 10);
		//spawn a new os thread
		thread::spawn(move || publish(client));
		
		//iterate throgh the notifications in the connection and handle each notification
		for (i, notification) in connection.iter().enumerate() {
				match notification {
						Ok(notif) => {
								println!("{i}. Notification = {notif:?}");
						}
						Err(error) => {
								println!("{i}. Notification = {error:?}");
						}
				}
		}
		
		println!("DOne with the stream!!");
		
}

/*helper function for publishing MQTT messages */
fn publish(client: Client) {
		
		//wait for one second before subscribing to a topic
		thread::sleep(Duration::from_secs(1));
		client.subscribe("hello/+/world", QoS::AtMostOnce).unwrap();
		
		//send ten messages with lengths ranging from 0 to 9, each message's QoS being at least once
		for i in 1..10_usize {
				let payload = vec![1; i];
				let topic = format!("hello/{i}/world");
				let qos = QoS::AtLeastOnce;
				
				client.publish(topic, qos, true, payload).unwrap();
		}
		
		//wait one second again to allow for messages to be transmitted.
		thread::sleep(Duration::from_secs(1));
			
}
