# validador

Uma crate Rust simples e leve para validação de CPF (Cadastro de Pessoas Físicas) brasileiro.

## ✨ Funcionalidades

- ✅ Validação completa do CPF (dígitos verificadores)
- ✅ Aceita CPF com ou sem formatação (pontos e traço)
- ✅ Rejeita CPFs com todos os dígitos iguais (ex: `111.111.111-11`)
- ✅ Sem dependências externas

## 📦 Instalação

A crate é instalada diretamente do repositório GitHub.

Adicione ao `Cargo.toml` do seu projeto:

```toml
[dependencies]
validador = { git = "https://github.com/ian-cunha/validador-crate-rust.git" }
```

Ou, se quiser fixar uma versão/branch/tag específica:

```toml
[dependencies]
validador = { git = "https://github.com/ian-cunha/validador-crate-rust.git", tag = "v0.1.0" }
```

Em seguida, rode:

```bash
cargo build
```

## 🚀 Uso

```rust
use validador::validar_cpf;

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

## 🧪 Exemplo de execução

```bash
cargo run --example teste
```

## 🛠️ Algoritmo

A validação segue o algoritmo oficial da Receita Federal:

1. **1º dígito verificador:** multiplica os 9 primeiros dígitos pelos pesos decrescentes de 10 a 2, soma os resultados, multiplica por 10 e calcula o resto da divisão por 11. Se o resto for 10, considera-se 0.
2. **2º dígito verificador:** mesmo processo, mas incluindo o 1º dígito verificador, com pesos de 11 a 2.

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.
