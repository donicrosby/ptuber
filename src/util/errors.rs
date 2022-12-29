use thiserror::Error;

#[derive(Error, Debug)]
pub enum WindowFinderError {
    #[error("x11rb connect")]
    X11Connect(#[from] x11rb::rust_connection::ConnectError),
    #[error("x11rb connection")]
    X11Connection(#[from] x11rb::rust_connection::ConnectionError),
    #[error("x11rb Reply error")]
    X11Error(#[from] x11rb::errors::ReplyError),
    #[error("windows monitor invalid")]
    WindowsMonitorInvalid
}