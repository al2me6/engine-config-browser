use std::collections::HashMap;

use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Display)]
#[serde(deny_unknown_fields)]
pub enum EngineType {
    Liquid,
    Electric,
    Solid,
    Unknown,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Gimbal {
    pub is_gimbaled: bool,
    pub range: f32,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ignition {
    pub number: i32,
    pub resources: LinkedHashMap<String, f32>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Reliability {
    pub rated_burn_time: i32,
    pub ignition_reliability_start: f32,
    pub ignition_reliability_end: f32,
    pub cycle_reliability_start: f32,
    pub cycle_reliability_end: f32,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EngineConfig {
    pub config_name: String,
    pub config_description: String,
    pub max_thrust: f32,
    pub min_thrust: f32,
    pub mass_mult: f32,
    pub ullage: bool,
    pub pressure_fed: bool,
    pub isp_vacuum: f32,
    pub isp_sea_level: f32,
    pub min_throttle: f32,
    pub air_lightable: bool,
    pub ignition: Ignition,
    pub propellants: LinkedHashMap<String, f32>,
    pub reliability: Reliability,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Engine {
    pub file_name: String,
    pub title: String,
    pub manufacturer: String,
    pub description: Option<String>,
    pub original_mass: f32,
    pub literal_zero_ignitions: bool,
    pub engine_type: EngineType,
    pub default_config: String,
    pub gimbal: Gimbal,
    pub engine_configs: LinkedHashMap<String, EngineConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineDatabase {
    pub commit: String,
    pub timestamp: String,
    pub engines: HashMap<String, Engine>,
}
