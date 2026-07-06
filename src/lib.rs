pub fn validar_cpf(cpf: &str) -> bool {
    // Extrai apenas os dígitos (ignorando pontos e traços)
    let digits: Vec<u32> = cpf.chars().filter_map(|c| c.to_digit(10)).collect();

    // Um CPF válido deve ter exatamente 11 dígitos
    if digits.len() != 11 {
        return false;
    }

    // Rejeita CPFs com todos os dígitos iguais (ex: "00000000000")
    if digits.iter().all(|&d| d == digits[0]) {
        return false;
    }

    // Cálculo do 1º dígito verificador (10ª posição)
    // Pesos: 10, 9, 8, 7, 6, 5, 4, 3, 2
    let sum1: u32 = (0..9).map(|i| digits[i] * (10 - i as u32)).sum();
    let mut check1 = (sum1 * 10) % 11;
    if check1 == 10 {
        check1 = 0;
    }

    if check1 != digits[9] {
        return false;
    }

    // Cálculo do 2º dígito verificador (11ª posição)
    // Pesos: 11, 10, 9, 8, 7, 6, 5, 4, 3, 2
    let sum2: u32 = (0..10).map(|i| digits[i] * (11 - i as u32)).sum();
    let mut check2 = (sum2 * 10) % 11;
    if check2 == 10 {
        check2 = 0;
    }

    if check2 != digits[10] {
        return false;
    }

    true
}
