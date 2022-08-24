use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,
    pub permissions: Option<Permissions>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Permissions {
    #[serde(rename = "consentToken")]
    pub consent_token: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,
    pub permissions: Option<Permissions>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Unit {
    #[serde(rename = "unitId")]
    pub unit_id: String,
    #[serde(rename = "persistentUnitId")]
    pub persistent_unit_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person{
    #[serde(rename = "personId")]
    pub person_id: String,
    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,
}