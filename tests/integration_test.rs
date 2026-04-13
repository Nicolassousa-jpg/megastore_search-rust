use std::collections::HashMap;

#[test]
fn teste_busca() {
    let mut produtos = HashMap::new();
    produtos.insert("Mouse", 150);

    assert_eq!(produtos.get("Mouse"), Some(&150));
}

#[test]
fn teste_nao_encontrado() {
    let mut produtos = HashMap::new();
    produtos.insert("Mouse", 150);

    assert_eq!(produtos.get("Teclado"), None);
}

#[test]
fn teste_filtro_preco() {
    let valores = vec![50, 100, 200];

    let resultado: Vec<i32> = valores.into_iter().filter(|v| *v >= 100).collect();

    assert_eq!(resultado, vec![100, 200]);
}