use serde::{Deserialize, Serialize};

/// Tax regime information (Simples Nacional and MEI)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRegime {
    pub cnpj: String,
    pub opcao_simples: Option<bool>,
    pub data_opcao_simples: Option<String>,
    pub data_exclusao_simples: Option<String>,
    pub opcao_mei: Option<bool>,
    pub data_opcao_mei: Option<String>,
    pub data_exclusao_mei: Option<String>,
}

impl TaxRegime {
    pub fn new(cnpj: String) -> Self {
        Self {
            cnpj,
            opcao_simples: None,
            data_opcao_simples: None,
            data_exclusao_simples: None,
            opcao_mei: None,
            data_opcao_mei: None,
            data_exclusao_mei: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_regime_creation() {
        let regime = TaxRegime::new("12345678000195".to_string());
        assert_eq!(regime.cnpj, "12345678000195");
        assert_eq!(regime.opcao_simples, None);
        assert_eq!(regime.opcao_mei, None);
    }
}
