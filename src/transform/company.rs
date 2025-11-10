use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex for matching CPF pattern in MEI names: (\D)(\d{3})(\d{5})(\d{3})
    static ref CPF_REGEX: Regex = Regex::new(r"(\D)(\d{3})(\d{5})(\d{3})$").unwrap();
}

/// Company data structure matching the Federal Revenue format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub cnpj: String,
    pub identificador_matriz_filial: Option<i32>,
    pub descricao_identificador_matriz_filial: Option<String>,
    pub nome_fantasia: String,
    pub situacao_cadastral: Option<i32>,
    pub descricao_situacao_cadastral: Option<String>,
    pub data_situacao_cadastral: Option<String>,
    pub motivo_situacao_cadastral: Option<i32>,
    pub descricao_motivo_situacao_cadastral: Option<String>,
    pub nome_cidade_no_exterior: String,
    pub codigo_pais: Option<i32>,
    pub pais: Option<String>,
    pub data_inicio_atividade: Option<String>,
    pub cnae_fiscal: Option<i32>,
    pub cnae_fiscal_descricao: Option<String>,
    pub descricao_tipo_de_logradouro: String,
    pub logradouro: String,
    pub numero: String,
    pub complemento: String,
    pub bairro: String,
    pub cep: String,
    pub uf: String,
    pub codigo_municipio: Option<i32>,
    pub codigo_municipio_ibge: Option<i32>,
    pub municipio: Option<String>,
    pub ddd_telefone_1: String,
    pub ddd_telefone_2: String,
    pub ddd_fax: String,
    pub email: Option<String>,
    pub situacao_especial: String,
    pub data_situacao_especial: Option<String>,
    pub opcao_pelo_simples: Option<bool>,
    pub data_opcao_pelo_simples: Option<String>,
    pub data_exclusao_do_simples: Option<String>,
    pub opcao_pelo_mei: Option<bool>,
    pub data_opcao_pelo_mei: Option<String>,
    pub data_exclusao_do_mei: Option<String>,
    pub razao_social: String,
    pub codigo_natureza_juridica: Option<i32>,
    pub natureza_juridica: Option<String>,
    pub qualificacao_do_responsavel: Option<i32>,
    pub capital_social: Option<f32>,
    pub codigo_porte: Option<i32>,
    pub porte: Option<String>,
    pub ente_federativo_responsavel: String,
}

impl Company {
    /// Parse situacao cadastral and return description
    pub fn parse_situacao_cadastral(code: i32) -> Option<&'static str> {
        match code {
            1 => Some("NULA"),
            2 => Some("ATIVA"),
            3 => Some("SUSPENSA"),
            4 => Some("INAPTA"),
            8 => Some("BAIXADA"),
            _ => None,
        }
    }

    /// Parse identificador matriz/filial and return description
    pub fn parse_matriz_filial(code: i32) -> Option<&'static str> {
        match code {
            1 => Some("MATRIZ"),
            2 => Some("FILIAL"),
            _ => None,
        }
    }
    
    /// Clean company name for privacy (masks CPF in MEI names)
    pub fn clean_name(name: &str) -> String {
        // Replace middle digits with ***
        CPF_REGEX.replace(name, "$1***$3***").trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_situacao_cadastral() {
        assert_eq!(Company::parse_situacao_cadastral(2), Some("ATIVA"));
        assert_eq!(Company::parse_situacao_cadastral(8), Some("BAIXADA"));
        assert_eq!(Company::parse_situacao_cadastral(99), None);
    }

    #[test]
    fn test_parse_matriz_filial() {
        assert_eq!(Company::parse_matriz_filial(1), Some("MATRIZ"));
        assert_eq!(Company::parse_matriz_filial(2), Some("FILIAL"));
        assert_eq!(Company::parse_matriz_filial(3), None);
    }

    #[test]
    fn test_clean_name() {
        let name = "JOAO SILVA 12345678901";
        let cleaned = Company::clean_name(name);
        assert!(cleaned.contains("***"));
    }
}
