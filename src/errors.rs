use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScraperError {
  #[error("selector not found")]
  SelectorNotFound,
}