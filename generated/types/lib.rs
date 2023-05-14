use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserSignedUpPayload {
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    ///baz
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///foo
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    ///bar
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}
impl From<&UserSignedUpPayload> for UserSignedUpPayload {
    fn from(value: &UserSignedUpPayload) -> Self {
        value.clone()
    }
}
