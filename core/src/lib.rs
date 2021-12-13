#![cfg_attr(feature = "nightly", feature(specialization))]
//! Core structs and traits for paperclip.

#[cfg(feature = "actix2")]
extern crate actix_web2 as actix_web;
#[cfg(feature = "actix3")]
extern crate actix_web3 as actix_web;
#[cfg(feature = "actix4")]
extern crate actix_web4 as actix_web;
#[cfg_attr(feature = "v2", macro_use)]
extern crate serde;

mod error;
pub mod im;
pub mod util;
#[cfg(feature = "v2")]
pub mod v2;
#[cfg(feature = "v3")]
pub mod v3;

pub use self::error::ValidationError;

#[cfg(all(feature = "actix2", feature = "actix3"))]
compile_error!("feature \"actix2\" and feature \"actix3\" cannot be enabled at the same time");

#[cfg(all(feature = "actix3", feature = "actix4"))]
compile_error!("feature \"\" and feature \"\" cannot be enabled at the same time");

#[cfg(all(feature = "actix2", feature = "actix4"))]
compile_error!("feature \"\" and feature \"\" cannot be enabled at the same time");

#[cfg(all(feature = "actix-multipart", any(feature = "actix2", feature = "actix3")))]
compile_error!("feature \"actix-multipart\" should go together with feature \"actix\" or \"actix4\"");

#[cfg(all(feature = "actix-session", any(feature = "actix2", feature = "actix3")))]
compile_error!("feature \"actix-session\" should go together with feature \"actix\" or \"actix4\"");

#[cfg(all(feature = "actix-files", any(feature = "actix2", feature = "actix3")))]
compile_error!("feature \"actix-files\" should go together with feature \"actix\" or \"actix4\"");

#[cfg(all(feature = "actix-multipart3", feature = "actix4"))]
compile_error!("feature \"actix-multipart3\" should go together with feature \"actix2\" or \"actix3\"");

#[cfg(all(feature = "actix-session3", feature = "actix4"))]
compile_error!("feature \"actix-session3\" should go together with feature \"actix2\" or \"actix3\"");

#[cfg(all(feature = "actix-files3", feature = "actix4"))]
compile_error!("feature \"actix-files3\" should go together with feature \"actix2\" or \"actix3\"");
