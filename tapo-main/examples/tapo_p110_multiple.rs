use std::{env, process::Command, thread, time::Duration};
use log::LevelFilter;
use tapo::{ApiClient, P110};
use paho_mqtt::{Client, CreateOptionsBuilder, Message};
use firebase_rs::*;
use serde_json::json;

/// Discover Tapo devices based on their MAC address prefix.
fn discover_tapo_devices() -> Vec<String> {
    let output = Command::new("sudo")
        .arg("arp-scan")
        .arg("-l")
        .output()
        .expect("Failed to execute arp-scan");
    let output_str = String::from_utf8_lossy(&output.stdout);

    let mut ip_addresses = Vec::new();
    for line in output_str.lines() {
        if line.contains("30:de:4b:36") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                ip_addresses.push(parts[0].to_string());
            }
        }
    }

    ip_addresses
}

/// Set up the MQTT client.
fn setup_mqtt() -> Client {
    let create_options = CreateOptionsBuilder::new()
        .server_uri("tcp://127.0.0.1:1883")
        .client_id("tapo-client")
        .finalize();
    let client = Client::new(create_options).expect("Failed to create MQTT client");
    client.connect(None).expect("Failed to connect to MQTT broker");

    client
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Firebase
    let firebase = Firebase::new("https://taicare-default-rtdb.europe-west1.firebasedatabase.app/")
        .expect("Failed to initialize Firebase");

    // Set up logging
    let log_level = env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .parse()
        .unwrap_or(LevelFilter::Info);
    pretty_env_logger::formatted_timed_builder()
        .filter(Some("tapo"), log_level)
        .init();

    // Read environment variables for Tapo authentication
    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;

    // Discover Tapo devices' IP addresses
    println!("Starting IP discovery...");
    let discovered_ips = discover_tapo_devices();
    println!("Discovered IPs: {:?}", discovered_ips);
    
    // Discover devices
    let device_futures: Vec<_> = discovered_ips.iter()
    .map(|ip| ApiClient::<P110>::new(ip.clone(), tapo_username.clone(), tapo_password.clone(), true))
    .collect();

    let devices = futures::future::join_all(device_futures).await;
    println!("API Clients created for {} devices.", devices.len());    

    // Set up MQTT
    println!("Setting up MQTT...");
    let client = setup_mqtt();
    println!("MQTT setup complete.");

    loop {
        println!("Starting loop iteration...");
        for device_result in &devices {
            // Check if the device creation was successful
            match device_result {
                Ok(device) => {
                    // Fetch device information and energy usage
                    println!("Fetching device info...");
                    let device_info = device.get_device_info().await?;
                    println!("Device info fetched successfully!");
    
                    println!("Fetching energy usage...");
                    let energy_usage = device.get_energy_usage().await?;
                    println!("Energy usage fetched successfully!");
    
                    // Combine them into a JSON object
                    let important_information = json!({
                        "device_info": device_info,
                        "energy_usage": energy_usage
                    });
    
                    // Prepare the message for MQTT
                    let important_information_str = serde_json::to_string(&important_information)?;
                    let message = Message::new("tapo/important_information", important_information_str.as_bytes(), 0);
    
                    // Send data to Firebase
                    println!("Publishing to Firebase...");
                    let firebase_info = firebase.at("importantInformation");
                    firebase_info.set(&important_information).await.map_err(|err| {
                        println!("{:?}", err);
                        std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", err))
                    })?;
                    println!("Published to Firebase!");
    
                    // Publish to MQTT
                    println!("Publishing to MQTT...");
                    client.publish(message)?;
                    println!("Published to MQTT!");
                },
                Err(e) => {
                    println!("Failed to create API client for a device: {}", e);
                }
            }
        }
    
        thread::sleep(Duration::from_secs(5));
    }
    
}