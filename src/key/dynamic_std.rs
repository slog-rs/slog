use std::borrow::Cow;
use std::cmp::PartialEq;
use std::convert::{AsRef, From, Into};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::{FromIterator, IntoIterator};
use std::ops::Deref;
use std::str::FromStr;
use std::string::String;

/// Opaque Key is a representation of a key.
///
/// It is owned, and largely forms a contract for
/// key to follow.
pub struct Key {
    data: Cow<'static, str>,
}
impl Key {
    ///
    pub fn as_str(&self) -> &str {
        self.data.as_ref()
    }

    pub fn as_ref(&self) -> &str {
        self.as_str()
    }

    pub fn into_string(&self) -> String {
        match &self.data {
            &Cow::Borrowed(ptr) => String::from(ptr),
            &Cow::Owned(ref ptr) => ptr.clone(),
        }
    }

    pub fn into_owned(&self) -> String {
        match &self.data {
            &Cow::Borrowed(ptr) => String::from(ptr),
            &Cow::Owned(ref ptr) => ptr.clone(),
        }
    }
}

impl Default for Key {
    fn default() -> Key {
        Key {
            data: Cow::Borrowed(""),
        }
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.data {
            Cow::Borrowed(ref ptr) => ptr.hash(state),
            Cow::Owned(ref ptr) => ptr.hash(state),
        }
    }
}

impl Clone for Key {
    fn clone(&self) -> Key {
        match self.data {
            Cow::Borrowed(ptr) => Key::from(ptr),
            Cow::Owned(ref ptr) => Key::from(ptr.clone()),
        }
    }
}

impl From<&'static str> for Key {
    #[inline(always)]
    fn from(data: &'static str) -> Key {
        Key {
            data: Cow::Borrowed(data),
        }
    }
}

impl From<String> for Key {
    #[inline(always)]
    fn from(data: String) -> Key {
        Key {
            data: Cow::Owned(data),
        }
    }
}
impl Into<String> for Key {
    #[inline(always)]
    fn into(self) -> String {
        self.data.into_owned()
    }
}
impl FromIterator<char> for Key {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Key {
        Key {
            data: Cow::Owned(iter.into_iter().collect::<String>()),
        }
    }
}
impl<'a> FromIterator<&'a char> for Key {
    fn from_iter<I: IntoIterator<Item = &'a char>>(iter: I) -> Key {
        Key {
            data: Cow::Owned(iter.into_iter().collect::<String>()),
        }
    }
}
impl FromIterator<String> for Key {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Key {
        Key {
            data: Cow::Owned(iter.into_iter().collect::<String>()),
        }
    }
}
impl<'a> FromIterator<&'a String> for Key {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> Key {
        Key {
            data: Cow::Owned(
                iter.into_iter().map(|x| x.as_str()).collect::<String>(),
            ),
        }
    }
}
impl<'a> FromIterator<&'a str> for Key {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Key {
        Key {
            data: Cow::Owned(iter.into_iter().collect::<String>()),
        }
    }
}
impl<'a> FromIterator<Cow<'a, str>> for Key {
    fn from_iter<I: IntoIterator<Item = Cow<'a, str>>>(iter: I) -> Key {
        Key {
            data: Cow::Owned(iter.into_iter().collect::<String>()),
        }
    }
}
impl PartialEq<str> for Key {
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        self.as_ref().eq(other)
    }
}
impl PartialEq<String> for Key {
    #[inline(always)]
    fn eq(&self, other: &String) -> bool {
        self.as_ref().eq(other.as_str())
    }
}
impl PartialEq<Self> for Key {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}
impl AsRef<str> for Key {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a str {
        self.data.as_ref()
    }
}
impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.data {
            Cow::Borrowed(ref ptr) => write!(f, "{}", ptr),
            Cow::Owned(ref ptr) => write!(f, "{}", ptr),
        }
    }
}
impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.data {
            Cow::Borrowed(ref ptr) => write!(f, "{:?}", ptr),
            Cow::Owned(ref ptr) => write!(f, "{:?}", ptr),
        }
    }
}
