use serde::{Deserialize, Serialize};

/// Partner (Sócio) data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partner {
    pub cnpj: String,
    pub identificador_socio: Option<i32>,
    pub nome_socio: String,
    pub cnpj_cpf_socio: String,
    pub codigo_qualificacao_socio: Option<i32>,
    pub qualificacao_socio: Option<String>,
    pub data_entrada_sociedade: Option<String>,
    pub codigo_pais: Option<i32>,
    pub pais: Option<String>,
    pub cpf_representante_legal: String,
    pub nome_representante_legal: String,
    pub codigo_qualificacao_representante_legal: Option<i32>,
    pub qualificacao_representante_legal: Option<String>,
    pub codigo_faixa_etaria: Option<i32>,
    pub faixa_etaria: Option<String>,
}

impl Partner {
    /// Parse identifier type
    pub fn parse_identificador_socio(code: i32) -> Option<&'static str> {
        match code {
            1 => Some("PESSOA JURÍDICA"),
            2 => Some("PESSOA FÍSICA"),
            3 => Some("ESTRANGEIRO"),
            _ => None,
        }
    }
    
    /// Parse age range description
    pub fn parse_faixa_etaria(code: i32) -> Option<&'static str> {
        match code {
            1 => Some("0 a 12 anos"),
            2 => Some("13 a 20 anos"),
            3 => Some("21 a 30 anos"),
            4 => Some("31 a 40 anos"),
            5 => Some("41 a 50 anos"),
            6 => Some("51 a 60 anos"),
            7 => Some("61 a 70 anos"),
            8 => Some("71 a 80 anos"),
            9 => Some("Maiores de 80 anos"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_identificador_socio() {
        assert_eq!(Partner::parse_identificador_socio(1), Some("PESSOA JURÍDICA"));
        assert_eq!(Partner::parse_identificador_socio(2), Some("PESSOA FÍSICA"));
        assert_eq!(Partner::parse_identificador_socio(99), None);
    }

    #[test]
    fn test_parse_faixa_etaria() {
        assert_eq!(Partner::parse_faixa_etaria(3), Some("21 a 30 anos"));
        assert_eq!(Partner::parse_faixa_etaria(9), Some("Maiores de 80 anos"));
        assert_eq!(Partner::parse_faixa_etaria(99), None);
    }
}
