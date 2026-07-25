use rocket::serde::json::Json;
use crate::models::recurso::Recurso;

#[get("/recursos")]
pub fn index() -> Json<Vec<Recurso>> {
    let recursos = vec![
        Recurso {
            id: 1,
            titulo: String::from("Recurso 1"),
            descricao: String::from("Descrição do recurso 1"),
        },
        Recurso {
            id: 2,
            titulo: String::from("Recurso 2"),
            descricao: String::from("Descrição do recurso 2"),
        },
    ];
    Json(recursos)
}