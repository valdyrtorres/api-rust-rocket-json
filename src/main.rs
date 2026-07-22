#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Recurso {
    id: u32,
    titulo: String,
    descricao: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Home {
    mensagem: String,
    endpoints: Vec<String>,
}

#[get("/")]
fn home() -> Json<Home> {
    Json(Home {
        mensagem: "Bem-vindo à API!".to_string(),
        endpoints: vec![
            "/recursos".to_string()
        ]
    })
}

#[get("/recursos")]
fn recurso_index() -> Json<Vec<Recurso>> {
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            home, 
            recurso_index])
}