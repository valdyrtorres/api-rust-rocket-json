use crate::models::recurso::Recurso;
use crate::dtos::recurso_dto::RecursoDto;

pub fn lista_de_recursos() -> Vec<Recurso> {
    let recurso1 = Recurso {
        id: 1,
        titulo: String::from("Recurso 1"),
        descricao: String::from("Descrição do Recurso 1"),
    };

    let recurso2 = Recurso {
        id: 2,
        titulo: String::from("Recurso 2"),
        descricao: String::from("Descrição do Recurso 2"),
    };

    vec![recurso1, recurso2]
}

pub fn cadastrar_recurso(recurso_dto: RecursoDto) -> Result<Recurso, String> {

    println!("Título: {}", recurso_dto.titulo);
    println!("Descrição: {}", recurso_dto.descricao);

    if true {
        Ok( Recurso { id: 1, titulo: recurso_dto.titulo, descricao: recurso_dto.descricao })
    } else {
        Err("Falha ao cadastrar recurso".to_string())
    }
}

pub fn alterar_recurso(id:u32, recurso_dto: RecursoDto) -> Result<Recurso, String> {

    println!("id: {}", id);
    println!("Título: {}", recurso_dto.titulo);
    println!("Descrição: {}", recurso_dto.descricao);

    if recurso_dto.titulo.is_empty() {
        return Err("O título do recurso não pode ser vazio".to_string());
    }

    if true {
        Ok( Recurso { id: 1, titulo: recurso_dto.titulo, descricao: recurso_dto.descricao })
    } else {
        Err("Falha ao atualizar recurso".to_string())
    }
}

pub fn busca_por_id(id: u32) -> Recurso {
    println!("Buscando recurso com id: {}", id);
    Recurso {
        id: 1,
        titulo: String::from("Recurso Exemplo"),
        descricao: String::from("Descrição do Recurso Exemplo"),
    }
}

pub fn apagar_recurso_por_id(id: u32) -> Result<(), String> {

    println!("Id: {}", id);

    if true {
        Ok( () )
    } else {
        Err("Falha ao apagar recurso".to_string())
    }
}