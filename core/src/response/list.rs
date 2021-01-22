use std::collections::HashMap;

use super::deserialize_string_from_number;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum List {
    Succes {
        #[serde(rename = "continue")]
        querycontinue: Option<Continue>,
        query: Query,
    },
    Failure {
        errors: Vec<super::Error>,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct Continue {
    #[serde(
        alias = "accontinue",
        alias = "aicontinue",
        alias = "alcontinue",
        alias = "apcontinue",
        alias = "blcontinue",
        alias = "cmcontinue",
        alias = "eicontinue",
        alias = "iucontinue",
        alias = "eucontinue",
        alias = "qpoffset",
        alias = "sroffset",
        deserialize_with = "deserialize_string_from_number"
    )]
    pub(crate) from: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Query {
    #[serde(
        alias = "allcategories",
        alias = "allimages",
        alias = "allinfoboxes",
        alias = "alllinks",
        alias = "allpages",
        alias = "backlinks",
        alias = "categorymembers",
        alias = "embeddedimages",
        alias = "imageusage",
        alias = "exturlusage",
        alias = "search",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub(crate) pages: Vec<Page>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Page {
    #[serde(alias = "category")]
    pub(crate) title: String,
    // For exturlusage
    pub(crate) url: Option<String>,
}

// Special case Querypage...
#[derive(Debug, Deserialize)]
pub(crate) struct Querypage {
    #[serde(rename = "continue")]
    pub(crate) querycontinue: Option<Continue>,
    pub(crate) query: QPQuery,
}

#[derive(Debug, Deserialize)]
pub(crate) struct QPQuery {
    pub(crate) querypage: QPQuerypage,
}

#[derive(Debug, Deserialize)]
pub(crate) struct QPQuerypage {
    pub(crate) results: Vec<Page>,
}

// get namespaces for allpages
#[derive(Debug, Deserialize)]
pub(crate) struct Namespaces {
    pub(crate) query: NSQuery,
}
#[derive(Debug, Deserialize)]
pub(crate) struct NSQuery {
    pub(crate) namespaces: HashMap<String, Namespace>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Namespace {
    pub(crate) id: i32,
    pub(crate) name: String,
}
