use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum Visibility {
    Public,
    Private,
}