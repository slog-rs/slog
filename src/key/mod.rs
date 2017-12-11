
#[cfg(all(feature = "dynamic-keys", feature = "std"))]
mod dynamic_std;
#[cfg(all(feature = "dynamic-keys", feature = "std"))]
pub use self::dynamic_std::Key;

#[cfg(not(feature = "dynamic-keys"))]
mod static_key;
#[cfg(not(feature = "dynamic-keys"))]
pub use self::static_key::Key;

#[cfg(all(feature = "dynamic-keys", not(feature = "std")))]
mod dynamic_nostd;
#[cfg(all(feature = "dynamic-keys", not(feature = "std")))]
pub use self::dynamic_nostd::Key;
