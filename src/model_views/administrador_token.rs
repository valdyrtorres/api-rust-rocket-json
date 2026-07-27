use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AdministradorToken {
     pub id: u64,
     pub nome: String,
     pub email: String,
     pub token: String,
 }