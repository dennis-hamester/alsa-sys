#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::useless_transmute)]

#[allow(dead_code)]
mod bindings {
    use libc::{pid_t, pollfd, timespec, timeval, FILE};

    #[cfg(feature = "use-bindgen")]
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));

    #[cfg(not(feature = "use-bindgen"))]
    include!("generated.rs");
}

pub use bindings::*;

#[cfg(target_endian = "little")]
#[doc = " Signed 16 bit CPU endian"]
pub const SND_PCM_FORMAT_S16: _snd_pcm_format = SND_PCM_FORMAT_S16_LE;

#[cfg(target_endian = "little")]
#[doc = " Unsigned 16 bit CPU endian"]
pub const SND_PCM_FORMAT_U16: _snd_pcm_format = SND_PCM_FORMAT_U16_LE;

#[cfg(target_endian = "little")]
#[doc = " Signed 24 bit CPU endian"]
pub const SND_PCM_FORMAT_S24: _snd_pcm_format = SND_PCM_FORMAT_S24_LE;

#[cfg(target_endian = "little")]
#[doc = " Unsigned 24 bit CPU endian"]
pub const SND_PCM_FORMAT_U24: _snd_pcm_format = SND_PCM_FORMAT_U24_LE;

#[cfg(target_endian = "little")]
#[doc = " Signed 32 bit CPU endian"]
pub const SND_PCM_FORMAT_S32: _snd_pcm_format = SND_PCM_FORMAT_S32_LE;

#[cfg(target_endian = "little")]
#[doc = " Unsigned 32 bit CPU endian"]
pub const SND_PCM_FORMAT_U32: _snd_pcm_format = SND_PCM_FORMAT_U32_LE;

#[cfg(target_endian = "little")]
#[doc = " Float 32 bit CPU endian"]
pub const SND_PCM_FORMAT_FLOAT: _snd_pcm_format = SND_PCM_FORMAT_FLOAT_LE;

#[cfg(target_endian = "little")]
#[doc = " Float 64 bit CPU endian"]
pub const SND_PCM_FORMAT_FLOAT64: _snd_pcm_format = SND_PCM_FORMAT_FLOAT64_LE;

#[cfg(target_endian = "little")]
#[doc = " IEC-958 CPU Endian"]
pub const SND_PCM_FORMAT_IEC958_SUBFRAME: _snd_pcm_format = SND_PCM_FORMAT_IEC958_SUBFRAME_LE;

#[cfg(target_endian = "little")]
#[doc = " Signed 20bit in 4bytes format, LSB justified, CPU Endian"]
pub const SND_PCM_FORMAT_S20: _snd_pcm_format = SND_PCM_FORMAT_S20_LE;

#[cfg(target_endian = "little")]
#[doc = " Unsigned 20bit in 4bytes format, LSB justified, CPU Endian"]
pub const SND_PCM_FORMAT_U20: _snd_pcm_format = SND_PCM_FORMAT_U20_LE;

#[cfg(target_endian = "big")]
#[doc = " Signed 16 bit CPU endian"]
pub const SND_PCM_FORMAT_S16: _snd_pcm_format = SND_PCM_FORMAT_S16_BE;

#[cfg(target_endian = "big")]
#[doc = " Unsigned 16 bit CPU endian"]
pub const SND_PCM_FORMAT_U16: _snd_pcm_format = SND_PCM_FORMAT_U16_BE;

#[cfg(target_endian = "big")]
#[doc = " Signed 24 bit CPU endian"]
pub const SND_PCM_FORMAT_S24: _snd_pcm_format = SND_PCM_FORMAT_S24_BE;

#[cfg(target_endian = "big")]
#[doc = " Unsigned 24 bit CPU endian"]
pub const SND_PCM_FORMAT_U24: _snd_pcm_format = SND_PCM_FORMAT_U24_BE;

#[cfg(target_endian = "big")]
#[doc = " Signed 32 bit CPU endian"]
pub const SND_PCM_FORMAT_S32: _snd_pcm_format = SND_PCM_FORMAT_S32_BE;

#[cfg(target_endian = "big")]
#[doc = " Unsigned 32 bit CPU endian"]
pub const SND_PCM_FORMAT_U32: _snd_pcm_format = SND_PCM_FORMAT_U32_BE;

#[cfg(target_endian = "big")]
#[doc = " Float 32 bit CPU endian"]
pub const SND_PCM_FORMAT_FLOAT: _snd_pcm_format = SND_PCM_FORMAT_FLOAT_BE;

#[cfg(target_endian = "big")]
#[doc = " Float 64 bit CPU endian"]
pub const SND_PCM_FORMAT_FLOAT64: _snd_pcm_format = SND_PCM_FORMAT_FLOAT64_BE;

#[cfg(target_endian = "big")]
#[doc = " IEC-958 CPU Endian"]
pub const SND_PCM_FORMAT_IEC958_SUBFRAME: _snd_pcm_format = SND_PCM_FORMAT_IEC958_SUBFRAME_BE;

#[cfg(target_endian = "big")]
#[doc = " Signed 20bit in 4bytes format, LSB justified, CPU Endian"]
pub const SND_PCM_FORMAT_S20: _snd_pcm_format = SND_PCM_FORMAT_S20_BE;

#[cfg(target_endian = "big")]
#[doc = " Unsigned 20bit in 4bytes format, LSB justified, CPU Endian"]
pub const SND_PCM_FORMAT_U20: _snd_pcm_format = SND_PCM_FORMAT_U20_BE;
