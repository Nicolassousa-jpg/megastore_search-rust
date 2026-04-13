# 🛒 Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📌 Descrição

Este projeto implementa um sistema de busca otimizado para um catálogo de produtos utilizando a linguagem Rust.

O objetivo é simular um sistema de e-commerce capaz de realizar buscas rápidas, eficientes e escaláveis, utilizando estruturas de dados apropriadas.

---

## 🚀 Funcionalidades

* 🔍 Busca por nome (com correspondência parcial)
* 📂 Busca por categoria
* 💰 Filtro de produtos por preço mínimo
* 📋 Listagem completa de produtos
* ⚡ Busca eficiente utilizando HashMap

---

## 🧠 Tecnologias Utilizadas

* Rust
* Cargo
* HashMap (estrutura de dados principal)

---

## ⚙️ Como Executar o Projeto

### 1. Clonar o repositório

```bash
git clone https://github.com/Nicolassousa-jpg/megastore_search-rust.git
```

---

### 2. Acessar a pasta

```bash
cd megastore_search-rust
```

---

### 3. Executar o sistema

```bash
cargo run
```

---

## 🧪 Como Executar os Testes

```bash
cargo test
```

---

## 🏗️ Arquitetura do Sistema

O projeto foi estruturado da seguinte forma:

```text
src/
 ├── main.rs        # Lógica principal do sistema
 ├── product.rs     # Estrutura de dados Produto

tests/
 ├── integration_test.rs  # Testes do sistema
```

---

## 📊 Estruturas de Dados e Algoritmos

O sistema utiliza:

* HashMap → Para indexação eficiente dos produtos
* Busca com complexidade próxima de O(1)
* Filtros utilizando iteração sobre os dados

---

## ⚡ Desempenho e Escalabilidade

A utilização de HashMap permite:

* Acesso rápido aos produtos
* Boa performance mesmo com grandes volumes de dados
* Facilidade de expansão do sistema

---

## 📈 Possíveis Melhorias Futuras

* Persistência de dados (arquivo ou banco de dados)
* Interface gráfica ou web
* Sistema de recomendação
* Ordenação por preço e relevância

---

## 👨‍💻 Autor

Desenvolvido por Nicolas Sousa

---

## 📄 Licença

Este projeto é acadêmico e de uso educacional.
