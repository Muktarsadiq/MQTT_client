# MQTT_client

MQTT Client in Rust 🦀

A lightweight and high-performance MQTT client built in Rust, demonstrating synchronous and asynchronous message publishing and subscribing using the rumqttc
crate.

This project serves as a practical exploration of IoT messaging and broker communication patterns, showing how Rust’s concurrency model can handle scalable IoT data pipelines.

🚀 Features

✅ Synchronous MQTT Client – Blocking publish/subscribe loop using native threads

⚡ Asynchronous MQTT Client – Fully async with tokio runtime

💬 Last Will and Testament support

🔁 QoS Levels (AtMostOnce, AtLeastOnce, ExactlyOnce)

🔒 Keep Alive and heartbeat handling

🧩 Clean architecture for both client and broker integration

🏗️ Architecture Overview

The project uses rumqttc
— a modern Rust MQTT client library — and demonstrates how to:

Initialize MqttOptions

Configure LastWill

Spawn publish and subscribe threads (sync)

Use async task spawning with tokio

Manage event loops for message notifications

📦 Dependencies
[dependencies]
rumqttc = "0.24.0"
pretty_env_logger = "0.4"
tokio = { version = "1", features = ["full"] }

🧩 Project Structure
├── Cargo.toml
├── src
│ ├── syncpubsub.rs # Synchronous publish/subscribe implementation
│ └── asyncpubsub.rs # Asynchronous publish/subscribe implementation

🧠 Usage
1️⃣ Run the synchronous example
cargo run --bin syncpubsub

2️⃣ Run the asynchronous example
cargo run --bin asyncpubsub

🌐 Default Broker

The examples use a public MQTT broker:

broker.emqx.io
port: 1883

You can change this by editing the MqttOptions initialization:

let mut mqttoptions = MqttOptions::new("client-id", "your-broker-address", 1883);

🧪 Example Output

1. Notification = Incoming(Publish(hello/1/world, payload length: 1))
2. Notification = Incoming(Publish(hello/2/world, payload length: 2))
   ...
   Done with the stream!!

🧱 Learning Goals

This project helped explore:

How MQTT protocol ensures reliable IoT message delivery

The role of QoS levels in controlling network reliability

Rust’s async runtime (tokio) in managing concurrent broker communication

Comparing threaded sync vs async task performance in real-world MQTT pipelines

📈 Next Steps

Add TLS (MQTTS) connection support

Implement a configurable CLI for topic management

Add logging to a file/database for real IoT telemetry simulation

Benchmark latency and throughput between sync and async versions

⚖️ License
MIT License © 2025
