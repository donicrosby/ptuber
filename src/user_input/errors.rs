use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilError {
    #[error("registering mouse event callback")]
    MouseCallbackRegister,
    #[error("registering mouse event callback")]
    KeyboardCallbackRegister,
}
