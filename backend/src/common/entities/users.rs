use sea_orm::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Text")]
    pub user_id: String,
    #[sea_orm(unique, column_type = "Text")]
    pub username: String,
    #[sea_orm(unqiue, column_type = "Text")]
    pub email: String,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    pub email_verified: bool,
    pub created_at: DateTimeUtc,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
