#[macro_use]
mod protocol;

mod client;
mod unix;

pub use client::{ClientResult, ClientStatus};
pub use unix::new as new_unix;
