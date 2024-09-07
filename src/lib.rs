use anyhow::Context;
use serde_json::Value;
use std::collections::HashMap;

/// List all lights connected to the bridge.
pub fn list_lights(
    client: reqwest::blocking::Client,
    bridge_ip: &String,
    bridge_user: &String,
) -> anyhow::Result<()> {
    let response = client
        .get(&format!("http://{}/api/{}/lights", bridge_ip, bridge_user))
        .send()
        .with_context(|| format!("Failed to build the HTTP request."))?;

    let lights: HashMap<String, Value> = response
        .json()
        .with_context(|| "Failed to extract JSON from lights list.")?;

    for (id, light) in lights {
        let name = light["name"].as_str().unwrap();
        println!("{:>3}: {}", id, name);
    }
    Ok(())
}

/// List all groups configured on the bridge.
pub fn list_groups(
    client: reqwest::blocking::Client,
    bridge_ip: &String,
    bridge_user: &String,
) -> anyhow::Result<()> {
    let response = client
        .get(&format!("http://{}/api/{}/groups", bridge_ip, bridge_user))
        .send()
        .with_context(|| format!("Failed to build the HTTP request."))?;

    let groups: HashMap<String, Value> = response
        .json()
        .with_context(|| "Failed to extract JSON from groups list.")?;

    for (id, group) in groups {
        let name = group["name"].as_str().unwrap();
        println!("{:>3}: {}", id, name);
    }
    Ok(())
}

pub fn turn_on_light(
    client: reqwest::blocking::Client,
    bridge_ip: &String,
    bridge_user: &String,
    light_id: &String,
) -> anyhow::Result<()> {
    let response = client
        .put(&format!(
            "http://{}/api/{}/lights/{}/state",
            bridge_ip, bridge_user, light_id
        ))
        .json(&serde_json::json!({"on": true, "bri": 254}))
        .send()
        .with_context(|| format!("Failed to build the HTTP request."))?;

    let response_json: Value = response
        .json()
        .with_context(|| "Failed to extract JSON from response.")?;

    println!("{}", response_json);
    Ok(())
}

pub fn turn_off_light(
    client: reqwest::blocking::Client,
    bridge_ip: &String,
    bridge_user: &String,
    light_id: &String,
) -> anyhow::Result<()> {
    let response = client
        .put(&format!(
            "http://{}/api/{}/lights/{}/state",
            bridge_ip, bridge_user, light_id
        ))
        .json(&serde_json::json!({"on": false}))
        .send()
        .with_context(|| format!("Failed to build the HTTP request."))?;

    let response_json: Value = response
        .json()
        .with_context(|| "Failed to extract JSON from response.")?;

    println!("{}", response_json);
    Ok(())
}
