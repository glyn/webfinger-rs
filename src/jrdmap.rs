/*

The initial version of this file was largely a fork of
https://raw.githubusercontent.com/kenkeiras/webfinger-rs/master/src/resource/resource.rs
which is from an archived project licensed under Apache v2.
Apache v2 material can be mixed into a GPL project so long as the resultant project is
licensed under the GPL.

*/

use std::collections::hash_map::HashMap;
use std::option::Option;
use std::str::FromStr;

use http::uri::Uri;
use serde;
use serde_json;

/* A JrdMap maps string URIs to the JSON Resource Descriptors associated
with those URIs. */
pub type JrdMap = HashMap<String, Jrd>;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Jrd {

    // The value of the "subject" member is a URI that identifies the entity
    // that the JRD describes.
    pub subject: String,


    // The "aliases" array is an array of zero or more URI strings that
    // identify the same entity as the "subject" URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,


    // The "properties" object comprises zero or more name/value pairs whose
    // names are URIs (referred to as "property identifiers") and whose
    // values are strings or null. Properties are used to convey additional
    // information about the subject of the JRD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,


    // The "links" array has any number of member objects, each of which
    // represents a link [4].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ResourceLink>>,
}

impl Jrd {
    pub fn filter(&self, rel: String) -> Jrd {
        Jrd {
            subject: self.subject.clone(),
            aliases: self.aliases.clone(),
            properties: self.properties.clone(),
            links: self.links.clone().
                map(|lks| lks.into_iter().
                    filter(|lk| Uri::from_str(&lk.rel).unwrap() == Uri::from_str(&rel).unwrap()).
                    collect()
                )
        }
    }
}

    
#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ResourceLink {
    // Each of these link objects can have the following members:
    //         o rel
    //         o href
    //         o type
    //         o titles
    //         o properties

    // The value of the "rel" member is a string that is either a URI or a
    // registered relation ‘type’ (see RFC 5988).  The value of the
    // "rel" member MUST contain exactly one URI or registered relation
    // type.  The URI or registered relation type identifies the type of the
    // link relation.
    // The "rel" member MUST be present in the link relation object.
    pub rel: String,


    // The value of the "type" member is a string that indicates the media
    // type of the target resource (see RFC 6838).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,


    // The value of the "href" member is a string that contains a URI
    // pointing to the target resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,


    // The "titles" object comprises zero or more name/value pairs whose
    // names are a language tag [11] or the string "und".  The string is
    // human-readable and describes the link relation.  More than one title
    // for the link relation MAY be provided for the benefit of users who
    // utilize the link relation, and, if used, a language identifier SHOULD
    // be duly used as the name.  If the language is unknown or unspecified,
    // then the name is "und".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titles: Option<HashMap<String, String>>,


    // The "properties" object within the link relation object comprises
    // zero or more name/value pairs whose names are URIs (referred to as
    // "property identifiers") and whose values are strings or null.
    // Properties are used to convey additional information about the link
    // relation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
}


pub fn to_json(resource : &Jrd) -> String {
    serde_json::to_string(&resource).unwrap()
}


pub fn from_json(s : &String) -> JrdMap {
    serde_json::from_str(s).unwrap()
}
