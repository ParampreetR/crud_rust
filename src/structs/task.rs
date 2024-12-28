use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "task"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub title: String,
    pub info: String,
    pub is_done: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Title,
    Info,
    IsDone,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Title => ColumnType::String(StringLen::None).def(),
            Self::Info => ColumnType::String(StringLen::None).def(),
            Self::IsDone => ColumnType::Boolean.def(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// impl<'r> Responder<'r, 'static> for Model {
//     fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
//         let data = serde_json::to_string(&self).expect("unable to convert struct to json");
//         Response::build()
//             .header(ContentType::JSON)
//             .sized_body(data.len(), data)
//             .ok()
//     }
// }

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;

    fn auto_increment() -> bool {
        true
    }
}
