use std::{env, thread, time::Duration, net::TcpStream};
use log::{LevelFilter};
use tapo::{requests::EnergyDataInterval, ApiClient, P110};
use time::{macros::{time},macros::{date}, OffsetDateTime};
use paho_mqtt::{Client, CreateOptionsBuilder, Message};
use firebase_rs::*;
use serde_json::json;
use tapo::responses::{DeviceUsageResult,EnergyDataResult,EnergyUsageResult,PlugDeviceInfoResult};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip_prefix = "192.168.";
    let start_ip = 1;
    let end_ip = 255;
    let ip_sufix = ".236";/* 
    let mut ip_address = String::new(); */
/* 
    for i in start_ip..=end_ip {
        let ip = format!("{}{}{}", ip_prefix, i, ip_sufix);

        match TcpStream::connect(format!("{}:9999", ip)) {
            Ok(_stream) => {
                // Connection succeeded, so this is likely a TAPO P110
                println!("Found TAPO P110 at IP address {}", ip);
                ip_address = format!("{}{}{}", ip_prefix, i, ip_sufix);
                break; 
            }
            Err(_) => {
                // Connection failed, so this is not a TAPO P110
                println!("No TAPO P110 found.");
            }
        }
    } */
    let today = OffsetDateTime::now_utc();

    let firebase = Firebase::new("https://taicare-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();

    let log_level = env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .parse()
        .unwrap_or(LevelFilter::Info);

    pretty_env_logger::formatted_timed_builder()
        .filter(Some("tapo"), log_level)
        .init();
    
    let ip_address = env::var("IP_ADDRESS")?;
    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;

    let device = ApiClient::<P110>::new(ip_address.to_string(), tapo_username.to_string(), tapo_password.to_string(), true).await?;

    // Conectar al broker MQTT
    let create_options = CreateOptionsBuilder::new()
        .server_uri("tcp://127.0.0.1:1883")
        .client_id("tapo-client")
        .finalize();

    let client = Client::new(create_options)?;

    client.connect(None)?;

    loop{
/*         let device_usage = device.get_device_usage().await?;
        let device_usage_message = Message::new("tapo/device_usage", serde_json::to_string(&device_usage)?.as_str(), 0);
        let firebase_device_usage = firebase.at("deviceUsage");
        firebase_device_usage.set::<DeviceUsageResult>(&device_usage).await.map_err(|err| println!("{:?}", err)).ok();
        client.publish(device_usage_message)?; */

/*         let device_info = device.get_device_info().await?;
        let device_info_message = Message::new("tapo/device_info", serde_json::to_string(&device_info)?.as_str(), 0);
        let firebase_device_info = firebase.at("deviceInfo");
        firebase_device_info.set::<PlugDeviceInfoResult>(&device_info).await.map_err(|err| println!("{:?}", err)).ok();
        client.publish(device_info_message)?;

        let energy_usage = device.get_energy_usage().await?;
        let energy_usage_message = Message::new("tapo/energy_usage", serde_json::to_string(&energy_usage)?.as_str(), 0);
        let firebase_energy_usage = firebase.at("energyUsage");
        firebase_energy_usage.set::<EnergyUsageResult>(&energy_usage).await.map_err(|err| println!("{:?}", err)).ok();
        client.publish(energy_usage_message)?; */

        // Retrieve device information and energy usage
        println!("Fetching device info...");
        let device_info = device.get_device_info().await?;
        println!("Device info fetched successfully!");
        
        println!("Fetching energy usage...");
        let energy_usage = device.get_energy_usage().await?;
        println!("Energy usage fetched successfully!");
        
        // Combine them into a single structured JSON object
        let important_information = json!({
            "device_info": device_info,
            "energy_usage": energy_usage
        });
        
        // Prepare the message for MQTT
        let important_information_str = serde_json::to_string(&important_information)?;
        let important_information_message = Message::new("tapo/important_information", important_information_str.as_bytes(), 0);
        
        // Send data to Firebase
        println!("Publishing to Firebase...");
        let firebase_important_information = firebase.at("importantInformation");
        firebase_important_information.set(&important_information).await.map_err(|err| {
            println!("{:?}", err);
            std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", err))
        })?;
        println!("Published to Firebase!");
        
        // Publish the message to MQTT
        println!("Publishing to MQTT...");
        client.publish(important_information_message)?; 
        println!("Published to MQTT!");       
        
/*         let energy_data_hourly = device
            .get_energy_data(EnergyDataInterval::Hourly {
                start_datetime: today,
                end_datetime: today.replace_time(time!(23:59)),
            })
            .await?;
        let energy_data_hourly_message = Message::new("tapo/energy_data_hourly", serde_json::to_string(&energy_data_hourly)?.as_str(), 0);
        let firebase_energy_data_hourly = firebase.at("energyDataHourly");
        firebase_energy_data_hourly.set::<EnergyDataResult>(&energy_data_hourly).await.map_err(|err| println!("{:?}", err)).ok();
        client.publish(energy_data_hourly_message)?;

        let energy_data_daily = device
            .get_energy_data(EnergyDataInterval::Daily {
                start_date: date!(2023-01-01),
            })
            .await?;
        let energy_data_daily_message = Message::new("tapo/energy_data_daily", serde_json::to_string(&energy_data_daily)?.as_str(), 0);
        let firebase_energy_data_daily = firebase.at("energyDataDaily");
        firebase_energy_data_daily.set::<EnergyDataResult>(&energy_data_daily).await.map_err(|err| println!("{:?}", err)).ok();
        client.publish(energy_data_daily_message)?;

        let energy_data_monthly = device
            .get_energy_data(EnergyDataInterval::Monthly {
                start_date: date!(2023-01-01),
            })
            .await?;
        let energy_data_monthly_message = Message::new("tapo/energy_data_monthly", serde_json::to_string(&energy_data_monthly)?.as_str(), 0);
        let firebase_energy_data_monthly = firebase.at("energyDataMonthly");
        firebase_energy_data_monthly.set::<EnergyDataResult>(&energy_data_monthly).await.map_err(|err| println!("{:?}", err)).ok();
        client.publish(energy_data_monthly_message)?;
 */
        thread::sleep(Duration::from_secs(5));
    }
}
