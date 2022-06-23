/*
 * Murdock API
 *
 * This is the Murdock API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InformationOfTheCommitToProcess {
    #[serde(rename = "sha", skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(rename = "tree", skip_serializing_if = "Option::is_none")]
    pub tree: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}

impl InformationOfTheCommitToProcess {
    pub fn new() -> InformationOfTheCommitToProcess {
        InformationOfTheCommitToProcess {
            sha: None,
            tree: None,
            message: None,
            author: None,
        }
    }
}

