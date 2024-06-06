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

use http::Uri;

#[derive(Clone, Debug)]
pub struct Rel(String);

impl PartialEq for Rel {
    fn eq(&self, other: &Self) -> bool {
        // Detect extension relation types to be URIs.
        if let Ok(self_uri) = self.0.parse::<Uri>() {
            if let Ok(other_uri) = other.0.parse::<Uri>() {
                // Compare extension relation types using their URIs.
                return self_uri == other_uri;
            }
        }

        // Compare registered relation types as case-insensitive strings.
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality_registered_strings_equal() {
        assert_eq!(Rel("me".to_string()), Rel("me".to_string()));
    }

    #[test]
    fn test_equality_registered_opposite_case_strings_equal() {
        // Note: registered link relation types must be lowercase
        assert_eq!(Rel("me".to_string()), Rel("ME".to_string()));
    }

    #[test]
    fn test_equality_registered_mixed_case_strings_equal() {
        // Note: registered link relation types must be lowercase
        assert_eq!(Rel("mE".to_string()), Rel("Me".to_string()));
    }

     #[test]
    fn test_equality_registered_and_extension() {
        assert_ne!(Rel("me".to_string()), Rel("http://example.com/reltype".to_string()));
    }

    #[test]
    fn test_equality_extension_strings_equal() {
        assert_eq!(Rel("http://example.com/reltype".to_string()), Rel("http://example.com/reltype".to_string()));
    }

    #[test]
    fn test_equality_extension_uri_scheme_normalization() {
        assert_eq!(Rel("example://a".to_string()), Rel("eXAMPLE://a".to_string()));
    }

    // The following tests track restrictions or limitations in http::Uri. If any of these start failing,
    // they can be negated and moved above (and renamed).

    #[test]
    fn restriction_test_equality_extension_uri_dot_normalization() {
        assert_ne!(Rel("example://a/b".to_string()), Rel("example://a/./b".to_string()));
    }

    #[test]
    fn restriction_test_equality_extension_uri_dotdot_normalization() {
        assert_ne!(Rel("example://a/b/../b".to_string()), Rel("example://a/b".to_string()));
    }

    #[test]
    fn restriction_test_equality_extension_uri_escape_normalization() {
        assert_ne!(Rel("example://a/%7Bfoo%7D".to_string()), Rel("example://a/%7bfoo%7d".to_string()));
    }

    #[test]
    fn restriction_test_equality_extension_uri_optional_escape_normalization() {
        assert_ne!(Rel("example://a/%62".to_string()), Rel("example://a/b".to_string()));
    }

}