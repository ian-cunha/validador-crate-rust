# Validador de CPF e CNPJ

Um crate Rust simples e leve para validação de CPF (Cadastro de Pessoas Físicas) e CNPJ (Cadastro Nacional de Pessoa Jurídica) brasileiro.

📦 Disponível em [crates.io](https://crates.io/crates/validador) · 📂 Código-fonte: [github.com/ian-cunha/validador-crate-rust](https://github.com/ian-cunha/validador-crate-rust)

## ✨ Funcionalidades

- ✅ Validação completa do CPF (dígitos verificadores)
- ✅ Validação completa do CNPJ (dígitos verificadores)
- ✅ Aceita CPF com ou sem formatação (pontos e traço)
- ✅ Aceita CNPJ com ou sem formatação (pontos, barra e traço)
- ✅ Rejeita CPFs com todos os dígitos iguais (ex: `111.111.111-11`)
- ✅ Rejeita CNPJs com todos os dígitos iguais (ex: `00.000.000/0000-00`)
- ✅ Sem dependências externas
- ✅ Compatível com `no_std`

## 📦 Instalação

A crate está publicada oficialmente no [crates.io](https://crates.io/crates/validador).

### Via crates.io (recomendado)

Adicione ao `Cargo.toml` do seu projeto:

```toml
[dependencies]
validador = "1.0.0"
```

Em seguida, rode:

```bash
cargo build
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
validador = { git = "https://github.com/ian-cunha/validador-crate-rust.git", tag = "v1.0.0" }
```

## 🚀 Uso

```rust
use validador::validar_cpf;
use validador::validar_cnpj;

fn main() {
    // CPF válido (formatado)
    assert!(validar_cpf("529.982.247-25"));

    // CPF válido (sem formatação)
    assert!(validar_cpf("52998224725"));

    // CPF inválido
    assert!(!validar_cpf("123.456.789-00"));

    // Todos os dígitos iguais
    assert!(!validar_cpf("111.111.111-11"));

    // Tamanho incorreto
    assert!(!validar_cpf("12345"));

    // CNPJ válido (formatado)
    assert!(validar_cnpj("12.345.678/0001-95"));

    // CNPJ válido (sem formatação)
    assert!(validar_cnpj("12345678000195"));

    // CNPJ inválido
    assert!(!validar_cnpj("12.345.678/0001-00"));

    // Todos os dígitos iguais
    assert!(!validar_cnpj("00.000.000/0000-00"));
}
```

## 📖 API

### `validar_cpf(cpf: &str) -> bool`

Valida uma string contendo um CPF e retorna `true` se for válido, `false` caso contrário.

**Parâmetros:**

| Nome  | Tipo    | Descrição                                  |
|-------|---------|--------------------------------------------|
| `cpf` | `&str`  | CPF a ser validado, com ou sem formatação. |

**Retorno:** `bool` — `true` se o CPF é válido, `false` caso contrário.

**Regras de validação:**

1. Deve conter exatamente 11 dígitos numéricos (caracteres não numéricos são ignorados).
2. Não pode ter todos os dígitos iguais.
3. Os dois dígitos verificadores devem ser calculados corretamente conforme o algoritmo oficial.

### `validar_cnpj(cnpj: &str) -> bool`

Valida uma string contendo um CNPJ e retorna `true` se for válido, `false` caso contrário.

**Parâmetros:**

| Nome   | Tipo    | Descrição                                    |
|--------|---------|----------------------------------------------|
| `cnpj` | `&str`  | CNPJ a ser validado, com ou sem formatação. |

**Retorno:** `bool` — `true` se o CNPJ é válido, `false` caso contrário.

**Regras de validação:**

1. Deve conter exatamente 14 dígitos numéricos (caracteres não numéricos são ignorados).
2. Não pode ter todos os dígitos iguais.
3. Os dois dígitos verificadores devem ser calculados corretamente conforme o algoritmo oficial.

## 🧪 Exemplo de execução

```bash
cargo run --example teste
```

## 🛠️ Algoritmo

### CPF

A validação segue o algoritmo oficial da Receita Federal:

1. **1º dígito verificador:** multiplica os 9 primeiros dígitos pelos pesos decrescentes de 10 a 2, soma os resultados, multiplica por 10 e calcula o resto da divisão por 11. Se o resto for 10, considera-se 0.
2. **2º dígito verificador:** mesmo processo, mas incluindo o 1º dígito verificador, com pesos de 11 a 2.

### CNPJ

A validação segue o algoritmo oficial da Receita Federal:

1. **1º dígito verificador:** multiplica os 12 primeiros dígitos pelos pesos [5,4,3,2,9,8,7,6,5,4,3,2], soma os resultados, calcula o resto da divisão por 11. Se o resto for menor que 2, o dígito é 0; caso contrário, é 11 menos o resto.
2. **2º dígito verificador:** mesmo processo, mas incluindo o 1º dígito verificador, com pesos [6,5,4,3,2,9,8,7,6,5,4,3,2].

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.