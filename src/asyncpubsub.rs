
/*Imports*/
use tokio::{task, time};

use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::error::Error;
use std::time::Duration;


/*Main with the tokio runtime  */
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
		
		//initialize the logger
		pretty_env_logger::init();
		
		//set MQTT connection option and the keep_alive secs
		let mut mqttoptions = MqttOptions::new("test-1", "broker.emqx.io", 1883);
		mqttoptions.set_keep_alive(Duration::from_secs(5));
		
		//create a asynchronous MQTT client and event loop
		let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
		
		//spawn new tokio threads
		task::spawn(async move {
				requests(client).await;
				time::sleep(Duration::from_secs(3)).await;
		});
		
		loop {
				//wait for and retrieve the next event in the loop
				let event = eventloop.poll().await;
				
				//
				match &event {
						Ok(v) => {
								println!("Event = {v:?}");
						}
						Err(e) => {
								println!("Error = {e:?}");
								return Ok(());
						}
				}
		}		
}

/*asynchronous function for publishing and subscribing to messages*/
async fn requests(client: AsyncClient) {
		
		//send subscribe request to broker for topic
		client
			.subscribe("hello/world", QoS::AtMostOnce)
			.await
			.unwrap();
			
			//send 10 messages to the topic, with the length of each message increasing from 1 to 10
			for i in 1..=10 {
					client
							.publish("hello/world", QoS::ExactlyOnce, false, vec![1; i])
							.await
							.unwrap();
							
							time::sleep(Duration::from_secs(1)).await;
			}
			
			//sleep for 120 secs to keep requests tasks alive
			time::sleep(Duration::from_secs(120)).await;
}