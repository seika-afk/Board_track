use serde::{Deserialize,Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct GameModel{
    pub id : Uuid,
    pub name : String,
    pub creator : String,
    pub plays : i32,
    pub created_at : Option<chrono::DateTime<chrono::Utc>>,


}
