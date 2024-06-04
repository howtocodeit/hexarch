/*!
    Module `newtypes` exposes commonly used newtypes shared throughout the application.
*/

use std::borrow::Borrow;
use std::fmt::{Display, Formatter};

use derive_more::{AsRef, Deref, From};

/// A [String] with leading and trailing whitespace removed.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, From, AsRef, Deref)]
pub struct TrimmedString(String);

impl TrimmedString {
    pub fn new(raw: &str) -> Self {
        Self(raw.trim().to_string())
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Borrow<str> for TrimmedString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Display for TrimmedString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
