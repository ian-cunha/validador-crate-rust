//! # Validador
//!
//! Uma crate Rust para validação de dados comuns em projetos do dia a dia.
//! Inclui validadores para CPF, CNPJ, e-mail, CEP, telefone, data, cartão de crédito, senha, URL, UUID e placa de carro.

pub mod validadores {
    /// Valida um CPF (Cadastro de Pessoas Físicas) brasileiro.
    ///
    /// # Argumentos
    /// * `cpf` - String contendo o CPF, com ou sem formatação (ex: "123.456.789-09" ou "12345678909").
    ///
    /// # Retorno
    /// `true` se o CPF for válido, `false` caso contrário.
    ///
    /// # Exemplo
    /// ```rust
    /// use validador::validadores;
    /// assert!(validadores::cpf("529.982.247-25"));
    /// assert!(!validadores::cpf("123.456.789-00"));
    /// ```
    pub fn cpf(cpf: &str) -> bool {
        let digits: Vec<u32> = cpf.chars().filter_map(|c| c.to_digit(10)).collect();

        if digits.len() != 11 {
            return false;
        }

        if digits.iter().all(|&d| d == digits[0]) {
            return false;
        }

        let sum1: u32 = (0..9).map(|i| digits[i] * (10 - i as u32)).sum();
        let mut check1 = sum1 * 10 % 11;
        if check1 == 10 {
            check1 = 0;
        }

        if check1 != digits[9] {
            return false;
        }

        let sum2: u32 = (0..10).map(|i| digits[i] * (11 - i as u32)).sum();
        let mut check2 = sum2 * 10 % 11;
        if check2 == 10 {
            check2 = 0;
        }

        check2 == digits[10]
    }

    /// Valida um CNPJ (Cadastro Nacional de Pessoa Jurídica) brasileiro.
    ///
    /// # Argumentos
    /// * `cnpj` - String contendo o CNPJ, com ou sem formatação (ex: "12.345.678/0001-95" ou "12345678000195").
    ///
    /// # Retorno
    /// `true` se o CNPJ for válido, `false` caso contrário.
    ///
    /// # Exemplo
    /// ```rust
    /// use validador::validadores;
    /// assert!(validadores::cnpj("12.345.678/0001-95"));
    /// assert!(!validadores::cnpj("12.345.678/0001-00"));
    /// ```
    pub fn cnpj(cnpj: &str) -> bool {
        let digits: Vec<u32> = cnpj.chars().filter_map(|c| c.to_digit(10)).collect();

        if digits.len() != 14 {
            return false;
        }

        if digits.iter().all(|&d| d == digits[0]) {
            return false;
        }

        let weights1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        let sum1: u32 = (0..12).map(|i| digits[i] * weights1[i]).sum();
        let mut check1 = sum1 % 11;
        if check1 < 2 {
            check1 = 0;
        } else {
            check1 = 11 - check1;
        }

        if check1 != digits[12] {
            return false;
        }

        let weights2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        let sum2: u32 = (0..13).map(|i| digits[i] * weights2[i]).sum();
        let mut check2 = sum2 % 11;
        if check2 < 2 {
            check2 = 0;
        } else {
            check2 = 11 - check2;
        }

        check2 == digits[13]
    }

    /// Valida um endereço de e-mail.
    ///
    /// # Argumentos
    /// * `email` - String contendo o e-mail a ser validado.
    ///
    /// # Retorno
    /// `true` se o e-mail for válido, `false` caso contrário.
    ///
    /// # Exemplo
    /// ```rust
    /// use validador::validadores;
    /// assert!(validadores::email("fulano@example.com"));
    /// assert!(!validadores::email("fulano@example"));
    /// ```
    pub fn email(email: &str) -> bool {
        email.contains('@')
            && email.contains('.')
            && !email.starts_with('@')
            && !email.ends_with('.')
    }

    /// Valida um CEP (Código de Endereçamento Postal) brasileiro.
    ///
    /// # Argumentos
    /// * `cep` - String contendo o CEP, com ou sem formatação (ex: "12345-678" ou "12345678").
    ///
    /// # Retorno
    /// `true` se o CEP for válido, `false` caso contrário.
    ///
    /// # Exemplo
    /// ```rust
    /// use validador::validadores;
    /// assert!(validadores::cep("12345-678"));
    /// assert!(!validadores::cep("12345"));
    /// ```
    pub fn cep(cep: &str) -> bool {
        let digits: Vec<u32> = cep.chars().filter_map(|c| c.to_digit(10)).collect();
        digits.len() == 8
    }

    /// Valida um número de telefone brasileiro.
    ///
    /// # Argumentos
    /// * `telefone` - String contendo o telefone, com ou sem formatação (ex: "(11) 98765-4321" ou "11987654321").
    ///
    /// # Retorno
    /// `true` se o telefone for válido, `false` caso contrário.
    ///
    /// # Exemplo
    /// ```rust
    /// use validador::validadores;
    /// assert!(validadores::telefone("(11) 98765-4321"));
    /// assert!(!validadores::telefone("123"));
    /// ```
    pub fn telefone(telefone: &str) -> bool {
        let digits: Vec<u32> = telefone.chars().filter_map(|c| c.to_digit(10)).collect();
        digits.len() == 10 || digits.len() == 11
    }

