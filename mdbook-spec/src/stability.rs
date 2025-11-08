//! Support for stability markers in markdown.
//!
//! These share some styling with admonitions, but use a fenced pair of HTML-like tags to avoid
//! putting the entirety of unstable text in a blockquote.
//!
//! ```markdown
//! <unstable-rust feature="name">
//! Text that documents the unstable Rust feature "name". Both the feature and the text documenting
//! it are unstable.
//! </unstable-rust>
//!
//! <unstable-text>
//! Text that documents stable Rust, but where the text itself is not yet considered stable.
//! </unstable-text>
//! ```

use crate::{Diagnostics, warn_or_err};
use mdbook::book::Chapter;
use regex::{Captures, Regex};
use std::sync::LazyLock;

/// The Regex for stability markers.
///
/// Note that this allows some invalid syntax, like attributes on an end tag, or features on an
/// `unstable-text` tag. The preprocessing handles those cases.
static STABILITY_MARKER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?m)^(?<space> *)<(?<end>/)?unstable-(?<ty>[^ >]+)(?: *feature *= *"(?<feature>[^"]*)")?>"#).unwrap()
});

/// Converts stability markers into admonition-like blockquotes.
///
/// This translates the HTML-like tags into a div wrapped around a blockquote, with an icon and
/// descriptive paragraph at the top.
pub fn preprocess_markers(chapter: &Chapter, diag: &mut Diagnostics) -> String {
    STABILITY_MARKER_RE
        .replace_all(&chapter.content, |caps: &Captures<'_>| {
            let space = &caps["space"];
            let end = caps.name("end").is_some();
            let ty = &caps["ty"];
            let feature = caps.name("feature").map(|f| f.as_str());

            match (end, ty, feature) {
                (true, _, Some(_)) => {
                    warn_or_err!(
                        diag,
                        "stability end marker in {:?} has attribute",
                        chapter.path.as_ref().unwrap()
                    );
                    "".to_string()
                }
                (false, "text", Some(_)) => {
                    warn_or_err!(
                        diag,
                        "Stability marker `unstable-text` in {:?} has a `feature` attribute.\n\
                        Use `unstable-rust` for unstable Rust features",
                        chapter.path.as_ref().unwrap()
                    );
                    "".to_string()
                }
                (false, "rust", None) => {
                    warn_or_err!(
                        diag,
                        "Stability marker `unstable-rust` in {:?} missing `feature` attribute.\n\
                        Usage: <unstable-rust feature=\"name\">",
                        chapter.path.as_ref().unwrap()
                    );
                    "".to_string()
                }
                (false, "text", None) => {
                    format!(
                        "{space}<div class=\"alert alert-unstable-{ty}\">\n\
                        {space}<blockquote>\n\
                        {space}<p class=\"alert-title\">Work in progress</p>\n\
                        \n",
                    )
                }
                (false, "rust", Some(feature)) => {
                    format!(
                        "{space}<div class=\"alert alert-unstable-{ty}\">\n\
                        {space}<blockquote>\n\
                        {space}<p class=\"alert-title\">Unstable Rust feature \"{feature}\"</p>\n\
                        \n",
                    )
                }
                (true, "text" | "rust", None) => format!("{space}</blockquote></div>"),
                (_, ty, _) => {
                    warn_or_err!(
                        diag,
                        "Unknown stability marker type `unstable-{ty}` in {:?}",
                        chapter.path.as_ref().unwrap()
                    );
                    "".to_string()
                }
            }
        })
        .to_string()
}
