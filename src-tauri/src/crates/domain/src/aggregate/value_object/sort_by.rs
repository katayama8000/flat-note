use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortBy {
    CreatedAt,
    UpdatedAt,
}
