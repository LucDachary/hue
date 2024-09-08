use anyhow;
use log::info;
use reqwest;
use std::env;
use std::thread::sleep;
use std::time;

const BRIDGE_IP: &str = "192.168.1.14";
const USERNAME: &str = "G5yH6nGxpayxZjOiglI-99WeP5T9N0qvnC4FCEZV";

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("Running the testing program…");

    // Build a client with a proxy, if HTTP_PROXY and HTTPS_PROXY have been set.

    let client: reqwest::blocking::Client = match env::var("HTTP_PROXY") {
        Ok(http_proxy) => reqwest::blocking::Client::builder()
            .proxy(reqwest::Proxy::http(http_proxy)?)
            .build()?,
        Err(_) => {
            info!("No proxy set. Returning a regular client.");
            reqwest::blocking::Client::new()
        }
    };

    // sudo rustscan -b 200 -r 80-80 --addresses 192.168.1.0/24
    // -> 192.168.1.14
    // Press the bridge button, then run:
    // http 192.168.1.14/api devicetype=rust#hue
    //
    //[
    //    {
    //        "success": {
    //            "username": "G5yH6nGxpayxZjOiglI-99WeP5T9N0qvnC4FCEZV"
    //        }
    //    }
    //]

    // Now this gives me much information:
    // http $HUE/api/$HUE_USER/

    // List of light names:
    // http $HUE/api/$HUE_USER/lights  --print b  |  jq .[].name

    // Get only names and state with `jq`:
    // http $HUE/api/$HUE_USER/lights  | jq -r 'pick(.[].name, .[].state.on)'

    info!("Listing lights…");
    hue::list_lights(&client, &BRIDGE_IP.to_string(), &USERNAME.to_string())?;

    info!("Listing groups…");
    hue::list_groups(&client, &BRIDGE_IP.to_string(), &USERNAME.to_string())?;

    info!("Turning on light 1…");
    hue::turn_on_light(
        &client,
        &BRIDGE_IP.to_string(),
        &USERNAME.to_string(),
        &"1".to_string(),
    )?;

    sleep(time::Duration::from_secs(2));

    info!("Turning off light 1…");
    hue::turn_off_light(
        &client,
        &BRIDGE_IP.to_string(),
        &USERNAME.to_string(),
        &"1".to_string(),
    )?;

    Ok(())
}
