#[macro_use]
mod protocol;

mod client;
mod unix;

pub use client::{ClientResult, ClientStatus};
pub use unix::new_client as new_unix_client;
