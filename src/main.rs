use rand::Rng;

fn generate_password(length: usize) -> String {
    // Conjunto de caracteres para a senha
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                 abcdefghijklmnopqrstuvwxyz\
                 0123456789\
                 !@#$%^&*()-_=+[]{}|;:,.<>?";
    
    // Gera a senha aleatoriamente a partir dos caracteres
    let password: String = (0..length)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..chars.len());
            chars.chars().nth(idx).unwrap()
        })
        .collect();
    
    password
}

fn main() {
    // Define o comprimento da senha, você pode mudar para qualquer valor
    let password_length = 12;
    let password = generate_password(password_length);
    println!("Generated password: {}", password);
}
