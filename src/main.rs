use anyhow;
use log::{debug, info};

use hue::list_lights;

const BRIDGE_IP: &str = "192.168.1.14";
const USERNAME: &str = "G5yH6nGxpayxZjOiglI-99WeP5T9N0qvnC4FCEZV";

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("Running the testing program…");

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

    debug!("Listing lights…");
    list_lights(&BRIDGE_IP.to_string(), &USERNAME.to_string())?;

    Ok(())
}
