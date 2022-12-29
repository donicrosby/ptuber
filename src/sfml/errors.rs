use thiserror::Error;
use sfml::ResourceLoadError;
use core::result;
use crate::WindowFinderError;

#[derive(Error, Debug)]
pub enum SfmlError {
    #[error("sfml load resource")]
    SfmlResource(#[from] ResourceLoadError),
    #[error("image path conversion to str")]
    PathConversion,
    #[error("window finder error")]
    WindowFinder(#[from] WindowFinderError)
}


pub type SfmlResult<T> = result::Result<T, SfmlError>;