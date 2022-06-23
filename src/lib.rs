#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use libc::{pid_t, pollfd, timespec, timeval, FILE};

#[cfg(feature = "use-bindgen")]
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[cfg(not(feature = "use-bindgen"))]
include!("generated.rs");
