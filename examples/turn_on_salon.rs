use anyhow::Context;
/// Search for a group named "Séjour", and turn all its lights on.
use log::{debug, info};
use std::env;

use hue::Bridge;

const BRIDGE_IP: &str = "192.168.1.14";
const USERNAME: &str = "G5yH6nGxpayxZjOiglI-99WeP5T9N0qvnC4FCEZV";

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // TODO replace with automated discovery.
    let bridge = Bridge {
        ip: String::from("192.168.1.14"),
        user: String::from("G5yH6nGxpayxZjOiglI-99WeP5T9N0qvnC4FCEZV"),
    };

    println!("Running with the bridge {:#?}", bridge);
    dbg!(bridge);

    let client: reqwest::blocking::Client = match env::var("HTTP_PROXY") {
        Ok(http_proxy) => reqwest::blocking::Client::builder()
            .proxy(reqwest::Proxy::http(http_proxy)?)
            .build()?,
        Err(_) => {
            info!("No proxy set. Returning a regular client.");
            reqwest::blocking::Client::new()
        }
    };

    let groups =
        hue::list_groups(&client, &BRIDGE_IP, &USERNAME).context("Failed to get groups.")?;

    for group in groups {
        if group.name == "Séjour" {
            debug!("Browsing all lights of this group to turn them on…");
            for light_id in group.lights {
                info!("Turning on light {:?}…", light_id);
                hue::turn_on_light(&client, &BRIDGE_IP, &USERNAME, &light_id.as_str())?;
            }
        }
    }

    Ok(())
}
