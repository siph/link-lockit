use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::Component;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Component)]
#[sea_orm(table_name = "links")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub links_id: i32,
    pub original: String,
    pub short: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
