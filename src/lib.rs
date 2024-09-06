use serde_json::Value;
use std::collections::HashMap;

pub fn list_lights(bridge_ip: &String, bridge_user: &String) -> anyhow::Result<()> {
    let response =
        reqwest::blocking::get(&format!("http://{}/api/{}/lights", bridge_ip, bridge_user))?;

    let lights: HashMap<String, Value> = response.json()?;
    for (id, light) in lights {
        let name = light["name"].as_str().unwrap();
        println!("{}: {}", id, name);
    }
    Ok(())
}
