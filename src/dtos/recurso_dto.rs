use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RecursoDto {
    pub titulo: String,
    pub descricao: String,
}