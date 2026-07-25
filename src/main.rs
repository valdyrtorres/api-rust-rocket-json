#[macro_use] extern crate rocket;

mod controllers;
mod model_views;
mod models;
mod servicos;
pub mod dtos;

use controllers::{ home_controller, recursos_controller };

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
            home_controller::index, 
            recursos_controller::index,
            recursos_controller::criar,
            recursos_controller::alterar,
            recursos_controller::mostrar,
            recursos_controller::excluir,
    ])
}