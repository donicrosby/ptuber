use thiserror::Error;

#[derive(Error, Debug)]
pub enum SfmlError {
    #[error("sfml load resource")]
    SfmlResource(#[from] sfml::ResourceLoadError),
    #[error("image path conversion to str")]
    PathConversion
}