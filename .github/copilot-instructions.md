# Diretrizes para GitHub Copilot e Modelos de IA - Projeto RFB-RS

## 📋 Visão Geral

Este documento define as diretrizes e práticas obrigatórias para o GitHub Copilot e outros modelos de IA ao interagir com o projeto **RFB-RS** - uma implementação em Rust para ETL e API de dados da Receita Federal Brasileira.

> **Importante**: Estas diretrizes devem ser seguidas rigorosamente em todas as interações com o código. Cada sugestão, modificação ou geração de código deve estar em conformidade com estas práticas.

---

## 🦀 Padrões de Código Rust

### Convenções de Nomenclatura

- **Módulos e arquivos**: `snake_case` (ex: `federal_revenue.rs`, `tax_regime.rs`)
- **Structs e Enums**: `PascalCase` (ex: `Company`, `DownloadConfig`)
- **Funções e métodos**: `snake_case` (ex: `download_file`, `check_integrity`)
- **Constantes**: `SCREAMING_SNAKE_CASE` (ex: `MAX_RETRIES`, `DEFAULT_PORT`)
- **Variáveis e parâmetros**: `snake_case` (ex: `data_dir`, `skip_existing`)
- **Traits**: `PascalCase` com nomes descritivos (ex: `Database`, `Transformer`)

### Estrutura de Módulos

```rust
// Ordem recomendada em arquivos .rs
// 1. Imports externos (std, crates)
use std::path::Path;
use tokio::fs::File;
use anyhow::Result;

// 2. Imports internos (módulos do projeto)
use crate::transform::Company;

// 3. Constantes e tipos
const MAX_RETRIES: u32 = 3;
type Result<T> = std::result::Result<T, Error>;

// 4. Structs e Enums
pub struct DownloadConfig { ... }

// 5. Implementações
impl DownloadConfig { ... }

// 6. Traits
pub trait Downloader { ... }

// 7. Funções públicas
pub fn download_file(...) -> Result<()> { ... }

// 8. Funções privadas
fn validate_url(...) -> bool { ... }

// 9. Testes (sempre ao final)
#[cfg(test)]
mod tests { ... }
```

### Formatação de Código

- **Sempre** executar `cargo fmt` antes de finalizar alterações
- Linha máxima: 100 caracteres (configuração padrão do rustfmt)
- Usar espaçamento consistente conforme rustfmt
- Documentação com `///` para itens públicos e `//` para comentários internos

---

## ✅ Validações Obrigatórias

### Antes de Sugerir Código

1. **Verificar compilação**: O código sugerido deve compilar sem erros
2. **Verificar tipos**: Garantir type-safety e compatibilidade de tipos
3. **Verificar ownership**: Respeitar as regras de ownership e borrowing do Rust
4. **Verificar lifetimes**: Usar lifetimes explícitos quando necessário

### Após Cada Modificação

```bash
# Verificação de compilação
cargo check

# Verificação de linting
cargo clippy --all-targets --all-features -- -D warnings

# Formatação
cargo fmt

# Testes unitários
cargo test --lib

# Testes de integração
cargo test --test '*'
```

### Checklist de Validação

- [ ] Código compila sem erros (`cargo check`)
- [ ] Sem warnings do clippy (`cargo clippy -- -D warnings`)
- [ ] Código formatado (`cargo fmt --check`)
- [ ] Testes passando (`cargo test`)
- [ ] Documentação atualizada se necessário
- [ ] Sem código morto ou imports não utilizados

---

## 🧪 Práticas de Testes

