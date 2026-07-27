use crate::model_views::administrador_token::AdministradorToken;
use crate::models::administrador::Administrador;
use crate::servicos::jwt_servico::gerar_token_jwt;

pub fn busca_administrador_por_email(_email:String, _senha:String) -> Option<Administrador> {
    // Simulando buscar o administrador no banco de dados
    println!("{}", _email);
    println!("{}", _senha);

    let administrador_encontrado  = _email == "valdir@teste.com" && _senha == "123456";

    if administrador_encontrado {
        Some(Administrador {
            id: 1,
            nome: "Valdir".to_string(),
            email: "valdir@teste.com".to_string(),
            senha: "123456".to_string(),
        })
    } else {
        None
    }
}

pub fn login(email:String, senha:String) -> Result<AdministradorToken, String> {
    // acesso aos recursos de DB para Administrador
    let adm = busca_administrador_por_email(email, senha);
    match adm {
        Some(adm) => Ok(
            AdministradorToken {
                id: adm.id,
                nome: adm.nome,
                email: adm.email,
                token: gerar_token_jwt(adm.id),
            }
        ),
        None => Err("Usuário e senha não encontrado".to_string()),
    }
}