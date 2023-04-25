//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

#[cfg(target_family = "wasm")]
use std::fmt::Display;

#[cfg(not(target_family = "wasm"))]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg_attr(
    not(target_family = "wasm"),
    derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        EnumIter,
        DeriveActiveEnum,
        Serialize,
        Deserialize
    )
)]
#[cfg_attr(
    target_family = "wasm",
    derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)
)]
#[cfg_attr(
    not(target_family = "wasm"),
    sea_orm(rs_type = "String", db_type = "Enum", enum_name = "genre")
)]
pub enum Genre {
    #[cfg_attr(not(target_family = "wasm"), sea_orm(string_value = "metal"))]
    Metal,
    #[cfg_attr(not(target_family = "wasm"), sea_orm(string_value = "classical"))]
    Classical,
    #[cfg_attr(not(target_family = "wasm"), sea_orm(string_value = "jazz"))]
    Jazz,
    #[cfg_attr(not(target_family = "wasm"), sea_orm(string_value = "pop"))]
    Pop,
    #[cfg_attr(not(target_family = "wasm"), sea_orm(string_value = "rock"))]
    Rock,
}

#[cfg(target_family = "wasm")]
impl Display for Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Metal => write!(f, "Metal"),
            Self::Classical => write!(f, "Classical"),
            Self::Jazz => write!(f, "Jazz"),
            Self::Pop => write!(f, "Pop"),
            Self::Rock => write!(f, "Rock"),
        }
    }
}
