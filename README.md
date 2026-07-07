# Validador

Uma crate Rust para validação de dados comuns em projetos do dia a dia. Inclui validadores para CPF, CNPJ, e-mail, CEP, telefone, data, cartão de crédito, senha, URL, UUID e placa de carro.

📦 Disponível em [crates.io](https://crates.io/crates/validador) · 📂 Código-fonte: [github.com/ian-cunha/validador-crate-rust](https://github.com/ian-cunha/validador-crate-rust)

## ✨ Funcionalidades

- ✅ Validação de CPF (Cadastro de Pessoas Físicas)
- ✅ Validação de CNPJ (Cadastro Nacional de Pessoa Jurídica)
- ✅ Validação de e-mail (básica)
- ✅ Validação de CEP (Código de Endereçamento Postal)
- ✅ Validação de telefone brasileiro
- ✅ Validação de data (formatos DD/MM/AAAA e YYYY-MM-DD)
- ✅ Validação de cartão de crédito (algoritmo de Luhn)
- ✅ Validação de senha (com requisitos configuráveis)
- ✅ Validação de URL (verifica http/https)
- ✅ Validação de UUID v4
- ✅ Validação de placa de carro brasileira (modelo antigo e novo)
- ✅ Sem dependências externas (exceto `chrono` para validação de data)
- ✅ Compatível com `no_std` (exceto para a função `data` que requer `chrono`)

## 📦 Instalação

A crate está publicada oficialmente no [crates.io](https://crates.io/crates/validador).

### Via crates.io (recomendado)

Adicione ao `Cargo.toml` do seu projeto:

```toml
[dependencies]
validador = "0.1.0"
```

### Via GitHub

Se preferir, você pode instalar diretamente do repositório:

```toml
[dependencies]
validador = { git = "https://github.com/ian-cunha/validador-crate-rust.git" }
```

Ou fixando uma branch/tag/branch específica:

```toml
[dependencies]
validador = { git = "https://github.com/ian-cunha/validador-crate-rust.git", tag = "v0.1.0" }
```

## 🚀 Uso

```rust
use validador::validadores;

fn main() {
    // CPF
    assert!(validadores::cpf("529.982.247-25"));
    assert!(!validadores::cpf("123.456.789-00"));

    // CNPJ
    assert!(validadores::cnpj("12.345.678/0001-95"));
    assert!(!validadores::cnpj("12.345.678/0001-00"));

    // E-mail
    assert!(validadores::email("fulano@example.com"));
    assert!(!validadores::email("fulano@example"));

    // CEP
    assert!(validadores::cep("12345-678"));
    assert!(!validadores::cep("12345"));

    // Telefone
    assert!(validadores::telefone("(11) 98765-4321"));
    assert!(!validadores::telefone("123"));

    // Data
    assert!(validadores::data("31/12/2023"));
    assert!(validadores::data("2023-12-31"));
    assert!(!validadores::data("31/02/2023"));

    // Cartão de crédito
    assert!(validadores::cartao_credito("4532015112830366"));
    assert!(!validadores::cartao_credito("1234567890123456"));

    // Senha
    assert!(validadores::senha("Senha@123", 8, true, true, true, true));
    assert!(!validadores::senha("senha", 8, true, true, true, true));

    // URL
    assert!(validadores::url("https://www.example.com"));
    assert!(!validadores::url("example"));

    // UUID
    assert!(validadores::uuid("550e8400-e29b-41d4-a716-446655440000"));
    assert!(!validadores::uuid("12345"));

    // Placa de carro
    assert!(validadores::placa_carro("ABC1234"));
    assert!(validadores::placa_carro("ABC-1234"));
    assert!(!validadores::placa_carro("ABC123"));
}
```

## 📖 API

### `cpf(cpf: &str) -> bool`

Valida um CPF brasileiro.

**Argumentos:**
- `cpf` - String contendo o CPF, com ou sem formatação (ex: "123.456.789-09" ou "12345678909").

**Retorno:** `true` se o CPF for válido, `false` caso contrário.

**Exemplo:**
```rust
use validador::validadores;
assert!(validadores::cpf("529.982.247-25"));
assert!(!validadores::cpf("123.456.789-00"));
```

### `cnpj(cnpj: &str) -> bool`

Valida um CNPJ brasileiro.

**Argumentos:**
- `cnpj` - String contendo o CNPJ, com ou sem formatação (ex: "12.345.678/0001-95" ou "12345678000195").

**Retorno:** `true` se o CNPJ for válido, `false` caso contrário.

**Exemplo:**
```rust
use validador::validadores;
assert!(validadores::cnpj("12.345.678/0001-95"));
assert!(!validadores::cnpj("12.345.678/0001-00"));
```

### `email(email: &str) -> bool`

Valida um endereço de e-mail (validação básica).

**Argumentos:**
- `email` - String contendo o e-mail a ser validado.

**Retorno:** `true` se o e-mail contiver '@' e '.', não iniciar com '@' e não terminar com '.', `false` caso contrário.

**Exemplo:**
```rust
use validador::validadores;
assert!(validadores::email("fulano@example.com"));
assert!(!validadores::email("fulano@example"));
```

### `cep(cep: &str) -> bool`

Valida um CEP brasileiro.

**Argumentos:**
- `cep` - String contendo o CEP, com ou sem formatação (ex: "12345-678" ou "12345678").

**Retorno:** `true` se o CEP tiver exatamente 8 dígitos, `false` caso contrário.

**Exemplo:**
```rust
use validador::validadores;
assert!(validadores::cep("12345-678"));
assert!(!validadores::cep("12345"));
```

### `telefone(telefone: &str) -> bool`

Valida um número de telefone brasileiro.

**Argumentos:**
- `telefone` - String contendo o telefone, com ou sem formatação (ex: "(11) 98765-4321" ou "11987654321").

**Retorno:** `true` se o telefone tiver 10 ou 11 dígitos, `false` caso contrário.

**Exemplo:**
```rust
use validador::validadores;
assert!(validadores::telefone("(11) 98765-4321"));
assert!(!validadores::telefone("123"));
```

### `data(data: &str) -> bool`

Valida uma data nos formatos `DD/MM/AAAA` ou `YYYY-MM-DD`.

**Argumentos:**
- `data` - String contendo a data a ser validada.

**Retorno:** `true` se a data for válida em algum dos formatos, `false` caso contrário.

**Exemplo:**
```rust
use validador::validadores;
assert!(validadores::data("31/12/2023"));
assert!(validadores::data("2023-12-31"));
assert!(!validadores::data("31/02/2023"));
```

*Nota: Esta função requer a dependência `chrono`.*

### `cartao_credito(cartao: &str) -> bool`

Valida um número de cartão de crédito usando o algoritmo de Luhn.

**Argumentos:**
- `cartao` - String contendo o número do cartão, com ou sem formatação (ex: "1234 5678 9012 3456" ou "1234567890123456").

**Retorno:** `true` se o cartão passar na validação de Luhn, `false` caso contrário.

**Exemplo:**
```rust
use validador::validadores;
assert!(validadores::cartao_credito("4532015112830366"));
assert!(!validadores::cartao_credito("1234567890123456"));
```

### `senha(senha: &str, min_length: usize, require_uppercase: bool, require_lowercase: bool, require_digit: bool, require_special: bool) -> bool`

Valida a força de uma senha com requisitos configuráveis.

**Argumentos:**
- `senha` - String contendo a senha a ser validada.
- `min_length` - Comprimento mínimo da senha.
- `require_uppercase` - Se `true`, exige pelo menos uma letra maiúscula.
- `require_lowercase` - Se `true`, exige pelo menos uma letra minúscula.
- `require_digit` - Se `true`, exige pelo menos um dígito.
- `require_special` - Se `true`, exige pelo menos um caractere especial.

**Retorno:** `true` se a senha atender a todos os requisitos especificados, `false` caso contrário.

**Exemplo:**
```rust
use validador::validadores;
assert!(validadores::senha("Senha@123", 8, true, true, true, true));
assert!(!validadores::senha("senha", 8, true, true, true, true));
```

### `url(url: &str) -> bool`

Valida uma URL.

**Argumentos:**
- `url` - String contendo a URL a ser validada.

**Retorno:** `true` se a URL iniciar com "http://" ou "https://", `false` caso contrário.

**Exemplo:**
```rust
use validador::validadores;
assert!(validadores::url("https://www.example.com"));
assert!(!validadores::url("example"));
```

### `uuid(uuid: &str) -> bool`

Valida um UUID v4.

**Argumentos:**
- `uuid` - String contendo o UUID a ser validado.

**Retorno:** `true` se o UUID seguir o formato v4 (8-4-4-4-12 hexadecimais separados por hífen), `false` caso contrário.

**Exemplo:**
```rust
use validador::validadores;
assert!(validadores::uuid("550e8400-e29b-41d4-a716-446655440000"));
assert!(!validadores::uuid("12345"));
```

### `placa_carro(placa: &str) -> bool`

Valida uma placa de carro brasileira (formato antigo ou novo).

**Argumentos:**
- `placa` - String contendo a placa a ser validada.

**Retorno:** `true` se a placa tiver 3 letras seguidas de 4 dígitos (com ou sem hífen), `false` caso contrário.

**Exemplo:**
```rust
use validador::validadores;
assert!(validadores::placa_carro("ABC1234"));
assert!(validadores::placa_carro("ABC-1234"));
assert!(!validadores::placa_carro("ABC123"));
```

## 🧪 Exemplo de execução

```bash
cargo run --example teste
```

## 🛠️ Observações

- A validação de e-mail é intencionalmente simples e não cobre todos os casos válidos conforme RFC 5322. Para validação rigorosa de e-mail, considere usar uma crate especializada.
- A validação de URL apenas verifica o prefixo "http://" ou "https://". Não valida a estrutura completa da URL.
- A validação de data usa a crate `chrono`, então certifique-se de incluí-la em suas dependências se for usar essa função.

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.