### Estrutura de Testes

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Testes unitários com nomes descritivos
    #[test]
    fn test_nome_funcao_cenario_esperado() {
        // Arrange
        let input = ...;
        
        // Act
        let result = funcao(input);
        
        // Assert
        assert_eq!(result, expected);
    }
    
    // Testes com casos de erro
    #[test]
    fn test_funcao_com_entrada_invalida_retorna_erro() {
        let result = funcao(entrada_invalida);
        assert!(result.is_err());
    }
    
    // Testes assíncronos
    #[tokio::test]
    async fn test_async_funcao() {
        let result = async_funcao().await;
        assert!(result.is_ok());
    }
}
```

### Tipos de Testes Requeridos

1. **Testes Unitários** (`src/*/mod.rs`): Para funções e métodos individuais
2. **Testes de Integração** (`tests/`): Para fluxos completos
3. **Testes de Snapshot**: Para validar estruturas de dados complexas
4. **Property-based Tests**: Para validação de invariantes (quando aplicável)

### Cobertura de Testes

- Novas funcionalidades **devem** ter testes correspondentes
- Bugs corrigidos **devem** ter testes de regressão
- Cobertura mínima esperada: 80% para código novo

---

## 🔒 Segurança

### Práticas Obrigatórias

1. **Validação de Entrada**
   ```rust
   // Sempre validar dados externos
   fn validate_cnpj(cnpj: &str) -> Result<(), ValidationError> {
       if cnpj.len() != 14 || !cnpj.chars().all(|c| c.is_ascii_digit()) {
           return Err(ValidationError::InvalidCnpj);
       }
       Ok(())
   }
   ```

2. **Tratamento de Erros Seguro**
   ```rust
   // Usar Result e Option corretamente
   // EVITAR: unwrap() em código de produção
   // PREFERIR: tratamento explícito com ? ou match
   let file = File::open(path)?; // Propagação segura
   ```

3. **Gerenciamento de Segredos**
   - Nunca hardcode credenciais ou tokens
   - Usar variáveis de ambiente para dados sensíveis
   - Seguir padrões do arquivo `.env.example`

4. **SQL Injection Prevention**
   ```rust
   // SEMPRE usar queries parametrizadas com Diesel
   // NUNCA concatenar strings para formar queries SQL
   diesel::insert_into(companies)
       .values(&new_company)
       .execute(conn)?;
   ```

5. **Path Traversal Prevention**
   ```rust
   // Validar caminhos de arquivo
   fn safe_path_join(base: &Path, user_input: &str) -> Result<PathBuf> {
       let path = base.join(user_input);
       if !path.starts_with(base) {
           return Err(Error::PathTraversal);
       }
       Ok(path)
   }
   ```

### Verificação de Dependências

```bash
# Auditar vulnerabilidades conhecidas
cargo audit

# Verificar dependências desatualizadas
cargo outdated
```

---

## ⚡ Performance

### Diretrizes de Performance

1. **Uso de Iteradores**
   ```rust
   // PREFERIR: iteradores lazy
   let result: Vec<_> = data
       .iter()
       .filter(|x| x.is_valid())
       .map(|x| x.transform())
       .collect();
   
   // EVITAR: loops com push em vetores
   ```

2. **Alocações de Memória**
   ```rust
   // Pré-alocar quando o tamanho é conhecido
   let mut vec = Vec::with_capacity(expected_size);
   
   // Usar &str ao invés de String quando possível
   fn process(data: &str) -> Result<()> { ... }
   ```

3. **Operações Assíncronas**
   ```rust
   // Usar tokio para I/O assíncrono
   // Evitar bloqueio em contextos async
   use tokio::fs::File;
   
   // Para operações CPU-bound em contexto async
   tokio::task::spawn_blocking(|| {
       expensive_computation()
   }).await?;
   ```

4. **Polars DataFrames**
   ```rust
   // Usar lazy evaluation para DataFrames
   let df = LazyCsvReader::new(path)
       .finish()?
       .filter(col("status").eq(lit("ATIVA")))
       .select([col("cnpj"), col("nome")])
       .collect()?;
   ```

### Benchmarking

Para código crítico de performance, utilize a crate `criterion` que é estável e recomendada para benchmarks em Rust:

```toml
# Adicionar ao Cargo.toml em [dev-dependencies]
criterion = "0.5"

[[bench]]
name = "transform_benchmark"
harness = false
```

```rust
// benches/transform_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_transform_company(c: &mut Criterion) {
    let test_data = setup_test_data();
    
    c.bench_function("transform_company", |b| {
        b.iter(|| transform_company(black_box(&test_data)))
    });
}

criterion_group!(benches, bench_transform_company);
criterion_main!(benches);
```

```bash
# Executar benchmarks
cargo bench
```

---

## 📝 Documentação

### Documentação de Código

```rust
/// Descrição breve da função.
///
/// Descrição detalhada explicando o comportamento,
/// casos de uso e qualquer consideração importante.
///
/// # Arguments
///
/// * `param1` - Descrição do primeiro parâmetro
/// * `param2` - Descrição do segundo parâmetro
///
/// # Returns
///
/// Descrição do valor retornado.
///
/// # Errors
///
/// Descreve quando e quais erros podem ocorrer.
///
/// # Examples
///
/// ```rust
/// use rfb_rs::download::Downloader;
///
/// let downloader = Downloader::new(config);
/// let result = downloader.download().await?;
/// ```
///
/// # Panics
///
/// Descreve condições que podem causar panic (se aplicável).
pub fn funcao_publica(param1: Tipo1, param2: Tipo2) -> Result<ReturnType> {
    // implementação
}
```

### Comentários TODO e FIXME

```rust
// TODO(usuario): Descrição da tarefa pendente
// FIXME(usuario): Descrição do bug a ser corrigido
// SAFETY: Justificativa para código unsafe
// PERF: Nota sobre decisões de performance
```

---

## 🔄 Tratamento de Erros

### Padrão de Erros

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Falha ao baixar arquivo: {url}")]
    DownloadFailed { url: String, #[source] source: reqwest::Error },
    
    #[error("Arquivo corrompido: {path}")]
    CorruptedFile { path: PathBuf },
    
    #[error("URL inválida: {0}")]
    InvalidUrl(String),
}
```

### Propagação de Erros

```rust
// Usar anyhow para aplicações e thiserror para bibliotecas
use anyhow::{Context, Result};

fn process_file(path: &Path) -> Result<Data> {
    let content = fs::read_to_string(path)
        .context(format!("Falha ao ler arquivo: {}", path.display()))?;
    
    parse_content(&content)
        .context("Falha ao processar conteúdo")
}
```

---

## 🏗️ Arquitetura do Projeto

### Módulos Principais

| Módulo | Responsabilidade |
|--------|------------------|
| `download/` | Download de arquivos da Receita Federal |
| `transform/` | Transformação e processamento de dados |
| `db/` | Operações de banco de dados com PostgreSQL |
| `api/` | Servidor REST API com Actix-web |

### Dependências Entre Módulos

```
main.rs → lib.rs → { download, transform, db, api }
                         ↓           ↓
                      (HTTP)    (PostgreSQL)
```

### Padrões de Design

- **Builder Pattern**: Para configuração de structs complexas
- **Repository Pattern**: Para acesso a dados
- **Result Type**: Para tratamento de erros
- **Async/Await**: Para operações I/O

---

## 🔄 Processos de Desenvolvimento

### Ciclo de Desenvolvimento

1. **Antes de Codificar**
   - Entender o requisito completamente
   - Verificar se já existe código similar no projeto
   - Seguir convenções existentes

2. **Durante a Codificação**
   - Escrever testes primeiro (TDD quando aplicável)
   - Fazer commits pequenos e frequentes
   - Usar mensagens de commit convencionais

3. **Após a Codificação**
   - Executar suite completa de testes
   - Revisar código com clippy
   - Atualizar documentação

### Mensagens de Commit

```bash
# Formato: <tipo>(escopo): descrição

# Tipos válidos:
feat:     # Nova funcionalidade
fix:      # Correção de bug
docs:     # Documentação
style:    # Formatação (não altera lógica)
refactor: # Refatoração
perf:     # Melhoria de performance
test:     # Adição/modificação de testes
chore:    # Manutenção geral

# Exemplos:
feat(download): adicionar suporte a downloads paralelos
fix(api): corrigir validação de CNPJ para casos especiais
docs: atualizar README com instruções de Docker
```

---

## 🚫 Práticas Proibidas

### Nunca Fazer

1. **Código**
   - ❌ Usar `unwrap()` ou `expect()` em código de produção
   - ❌ Ignorar erros com `let _ = ...`
   - ❌ Usar `unsafe` sem justificativa documentada
   - ❌ Criar funções com mais de 50 linhas
   - ❌ Ter mais de 3 níveis de indentação

2. **Segurança**
   - ❌ Hardcode de credenciais
   - ❌ Concatenação de strings para SQL
   - ❌ Aceitar paths de usuário sem validação
   - ❌ Ignorar erros de TLS/SSL

3. **Testes**
   - ❌ Commitar código sem testes
   - ❌ Desabilitar testes existentes
   - ❌ Usar dados sensíveis reais em testes

---

## 📊 Métricas de Qualidade

### Limites Aceitáveis

| Métrica | Limite |
|---------|--------|
| Complexidade ciclomática | ≤ 10 por função |
| Linhas por função | ≤ 50 |
| Parâmetros por função | ≤ 5 |
| Profundidade de indentação | ≤ 3 níveis |
| Cobertura de testes | ≥ 80% |
| Warnings do Clippy | 0 |

---

## 🔧 Configuração do Ambiente

### Ferramentas Requeridas

```bash
# Rust toolchain
rustup default stable
rustup component add rustfmt clippy

# Ferramentas de desenvolvimento
cargo install cargo-audit    # Auditoria de vulnerabilidades em dependências
cargo install cargo-outdated # Verificação de dependências desatualizadas
cargo install cargo-llvm-cov # Cobertura de código com LLVM (usado pelo CI para métricas de coverage)

# Para testes com banco de dados
docker run -d -p 5432:5432 \
  -e POSTGRES_DB=rfb_test \
  -e POSTGRES_USER=rfb \
  -e POSTGRES_PASSWORD=rfb \
  postgres:15-alpine
```

### Cobertura de Código

```bash
# Gerar relatório de cobertura local
cargo llvm-cov --all-features --workspace --lcov --output-path coverage/lcov.info

# Ver cobertura no terminal
cargo llvm-cov --all-features

# Gerar relatório HTML
cargo llvm-cov --html
```

### Variáveis de Ambiente

```bash
# Desenvolvimento
export DATABASE_URL="postgres://rfb:rfb@localhost:5432/rfb"
export RUST_LOG="debug"
export RUST_BACKTRACE="1"
```

---

## 📚 Referências

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- [Polars Documentation](https://pola-rs.github.io/polars-book/)
- [Diesel Documentation](https://diesel.rs/)
- [Actix-web Documentation](https://actix.rs/)
- [minha-receita](https://github.com/cuducos/minha-receita) - Implementação de referência

---

## 🤖 Instruções Específicas para IA

### Ao Gerar Código

1. **Sempre** verificar se o código segue os padrões deste projeto
2. **Sempre** incluir tratamento de erros apropriado
3. **Sempre** considerar edge cases e cenários de erro
4. **Sempre** manter consistência com o código existente
5. **Sempre** preferir código idiomático Rust

### Ao Sugerir Modificações

1. Explicar o raciocínio por trás da mudança
2. Indicar possíveis impactos em outras partes do código
3. Sugerir testes para validar a modificação
4. Alertar sobre possíveis problemas de performance ou segurança

### Ao Responder Perguntas

1. Basear respostas no contexto específico do projeto RFB-RS
2. Referenciar documentação existente quando aplicável
3. Fornecer exemplos de código que sigam as convenções do projeto
4. Indicar arquivos relevantes do projeto quando útil

---

> **Última atualização**: Este documento deve ser revisado e atualizado conforme o projeto evolui.
