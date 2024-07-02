/*
Copyright 2024 Glyn Normington

This file is part of webfinger-rs.

webfinger-rs is free software: you can redistribute it and/or modify it under the terms
of the GNU General Public License as published by the Free Software Foundation, either
version 3 of the License, or (at your option) any later version.

webfinger-rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with webfinger-rs.
If not, see <https://www.gnu.org/licenses/>.
*/

use fluent_uri::Uri;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Rel{
    rel: String,
}

pub fn make_rel(v: String) -> Rel {
    Rel{rel: v.clone()}
}

impl PartialEq for Rel {
    fn eq(&self, other: &Self) -> bool {
        // Detect extension relation types to be URIs.
        if let Ok(self_uri_reference) = Uri::parse(self.rel.clone()) {
            if let Ok(other_uri_reference) = Uri::parse(other.rel.clone()) {
                if self_uri_reference.has_scheme() && other_uri_reference.has_scheme() {
                    eprintln!("comparing normalized URI references {} and {}", self_uri_reference.normalize(), other_uri_reference.normalize());
                    return self_uri_reference.normalize() == other_uri_reference.normalize()
                }
            }
        }

        // Compare registered relation types as case-insensitive strings.
        self.rel.eq_ignore_ascii_case(&other.rel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality_registered_strings_equal() {
        assert_eq!(make_rel("me".to_string()), make_rel("me".to_string()));
    }

    #[test]
    fn test_equality_registered_opposite_case_strings_equal() {
        // Note: registered link relation types must be lowercase
        assert_eq!(make_rel("me".to_string()), make_rel("ME".to_string()));
    }

    #[test]
    fn test_equality_registered_mixed_case_strings_equal() {
        // Note: registered link relation types must be lowercase
        assert_eq!(make_rel("mE".to_string()), make_rel("Me".to_string()));
    }

     #[test]
    fn test_equality_registered_and_extension() {
        assert_ne!(make_rel("me".to_string()), make_rel("http://example.com/reltype".to_string()));
    }

    #[test]
    fn test_equality_extension_strings_equal() {
        assert_eq!(make_rel("http://example.com/reltype".to_string()), make_rel("http://example.com/reltype".to_string()));
    }

    #[test]
    fn test_equality_extension_uri_scheme_normalization() {
        assert_eq!(make_rel("example://a".to_string()), make_rel("eXAMPLE://a".to_string()));
    }

    #[test]
    fn test_equality_extension_uri_dot_normalization() {
        assert_eq!(make_rel("example://a/b".to_string()), make_rel("example://a/./b".to_string()));
    }

    #[test]
    fn test_equality_extension_uri_dotdot_normalization() {
        assert_eq!(make_rel("example://a/b/../b".to_string()), make_rel("example://a/b".to_string()));
    }

    #[test]
    fn test_equality_extension_uri_escape_normalization() {
        assert_eq!(make_rel("example://a/%7Bfoo%7D".to_string()), make_rel("example://a/%7bfoo%7d".to_string()));
    }

    #[test]
    fn test_equality_extension_uri_optional_escape_normalization() {
        assert_eq!(make_rel("example://a/%62".to_string()), make_rel("example://a/b".to_string()));
    }

}