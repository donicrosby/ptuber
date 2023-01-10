use cfg_if::cfg_if;
use log::error;
use ptuber::{PTuber, PtuberResult as Result};
#[cfg(windows)]
use windows::Win32::System::Console::{AttachConsole, FreeConsole, ATTACH_PARENT_PROCESS};

cfg_if! {
    if #[cfg(windows)] {
        fn attach_to_console() {
            unsafe {
                FreeConsole();
                AttachConsole(ATTACH_PARENT_PROCESS);
            }
        }
    }

}

fn main() -> Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            attach_to_console();
        }
    }
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
