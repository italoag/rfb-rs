use serde::{Deserialize, Serialize};

/// CNAE (Classificação Nacional de Atividades Econômicas) data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CNAE {
    pub codigo: i32,
    pub descricao: String,
}

impl CNAE {
    pub fn new(codigo: i32, descricao: String) -> Self {
        Self { codigo, descricao }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cnae_creation() {
        let cnae = CNAE::new(4751201, "Comércio varejista de produtos de panificação".to_string());
        assert_eq!(cnae.codigo, 4751201);
        assert!(cnae.descricao.contains("panificação"));
    }
}
