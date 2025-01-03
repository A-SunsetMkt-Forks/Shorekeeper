use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropValueData {
    pub id: i32,
    pub value: f32,
    pub is_ratio: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VectorData([f32; 3]);

impl VectorData {
    pub fn get_x(&self) -> f32 {
        self.0[0]
    }

    pub fn get_y(&self) -> f32 {
        self.0[1]
    }

    pub fn get_z(&self) -> f32 {
        self.0[2]
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RawVectorData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl RawVectorData {
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_z(&self) -> f32 {
        self.z
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntranceEntityData {
    pub dungeon_id: i32,
    pub entrance_entity_id: i32,
}
