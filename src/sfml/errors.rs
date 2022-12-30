use crate::WindowFinderError;
use core::result;
use sfml::ResourceLoadError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SfmlError {
    #[error("sfml load resource")]
    SfmlResource(#[from] ResourceLoadError),
    #[error("image path conversion to str")]
    PathConversion,
    #[error("window finder error")]
    WindowFinder(#[from] WindowFinderError),
}

pub type SfmlResult<T> = result::Result<T, SfmlError>;
