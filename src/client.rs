extern crate reqwest;

use entity::{AddRouteResp,
             AddServiceResp,
             ApiInfo,
             ConsumerDO,
             KongInfo,
             LegacyPluginAppliedType,
             ListApiResp,
             PluginInfo,
             PluginTarget,
             RouteInfo,
             RouteList,
             ServiceInfo,
             ServiceList};
use self::reqwest::StatusCode;
use serde_json::{Map as SerdeMap, Value};
use std::collections::BTreeMap;
use std::collections::HashMap;


pub struct KongApiClient<'t> {
    pub base_url: &'t str,
    client: reqwest::Client,
}

impl<'t> KongApiClient<'t> {
    pub fn build_with_url(kong_admin_url: &'t str) -> KongApiClient<'t> {
        return KongApiClient { base_url: kong_admin_url, client: reqwest::Client::new() };
    }

    pub fn get_node_info(&self) -> Result<KongInfo, reqwest::Error> {
        return self.client.get(&format!("{}/", self.base_url))
            .send()
            .and_then(|mut res| res.json::<KongInfo>());
    }

    /*********** services ****************/
    pub fn list_services(&self, offset: Option<String>) -> Result<ServiceList, reqwest::Error> {
        let list_srv_url = match offset {
            None => format!("{}/services", self.base_url),
            Some(offset) => format!("{}/services?offset={}", self.base_url, offset)
        };
        return self.client.get(&list_srv_url)
            .send()
            .and_then(|mut res| res.json::<ServiceList>());
    }

    pub fn delete_all_services(&self) {
        self._delete_service_batch(None);
    }

    pub fn _delete_service_batch(&self, next_offset: Option<String>) {
        let service_list = self.list_services(next_offset).unwrap();

        service_list.data.iter().for_each(|service_item| {
            self.delete_service(&service_item.id);
        });

        match service_list.offset {
            None => {}
            Some(next) => self._delete_service_batch(Some(next))
        }
    }

    pub fn delete_service(&self, service_id_or_name: &str) {
        match self.client.delete(&format!("{}/services/{}", self.base_url, service_id_or_name))
            .send() {
            Err(why) => error!("delete_service: {} using id={}", why, service_id_or_name),
            Ok(resp) => {
                if resp.status() == StatusCode::NoContent {
                    info!("service {} has removed!", service_id_or_name)
                } else if resp.status() == StatusCode::NotFound {
                    debug!("service {} not found, skip!", service_id_or_name)
                } else {
                    // TODO add body msg
                    error!("delete_service: {} using id={}", resp.status(), service_id_or_name)
                }
            }
        }
    }

    pub fn add_service(&self, payload: &ServiceInfo) -> Option<String> {
        let s_name = payload.get("name").unwrap();
        return match self.client.post(&format!("{}/services", self.base_url))
            .json(payload)
            .send() {
            Err(why) => {
                error!("add_service: {}", why);
                None
            }
            Ok(mut resp) => {
                if resp.status() == StatusCode::Created {
                    info!("Service {} has created/updated!", s_name);
                    resp.json::<AddServiceResp>()
                        .map(|obj| obj.id)
                        .ok()
                } else {
                    warn!("add_service: {}", resp.status());
                    None
                }
            }
        };
    }

    /*********** routes ****************/
    pub fn list_routes(&self, offset: Option<String>) -> Result<RouteList, reqwest::Error> {
        let list_route_url = match offset {
            None => format!("{}/routes", self.base_url),
            Some(offset) => format!("{}/routes?offset={}", self.base_url, offset)
        };
        return self.client.get(&list_route_url)
            .send()
            .and_then(|mut res| res.json::<RouteList>());
    }

    pub fn delete_all_routes(&self) {
        self._delete_route_batch(None);
    }

    pub fn _delete_route_batch(&self, next_offset: Option<String>) {
        let route_list = self.list_routes(next_offset).unwrap();

        route_list.data.iter().for_each(|route_item| {
            self.delete_route(&route_item.id);
        });

        match route_list.offset {
            None => {}
            Some(next) => self._delete_route_batch(Some(next))
        }
    }

    pub fn delete_route(&self, route_id: &str) {
        match self.client.delete(&format!("{}/routes/{}", self.base_url, route_id))
            .send() {
            Err(why) => error!("delete_route: {} using id={}", why, route_id),
            Ok(resp) => {
                if resp.status() == StatusCode::NoContent {
                    info!("route {} has removed!", route_id)
                } else if resp.status() == StatusCode::NotFound {
                    debug!("route {} not found, skip!", route_id)
                } else {
                    // TODO add body msg
                    error!("delete_route: {} using id={}", resp.status(), route_id)
                }
            }
        }
    }

    pub fn add_route_to_service(&self, service_id: String, mut route_info: RouteInfo) -> Option<String> {
        let route_cfg = &mut route_info.config;

        let mut silly_obj_map = SerdeMap::new();
        silly_obj_map.insert("id".to_string(), Value::String(service_id));
        route_cfg.insert("service".to_string(), Value::Object(silly_obj_map));

        match self.client.post(&format!("{}/routes", self.base_url))
            .json(&route_cfg)
            .send() {
            Err(why) => {
                error!("add_route: {}", why);
                return None;
            }
            Ok(mut resp) => {
                if resp.status() == StatusCode::Created {
                    info!("Route {} has created/updated!", route_info.name);
                    return resp.json::<AddRouteResp>()
                        .map(|obj| obj.id)
                        .ok();
                } else {
                    warn!("add_route: status={} {}", resp.status(), resp.text().unwrap());
                    return None;
                }
            }
        }
    }

    pub fn get_api_counts(&self) -> Result<i32, reqwest::Error> {
        return self.client.get(&format!("{}/apis", self.base_url))
            .send()
            .and_then(|mut res| res.json::<ListApiResp>())
            .map(|list_api_info| list_api_info.total);
    }

    pub fn delete_api(&self, api_name: &str) {
        match self.client.delete(&format!("{}/apis/{}", self.base_url, api_name))
            .send() {
            Err(why) => error!("delete_api: {}", why),
            Ok(resp) => {
                if resp.status() == StatusCode::NoContent {
                    info!("API {} has removed!", api_name)
                } else if resp.status() == StatusCode::NotFound {
                    debug!(" API {} not found, skip!", api_name)
                } else {
                    warn!("delete_api: {}", resp.status())
                }
            }
        }
    }

    pub fn upsert_api(&self, api_name: &str, payload: &ApiInfo) {
        match self.client.put(&format!("{}/apis", self.base_url))
            .json(payload)
            .send() {
            Err(why) => error!("upsert_api: {}", why),
            Ok(resp) => {
                if resp.status() == StatusCode::Created {
                    info!(" API {} has created/updated!", api_name)
                } else {
                    warn!("upsert_api: {}", resp.status())
                }
            }
        }
    }

    pub fn init_guest_consumer(&self, custom_id: &str) -> String {
        let payload = json!({
            "custom_id": custom_id,
            "username": custom_id
        });

        return match self.client.post(&format!("{}/consumers", self.base_url))
            .json(&payload)
            .send() {
            Err(why) => {
                error!("upsert_consumer: {}", why);
                String::from("error_id")
            }
            Ok(mut resp) => {
                return if resp.status() == StatusCode::Created {
                    info!("upsert_consumer: custom_id={} has created!", custom_id);
                    resp.json::<ConsumerDO>().unwrap().id
                } else if resp.status() == StatusCode::Conflict {
                    self.client.get(&format!("{}/consumers/{}", self.base_url, custom_id))
                        .send()
                        .and_then(|mut res| res.json::<ConsumerDO>())
                        .map(|c_info| c_info.id).unwrap()
                } else {
                    info!("upsert_consumer: unexpected status returned {}", resp.status());
                    String::from("error_id")
                };
            }
        };
    }

    pub fn add_consumer(&self, payload: &BTreeMap<String, String>) {
        let username = payload.get("username").unwrap();
        return match self.client.post(&format!("{}/consumers", self.base_url))
            .json(&payload)
            .send() {
            Err(why) => {
                error!("upsert_consumer: {}", why);
            }
            Ok(resp) => {
                return if resp.status() == StatusCode::Created {
                    info!("upsert_consumer: username={} has created!", username);
                } else if resp.status() == StatusCode::Conflict {
                    info!("upsert_consumer: username={} has existed! skip..", username);
                } else {
                    error!("upsert_consumer: unexpected status returned {}", resp.status());
                };
            }
        };
    }

    fn _apply_plugin_to_one(&self, plugin_type: &str, plugin_conf: &BTreeMap<String, String>, api_name: &str) {
        let mut json_payload = HashMap::new();
        json_payload.insert("name".to_string(), plugin_type.to_string());
        for (k, v) in plugin_conf.iter() {
            json_payload.insert(format!("config.{}", k), v.to_string());
        }

        match self.client.post(&format!("{}/apis/{}/plugins", self.base_url, api_name))
            .json(&json_payload)
            .send() {
            Err(why) => error!("apply_plugin_to_one: {}", why),
            Ok(resp) => {
                if resp.status() == StatusCode::Created || resp.status() == StatusCode::Conflict {
                    info!("succeed applying plugin {} to API {}", plugin_type, api_name)
                } else {
                    error!("_apply_plugin_to_one: {}", resp.status())
                }
            }
        }
    }

    fn _apply_plugin_to_all(&self, plugin_type: &str, plugin_conf: &BTreeMap<String, String>) {
        let mut json_payload = HashMap::new();
        json_payload.insert("name".to_string(), plugin_type.to_string());
        for (k, v) in plugin_conf.iter() {
            json_payload.insert(format!("config.{}", k), v.to_string());
        }

        match self.client.post(&format!("{}/plugins", self.base_url))
            .json(&json_payload)
            .send() {
            Err(why) => error!("apply_plugin_to_all: {}", why),
            Ok(resp) => {
                if resp.status() == StatusCode::Created || resp.status() == StatusCode::Conflict {
                    info!("succeed applying plugin {} to all API", plugin_type)
                } else {
                    error!("_apply_plugin_to_all: {}", resp.status())
                }
            }
        }
    }

    pub fn apply_plugin_to_api_legacy(&self, plugin_type: &str, target_apis: (LegacyPluginAppliedType, Option<Vec<String>>), plugin_conf: &BTreeMap<String, String>) {
        match target_apis {
            (LegacyPluginAppliedType::ALL, _) => self._apply_plugin_to_all(plugin_type, plugin_conf),
            (LegacyPluginAppliedType::SOME, Some(apis)) => {
                for api_name in apis {
                    self._apply_plugin_to_one(plugin_type, plugin_conf, &api_name)
                }
            }
            (_, _) => {}
        }
    }

    pub fn apply_plugin(&self, target: PluginTarget, plugin_conf: &PluginInfo) {
        let mut json_payload = HashMap::new();
        json_payload.insert("name".to_string(), Value::String(plugin_conf.name.clone()));
        json_payload.insert("enabled".to_string(), Value::Bool(plugin_conf.enabled));
        for (k, v) in plugin_conf.config.iter() {
            json_payload.insert(format!("config.{}", k), Value::String(v.to_string()));
        }

        match target {
            PluginTarget::GLOBAL => {
                let msg = &format!("applying plugin {} to Global", plugin_conf.name);
                self._apply_plugin(msg, &json_payload);
            }
            PluginTarget::SERVICES(services) => {
                services.iter().for_each(|s_id| {
                    let msg = &format!("applying plugin {} to service {}", plugin_conf.name, s_id);
                    json_payload.insert("service_id".to_string(), Value::String(s_id.clone()));
                    self._apply_plugin(msg, &json_payload);
                });
            }
            PluginTarget::Routes(routes) => {
                routes.iter().for_each(|r_id| {
                    let msg = &format!("applying plugin {} to route {}", plugin_conf.name, r_id);
                    json_payload.insert("route_id".to_string(), Value::String(r_id.clone()));
                    self._apply_plugin(msg, &json_payload);
                });
            }
        }
    }

    pub fn _apply_plugin(&self, target_desc: &str, payload: &HashMap<String, Value>) {
        match self.client.post(&format!("{}/plugins", self.base_url))
            .json(payload)
            .send() {
            Err(why) => error!("apply_plugin_to_one: {}", why),
            Ok(resp) => {
                if resp.status() == StatusCode::Created
                    || resp.status() == StatusCode::Conflict {
                    info!("{}", target_desc)
                } else {
                    error!("_apply_plugin: error {}", resp.status())
                }
            }
        }
    }
}