    /// Valida uma data no formato `DD/MM/AAAA` ou `YYYY-MM-DD`.
    ///
    /// # Argumentos
    /// * `data` - String contendo a data a ser validada.
    ///
    /// # Retorno
    /// `true` se a data for válida, `false` caso contrário.
    ///
    /// # Exemplo
    /// ```rust
    /// use validador::validadores;
    /// assert!(validadores::data("31/12/2023"));
    /// assert!(validadores::data("2023-12-31"));
    /// assert!(!validadores::data("31/02/2023"));
    /// ```
    pub fn data(data: &str) -> bool {
        if let Ok(_) = chrono::NaiveDate::parse_from_str(data, "%d/%m/%Y") {
            return true;
        }
        if let Ok(_) = chrono::NaiveDate::parse_from_str(data, "%Y-%m-%d") {
            return true;
        }
        false
    }

    /// Valida um número de cartão de crédito usando o algoritmo de Luhn.
    ///
    /// # Argumentos
    /// * `cartao` - String contendo o número do cartão, com ou sem formatação (ex: "1234 5678 9012 3456" ou "1234567890123456").
    ///
    /// # Retorno
    /// `true` se o cartão for válido, `false` caso contrário.
    ///
    /// # Exemplo
    /// ```rust
    /// use validador::validadores;
    /// assert!(validadores::cartao_credito("4532015112830366"));
    /// assert!(!validadores::cartao_credito("1234567890123456"));
    /// ```
    pub fn cartao_credito(cartao: &str) -> bool {
        let digits: Vec<u32> = cartao.chars().filter_map(|c| c.to_digit(10)).collect();
        if digits.is_empty() {
            return false;
        }

        let mut sum = 0;
        let mut should_double = false;

        for &digit in digits.iter().rev() {
            let mut d = digit;
            if should_double {
                d *= 2;
                if d > 9 {
                    d -= 9;
                }
            }
            sum += d;
            should_double = !should_double;
        }

        sum % 10 == 0
    }

    /// Valida a força de uma senha.
    ///
    /// # Argumentos
    /// * `senha` - String contendo a senha a ser validada.
    /// * `min_length` - Comprimento mínimo da senha (padrão: 8).
    /// * `require_uppercase` - Se `true`, exige pelo menos uma letra maiúscula (padrão: `true`).
    /// * `require_lowercase` - Se `true`, exige pelo menos uma letra minúscula (padrão: `true`).
    /// * `require_digit` - Se `true`, exige pelo menos um dígito (padrão: `true`).
    /// * `require_special` - Se `true`, exige pelo menos um caractere especial (padrão: `true`).
    ///
    /// # Retorno
    /// `true` se a senha for válida, `false` caso contrário.
    ///
    /// # Exemplo
    /// ```rust
    /// use validador::validadores;
    /// assert!(validadores::senha("Senha@123", 8, true, true, true, true));
    /// assert!(!validadores::senha("senha", 8, true, true, true, true));
    /// ```
    pub fn senha(
        senha: &str,
        min_length: usize,
        require_uppercase: bool,
        require_lowercase: bool,
        require_digit: bool,
        require_special: bool,
    ) -> bool {
        if senha.len() < min_length {
            return false;
        }

        let has_uppercase = senha.chars().any(|c| c.is_ascii_uppercase());
        let has_lowercase = senha.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = senha.chars().any(|c| c.is_ascii_digit());
        let has_special = senha.chars().any(|c| !c.is_ascii_alphanumeric());

        (!require_uppercase || has_uppercase)
            && (!require_lowercase || has_lowercase)
            && (!require_digit || has_digit)
            && (!require_special || has_special)
    }

    /// Valida uma URL.
    ///
    /// # Argumentos
    /// * `url` - String contendo a URL a ser validada.
    ///
    /// # Retorno
    /// `true` se a URL for válida, `false` caso contrário.
    ///
    /// # Exemplo
    /// ```rust
    /// use validador::validadores;
    /// assert!(validadores::url("https://www.example.com"));
    /// assert!(!validadores::url("example"));
    /// ```
    pub fn url(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }

    /// Valida um UUID v4.
    ///
    /// # Argumentos
    /// * `uuid` - String contendo o UUID a ser validado.
    ///
    /// # Retorno
    /// `true` se o UUID for válido, `false` caso contrário.
    ///
    /// # Exemplo
    /// ```rust
    /// use validador::validadores;
    /// assert!(validadores::uuid("550e8400-e29b-41d4-a716-446655440000"));
    /// assert!(!validadores::uuid("12345"));
    /// ```
    pub fn uuid(uuid: &str) -> bool {
        let parts: Vec<&str> = uuid.split('-').collect();
        if parts.len() != 5 {
            return false;
        }
        parts[0].len() == 8
            && parts[1].len() == 4
            && parts[2].len() == 4
            && parts[3].len() == 4
            && parts[4].len() == 12
            && uuid.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
    }

    /// Valida uma placa de carro brasileira (formato antigo ou novo).
    ///
    /// # Argumentos
    /// * `placa` - String contendo a placa a ser validada.
    ///
    /// # Retorno
    /// `true` se a placa for válida, `false` caso contrário.
    ///
    /// # Exemplo
    /// ```rust
    /// use validador::validadores;
    /// assert!(validadores::placa_carro("ABC1234"));
    /// assert!(validadores::placa_carro("ABC-1234"));
    /// assert!(!validadores::placa_carro("ABC123"));
    /// ```
    pub fn placa_carro(placa: &str) -> bool {
        let clean_placa: String = placa
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect();
        if clean_placa.len() != 7 {
            return false;
        }
        let (letters, digits) = clean_placa.split_at(3);
        letters.chars().all(|c| c.is_ascii_uppercase())
            && digits.chars().all(|c| c.is_ascii_digit())
    }
}
