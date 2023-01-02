use thiserror::Error;

cfg_if::cfg_if! {
    if #[cfg(all(unix, target_os="linux"))] {
        #[derive(Error, Debug)]
        pub enum LinuxFinderError {
            #[error("x11rb connect")]
            X11Connect(#[from] x11rb::rust_connection::ConnectError),
            #[error("x11rb connection")]
            X11Connection(#[from] x11rb::rust_connection::ConnectionError),
            #[error("x11rb Reply error")]
            X11Error(#[from] x11rb::errors::ReplyError),
        }
        pub type WindowFinderError = LinuxFinderError;
    } else if #[cfg(windows)] {
        #[derive(Error, Debug)]
        pub enum WindowsFinderError {
            #[error("windows monitor invalid")]
            WindowsMonitorInvalid
        }
        pub type WindowFinderError = WindowsFinderError;
    }
}
