use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct CreateProviderReq {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub api_base: Option<String>,
    #[serde(default)]
    pub api_key: Option<String>,
    pub enabled: bool,
}


#[derive(Default, Debug, Deserialize)]
pub struct UpdateProviderReq {
    pub name: Option<String>,
    #[serde(default)]
    pub api_base: Option<String>,
    #[serde(default)]
    pub api_key: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Default, Debug, Deserialize)]
pub struct CreateModelReq {
    pub id: String,
    pub name: String,
    
}


#[derive(Default, Debug, Deserialize)]
pub struct UpdateModelReq {
    pub name: Option<String>,
 
}


#[derive(Default, Debug, Deserialize)]
pub struct UpdateDefaultModelReq {
    pub id: String,
}