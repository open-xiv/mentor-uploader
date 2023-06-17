use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Instance {
    pub id: u32,
    pub name: String,
    pub level: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub id: u32,
    pub name: String,
    pub gauge: String, // tank, healer, melee, physical, magical, unknown
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub instances: Vec<Instance>,
    pub jobs: Vec<Job>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Oper {
    pub op_code: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Area {
    pub op: Oper,
    pub instance: Instance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub op: Oper,
    pub id: String,
    pub name: String,
    pub job: Job,
    pub level: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FightRecord {
    pub area: Area,
    pub players: Vec<Player>,
    pub pretty: bool,
    pub useful: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotionConfig {
    pub block_id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuConfig {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CeresConfig {
    pub notion: NotionConfig,
    pub su: SuConfig,
}
