use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Player {
    uuid: Uuid,
    name: String,
}
