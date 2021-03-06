/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! The `@namespace` at-rule.

use cssparser::SourceLocation;
use shared_lock::{SharedRwLockReadGuard, ToCssWithGuard};
use std::fmt::{self, Write};
use str::CssStringWriter;
use {Namespace, Prefix};

/// A `@namespace` rule.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct NamespaceRule {
    /// The namespace prefix, and `None` if it's the default Namespace
    pub prefix: Option<Prefix>,
    /// The actual namespace url.
    pub url: Namespace,
    /// The source location this rule was found at.
    pub source_location: SourceLocation,
}

impl ToCssWithGuard for NamespaceRule {
    // https://drafts.csswg.org/cssom/#serialize-a-css-rule CSSNamespaceRule
    fn to_css(&self, _guard: &SharedRwLockReadGuard, dest: &mut CssStringWriter) -> fmt::Result {
        dest.write_str("@namespace ")?;
        if let Some(ref prefix) = self.prefix {
            dest.write_str(&*prefix.to_string())?;
            dest.write_str(" ")?;
        }

        // FIXME(emilio): Pretty sure this needs some escaping, or something?
        dest.write_str("url(\"")?;
        dest.write_str(&*self.url.to_string())?;
        dest.write_str("\");")
    }
}
