//! Defines a clone-on-write [`Value`].

use std::ops::Deref;

use crate::Value;

#[cfg_attr(test, derive(Debug))]
pub enum ValueCow<'a> {
    Borrowed(&'a Value),
    Owned(Value),
}

impl Deref for ValueCow<'_> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(v) => v,
            Self::Owned(v) => &*v,
        }
    }
}

impl<'a> ValueCow<'a> {
    #[cfg(feature = "filters")]
    pub fn take(&mut self) -> Value {
        match self {
            Self::Borrowed(v) => v.clone(),
            Self::Owned(v) => std::mem::take(v),
        }
    }
}
