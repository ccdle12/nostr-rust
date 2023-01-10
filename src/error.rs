use std::{fmt, time};

#[derive(Debug)]
pub enum Error {
    InvalidSignature(),
    SystemTimeError(std::time::SystemTimeError),
    Secp256k1Error(secp256k1::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidSignature() => write!(f, "Signature is invalid"),
            Error::SystemTimeError(ref message) => write!(f, "{}", message),
            Error::Secp256k1Error(ref message) => write!(f, "{}", message),
        }
    }
}

macro_rules! impl_error_conversions {
    ($($error_type:path => $error_variant:path),*) => {
        $(impl From<$error_type> for Error {
            fn from(err: $error_type) -> Error {
                $error_variant(err)
            }
        })*
    };
}

impl_error_conversions!(
    time::SystemTimeError => Error::SystemTimeError,
    secp256k1::Error => Error::Secp256k1Error
);

pub(crate) type Result<T> = std::result::Result<T, Error>;
