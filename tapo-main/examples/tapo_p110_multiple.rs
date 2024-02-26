use std::{env, process::Command, thread, time::Duration};
use mongodb::{Client, options::{ClientOptions, ResolverConfig}, bson::doc};
use log::LevelFilter;
use tapo::{ApiClient, PlugEnergyMonitoringHandler};
use firebase_rs::*;
use serde_json::json;
use bson::Document;
use bson::Bson::DateTime;
use tokio::time::timeout;
use chrono::Utc;

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
        if line.contains("30:de:4b:36") || line.contains("78:8c:b5:7") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                ip_addresses.push(parts[0].to_string());
            }
        }
    }
    ip_addresses
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let current_time = Utc::now();

   // Load the MongoDB connection string from an environment variable:
   let client_uri =
      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

   // A Client is needed to connect to MongoDB and an extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;

    // Initialize Firebase
    let _firebase = Firebase::new("https://taicare-default-rtdb.europe-west1.firebasedatabase.app/")
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
    .map(|ip| ApiClient::new(tapo_username.clone(), tapo_password.clone()).expect("REASON").p110(ip.clone()))
    .collect();

    let devices: Vec<Result<PlugEnergyMonitoringHandler, tapo::Error>> = futures::future::join_all(device_futures).await;
    println!("API Clients created for {} devices.", devices.len());    

    loop {
        println!("Starting loop iteration...");
	let loop_result = timeout(Duration::from_secs(30), async {
		for device_result in &devices {
		    // Check if the device creation was successful
		    match device_result {
		        Ok(device) => {
		            let current_time = Utc::now();

		            // Fetch device information and energy usage
		            println!("Fetching device info...");
		            let device_info = device.get_device_info().await?;
		            println!("Device info fetched successfully!");
	    
		            println!("Fetching energy usage...");
		            let energy_usage = device.get_energy_usage().await?;
		            println!("Energy usage fetched successfully!");
	    
		            // FILTRAR SI ESTÁ ENCENDIDO O APAGADO
		            let nickname = &device_info.nickname;
		            let device_id = &device_info.device_id;

		            let current_power = &energy_usage.current_power;
		            let current_power_i64 = *current_power as i64;
		            let local_time = &energy_usage.local_time;
		            let status = &device_info.device_on;
		            
		            let local_time_str = format!("{}", local_time);

		            let _important_information = json!({
		                "device_info": {
		                    "nickname": nickname,
		                    "device_id": device_id
		                },
		                "energy_usage": {
		                    "current_power": current_power,
		                    "local_time": local_time_str
		                }
		            });                    
	    
		            // Create the devices collection and insert a sample device with an "appliance" field
		            let devices: mongodb:: Collection<Document>  = client.database("TAICare").collection("Device");
		            println!("Collection found");
		            let existing_device = devices.find_one(doc! { "appliance": nickname }, None).await?;
		            println!("Device found or created");

		            let device_id = if let Some(device) = existing_device {
		                // If the device already exists, get its ID
		                device.get("_id").and_then(|id| id.as_object_id()).expect("Expected device to have an ObjectId").clone()
		            } else {
		                // If the device doesn't exist, insert it and get its new ID
		                let new_device = doc! {
		                    "appliance": nickname
		                };
		                let device_insert_result = devices.insert_one(new_device, None).await.expect("Failed to insert device.");
		                device_insert_result.inserted_id.as_object_id().expect("Retrieved _id should have been of type ObjectId").clone()
		            };
		            
		            println!("Working with device ID: {:?}", device_id);

		            // Create the data collection and insert sample data related to the above device
		            let data: mongodb:: Collection<Document>  = client.database("TAICare").collection("Data");
		            let new_data = doc! {
		                "power": current_power_i64,
		                "device_id": device_id,
		                "status": status,
		                "time": DateTime(current_time.into())
		            };
		            let data_insert_result = data.insert_one(new_data, None).await.expect("Failed to insert data.");

		            println!("Inserted data with ID: {:?}", data_insert_result.inserted_id);
		        
		            // Send data to Firebase
		            // println!("Publishing to Firebase...");
		            // let firebase_info = firebase.at("importantInformation");
		            // firebase_info.set(&important_information).await.map_err(|err| {
		            //     println!("{:?}", err);
		            //     std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", err))
		            // })?;
		            // println!("Published to Firebase!");
		        },
		        Err(e) => {
		            println!("Failed to create API client for a device: {}", e);
		        }
		    }
		}
	    	Ok::<(), Box<dyn std::error::Error>>(())
	}).await;

	match loop_result {
	    Ok(_) => println!("--------------------------------"),
	    Err(e) => println!("LOOP TIME OUT, RESET: {}", e),
	}
        thread::sleep(Duration::from_secs(5));
    }
}
