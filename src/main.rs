mod product;

use std::collections::HashMap;
use std::io;
use product::Produto;

fn main() {
    let mut produtos: HashMap<String, Produto> = HashMap::new();

    // Produtos
    let lista = vec![
        Produto::novo("Notebook", 3500.0, "Eletrônicos"),
        Produto::novo("Mouse Gamer", 150.0, "Eletrônicos"),
        Produto::novo("Teclado Mecânico", 250.0, "Eletrônicos"),
        Produto::novo("Camiseta", 80.0, "Roupas"),
        Produto::novo("Calça Jeans", 120.0, "Roupas"),
    ];

    for p in lista {
        produtos.insert(p.nome.clone(), p);
    }

    loop {
        println!("\n========= MENU =========");
        println!("1 - Buscar produto por nome");
        println!("2 - Buscar por categoria");
        println!("3 - Filtrar por preço");
        println!("4 - Listar todos");
        println!("0 - Sair");
        println!("Escolha uma opção:");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro");

        match opcao.trim() {
            "1" => buscar_nome(&produtos),
            "2" => buscar_categoria(&produtos),
            "3" => filtrar_preco(&produtos),
            "4" => listar_todos(&produtos),
            "0" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida"),
        }
    }
}

// 🔍 BUSCA POR NOME (PARCIAL)
fn buscar_nome(produtos: &HashMap<String, Produto>) {
    println!("Digite o nome do produto:");

    let mut busca = String::new();
    io::stdin().read_line(&mut busca).unwrap();
    let busca = busca.trim().to_lowercase();

    println!("\nResultados:");

    for produto in produtos.values() {
        if produto.nome.to_lowercase().contains(&busca) {
            println!(
                "Nome: {} | Preço: R${} | Categoria: {}",
                produto.nome, produto.preco, produto.categoria
            );
        }
    }
}

// 📂 BUSCA POR CATEGORIA
fn buscar_categoria(produtos: &HashMap<String, Produto>) {
    println!("Digite a categoria:");

    let mut categoria = String::new();
    io::stdin().read_line(&mut categoria).unwrap();
    let categoria = categoria.trim().to_lowercase();

    println!("\nResultados:");

    for produto in produtos.values() {
        if produto.categoria.to_lowercase() == categoria {
            println!(
                "Nome: {} | Preço: R${}",
                produto.nome, produto.preco
            );
        }
    }
}

// 💰 FILTRO POR PREÇO
fn filtrar_preco(produtos: &HashMap<String, Produto>) {
    println!("Digite o preço mínimo:");

    let mut preco = String::new();
    io::stdin().read_line(&mut preco).unwrap();

    let preco: f32 = preco.trim().parse().unwrap_or(0.0);

    println!("\nResultados:");

    for produto in produtos.values() {
        if produto.preco >= preco {
            println!(
                "Nome: {} | Preço: R${}",
                produto.nome, produto.preco
            );
        }
    }
}

// 📋 LISTAR TODOS
fn listar_todos(produtos: &HashMap<String, Produto>) {
    println!("\nLista de produtos:");

    for produto in produtos.values() {
        println!(
            "Nome: {} | Preço: R${} | Categoria: {}",
            produto.nome, produto.preco, produto.categoria
        );
    }
}