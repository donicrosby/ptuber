use env_logger;
use ptuber::{PTuber, PtuberResult as Result};
use log::error;

fn main() -> Result<()> {
    env_logger::init();
    match PTuber::new() {
        Ok(ptuber) => {
            ptuber.start_ptuber().map_err(|err| {
                error!("Got Error (Run): {:?}", err); 
                err
        })?;
            Ok(())
        },
        Err(err) => {
            error!("Got Error: {:?}", err);
            Err(err)
        }
    }
}
