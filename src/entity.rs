use std::collections::BTreeMap;

use serde_json::{Value};
pub type ApiInfo = BTreeMap<String, String>;
pub type ServiceInfo = BTreeMap<String, String>;
pub type ConsumerInfo = BTreeMap<String, String>;

#[derive(Debug, Deserialize, PartialEq)]
pub struct LegacyKongConf {
    pub apis: Vec<ApiInfo>,
    pub plugins: Vec<LegacyPluginInfo>,
    pub consumers: Vec<BTreeMap<String, String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct KongConf {
    pub services: Vec<ServiceInfo>,
    pub routes: Vec<RouteInfo>,
    pub plugins: Vec<PluginInfo>,
    pub consumers: Vec<BTreeMap<String, String>>,
    pub credentials: Vec<CredentialsInfo>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RouteInfo {
    pub name: String,
    pub apply_to: String,
    pub config: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct LegacyPluginInfo {
    pub name: String,
    pub plugin_type: String,
    pub target_api: String,
    pub config: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PluginInfo {
    pub name: String,
    pub target: String,

    #[serde(default)]
    pub config: BTreeMap<String, String>,
    pub enabled: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CredentialsInfo {
    pub name: String,
    pub target: String,

    #[serde(default)]
    pub config: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct ConsumerDO {
    pub custom_id: Option<String>,
    pub id: String,
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListApiResp {
    pub total: i32,
    pub next: Option<String>,
    pub offset: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddServiceResp {
    pub id: String,
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct AddRouteResp {
    pub id: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KongInfo {
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct ServiceList {
    pub data: Vec<ServiceItem>,
    pub next: Option<String>,
    pub offset: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RouteList {
    pub data: Vec<RouteItem>,
    pub next: Option<String>,
    pub offset: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PluginList {
    pub total: i32,
    pub data: Vec<PluginItem>,
    pub offset: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ServiceItem {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RouteItem {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct PluginItem {
    pub id: String,
    pub name: String,
}

pub enum LegacyPluginAppliedType {
    ALL,
    NONE,
    SOME,
}

pub enum PluginTarget {
    GLOBAL,
    SERVICES(Vec<String>),
    Routes(Vec<String>),
}

pub enum ConfFileStyle {
    Suggested(KongConf), // services + routes + plugins
    Legacy(LegacyKongConf), // apis + plugins
    IllegalFormat { msg: String } // mixed or missing necessary field
}