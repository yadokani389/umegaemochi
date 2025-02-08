#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub id: uuid::Uuid,
    pub text: String,
    pub completed: bool,
}
