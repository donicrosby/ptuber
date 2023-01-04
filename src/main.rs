
#![windows_subsystem = "windows"]

use log::error;
use ptuber::{PTuber, PtuberResult as Result};

fn main() -> Result<()> {
    env_logger::init();
    match PTuber::new() {
        Ok(mut ptuber) => {
            ptuber.start_ptuber().map_err(|err| {
                error!("Got Error (Run): {:?}", err);
                err
            })?;
            Ok(())
        }
        Err(err) => {
            error!("Got Error: {:?}", err);
            Err(err)
        }
    }
}
