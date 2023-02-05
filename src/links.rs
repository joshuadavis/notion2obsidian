/// Turns an empty string into None, and a non-empty string into Some(s).
pub fn empty_is_none(s: &str) -> Option<&str> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// Format a wiki link, with optional link text.
pub fn fmt_wiki_link(dest: &str, text: Option<&str>) -> String {
    if let Some(text) = text {
        format!("[[{dest}|{text}]]") // Yeah, this is the reverse of markdown links.  Ugh.
    } else {
        format!("[[{dest}]]")
    }
}

/// Format a markdown-style link.  If there is no link text, just use the link address as it is.
pub fn fmt_md_link(dest: &str, text: Option<&str>) -> String {
    if let Some(text) = text {
        format!("[{text}]({dest})")
    } else {
        dest.to_string()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_empty_is_none() {
        assert_eq!(empty_is_none(""), None);
        assert_eq!(empty_is_none("foo"), Some("foo"));
    }

    #[test]
    fn test_wiki_link() {
        assert_eq!(fmt_wiki_link("foo", None), "[[foo]]");
        assert_eq!(fmt_wiki_link("foo", Some("bar")), "[[foo|bar]]");
    }

    #[test]
    fn test_md_link() {
        assert_eq!(fmt_md_link("foo", None), "foo");
        assert_eq!(fmt_md_link("foo", Some("bar")), "[bar](foo)");
    }
}
