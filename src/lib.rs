#![warn(clippy::pedantic, clippy::cargo, clippy::nursery)]
#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]

//! This crate is based off of <https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f>

macro_rules! mod_use {
    ($z: ident, $y: ident) => {
        mod $z;
        pub use $z::$y;
    };
    ($z: ident) => {
        mod $z;
        pub use $z::$z;
    };
}

mod_use!(keyboard_identifier, KeyboardIdentifier);
mod_use!(language_id, LanguageID);
mod_use!(language_tag, LanguageTag);
mod_use!(list_keyboards);
