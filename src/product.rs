#[derive(Debug)]
pub struct Produto {
    pub nome: String,
    pub preco: f32,
    pub categoria: String,
}

impl Produto {
    pub fn novo(nome: &str, preco: f32, categoria: &str) -> Produto {
        Produto {
            nome: nome.to_string(),
            preco,
            categoria: categoria.to_string(),
        }
    }
}