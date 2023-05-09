//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
pub enum Status {
    #[sea_orm(string_value = "Draft")]
    Draft,
    #[sea_orm(string_value = "Ready")]
    Ready,
    #[sea_orm(string_value = "Doing")]
    Doing,
    #[sea_orm(string_value = "Done")]
    Done,
}