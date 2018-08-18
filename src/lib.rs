#[macro_use]
extern crate failure;

mod error;
mod utils;
mod envconfig_macro;

pub use error::Error;
pub use utils::load_var;
