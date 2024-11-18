use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

impl Model {
    pub fn name(&self) -> String {
        [self.first_name.clone(), self.last_name.clone()].join(" ")
    }

    pub fn abbr(&self) -> String {
        let mut abbr = String::from("");

        if let Some(l) = self.first_name.chars().take(1).last() {
            abbr.push(l);
        }

        if let Some(l) = self.last_name.chars().take(1).last() {
            abbr.push(l);
        }

        abbr
    }

    pub fn color(&self) -> String {
        random_color::RandomColor::new()
            .seed(self.id.to_string())
            .to_hsla_string()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
