use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
    pub preco: f64,
}

pub struct Catalogo {
    pub produtos_por_id: HashMap<u32, Produto>,
    pub produtos_por_categoria: HashMap<String, Vec<u32>>, 
}

impl Catalogo {
    pub fn novo() -> Self {
        Catalogo {
            produtos_por_id: HashMap::new(),
            produtos_por_categoria: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        let id = produto.id;
        let categoria = produto.categoria.clone(); 

        self.produtos_por_id.insert(id, produto);

        self.produtos_por_categoria
            .entry(categoria)
            .or_insert_with(Vec::new)
            .push(id);
    }

    pub fn buscar_por_id(&self, id: u32) -> Option<&Produto> {
        self.produtos_por_id.get(&id)
    }

    pub fn buscar_por_categoria(&self, categoria: &str) -> Vec<&Produto> {
        let mut resultados = Vec::new(); 
        
        if let Some(ids_encontrados) = self.produtos_por_categoria.get(categoria) {
            for id in ids_encontrados {
                if let Some(produto) = self.produtos_por_id.get(id) {
                    resultados.push(produto);
                }
            }
        }
        
        resultados 
    }
}

fn main() {
    let mut meu_catalogo = Catalogo::novo();

    meu_catalogo.adicionar_produto(Produto { id: 1, nome: String::from("Teclado Mecânico"), categoria: String::from("Eletrônicos"), preco: 250.50 });
    meu_catalogo.adicionar_produto(Produto { id: 2, nome: String::from("Cadeira Ergonômica"), categoria: String::from("Móveis"), preco: 1200.00 });
    meu_catalogo.adicionar_produto(Produto { id: 3, nome: String::from("Mouse Gamer"), categoria: String::from("Eletrônicos"), preco: 150.00 });
    meu_catalogo.adicionar_produto(Produto { id: 4, nome: String::from("Mesa de Escritório"), categoria: String::from("Móveis"), preco: 800.00 });

    println!("--- Sistema MegaStore ---");
    
    let categoria_busca = "Eletrônicos";
    println!("Buscando produtos da categoria: '{}'...", categoria_busca);

    let produtos_encontrados = meu_catalogo.buscar_por_categoria(categoria_busca);

    if produtos_encontrados.is_empty() {
        println!("Nenhum produto encontrado nesta categoria.");
    } else {
        for produto in produtos_encontrados {
            println!("- [{}] {} (R$ {})", produto.id, produto.nome, produto.preco);
        }
    }
}

#[cfg(test)]
mod tests {
    
    use crate::{Catalogo, Produto}; 

    #[test]
    fn teste_adicionar_e_buscar_por_id() {
        let mut catalogo = Catalogo::novo();
      
        let produto = Produto { 
            id: 99, 
            nome: String::from("Monitor Ultrawide"), 
            categoria: String::from("Eletrônicos"), 
            preco: 2500.00 
        };
        
        catalogo.adicionar_produto(produto);

        let resultado = catalogo.buscar_por_id(99);
        
        assert!(resultado.is_some()); 
        assert_eq!(resultado.unwrap().nome, "Monitor Ultrawide");
    }

    #[test]
    fn teste_buscar_por_categoria() {
        let mut catalogo = Catalogo::novo();
        catalogo.adicionar_produto(Produto { id: 1, nome: String::from("Livro A"), categoria: String::from("Livros"), preco: 50.0 });
        catalogo.adicionar_produto(Produto { id: 2, nome: String::from("Livro B"), categoria: String::from("Livros"), preco: 60.0 });
        catalogo.adicionar_produto(Produto { id: 3, nome: String::from("Caneta"), categoria: String::from("Papelaria"), preco: 5.0 });

        let resultados = catalogo.buscar_por_categoria("Livros");
        
        assert_eq!(resultados.len(), 2);
    }
}
