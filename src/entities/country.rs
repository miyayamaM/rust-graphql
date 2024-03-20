use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "country", rename_all = "lowercase")]
pub enum Country {
    Japan,
    Usa,
    China,
    Other,
}
