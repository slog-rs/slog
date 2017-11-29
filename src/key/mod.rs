
#[cfg(all(feature = "opaque-keys", feature = "std"))]
mod opaque_key_std;
#[cfg(all(feature = "opaque-keys", feature = "std"))]
pub use self::opaque_key_std::Key;

#[cfg(not(feature = "opaque-keys"))]
mod old_key;
#[cfg(not(feature = "opaque-keys"))]
pub use self::old_key::Key;

#[cfg(all(feature = "opaque-keys", not(feature = "std")))]
mod opaque_key_nostd;
#[cfg(all(feature = "opaque-keys", not(feature = "std")))]
pub use self::opaque_key_nostd::Key;

