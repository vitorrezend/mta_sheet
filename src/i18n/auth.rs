use super::Language;

/// Translates authentication and network error messages
pub fn tr_auth_error(msg: &str, lang: Language) -> String {
    let clean = msg.trim_start_matches("error: ").trim();
    match lang {
        Language::PtBr => match clean {
            "Usuário ou senha incorretos" | "Invalid username or password" => "Usuário ou senha incorretos".to_string(),
            "Preencha todos os campos obrigatórios" | "Please fill in all required fields" => "Preencha todos os campos obrigatórios".to_string(),
            "As senhas não conferem" | "Passwords do not match" => "As senhas não conferem".to_string(),
            "Usuário e senha são obrigatórios" | "Username and password are required" => "Usuário e senha são obrigatórios".to_string(),
            "Nome de usuário deve ter no mínimo 3 caracteres" | "Username must be at least 3 characters" => "Nome de usuário deve ter no mínimo 3 caracteres".to_string(),
            "A senha deve ter no mínimo 4 caracteres" | "Password must be at least 4 characters" => "A senha deve ter no mínimo 4 caracteres".to_string(),
            "A senha deve ter no mínimo 8 caracteres." | "Password must be at least 8 characters." => "A senha deve ter no mínimo 8 caracteres.".to_string(),
            "A senha não pode exceder 128 caracteres." | "Password cannot exceed 128 characters." => "A senha não pode exceder 128 caracteres.".to_string(),
            "A senha deve conter pelo menos uma letra e um número." | "Password must contain at least one letter and one number." => "A senha deve conter pelo menos uma letra e um número.".to_string(),
            "Muitas tentativas de cadastro. Aguarde 1 minuto antes de tentar novamente." | "Too many registration attempts. Please wait 1 minute before trying again." => "Muitas tentativas de cadastro. Aguarde 1 minuto antes de tentar novamente.".to_string(),
            "Muitas tentativas de login. Aguarde 1 minuto antes de tentar novamente." | "Too many login attempts. Please wait 1 minute before trying again." => "Muitas tentativas de login. Aguarde 1 minuto antes de tentar novamente.".to_string(),
            "Este nome de usuário já está em uso" | "This username is already taken" => "Este nome de usuário já está em uso".to_string(),
            "Falha temporária ao conectar ao banco de dados" | "Temporary database connection error" => "Falha temporária ao conectar ao banco de dados".to_string(),
            "Conexão com o banco de dados indisponível" | "Database connection unavailable" => "Conexão com o banco de dados indisponível".to_string(),
            "Erro ao verificar credenciais" | "Error verifying credentials" => "Erro ao verificar credenciais".to_string(),
            _ => clean.to_string(),
        },
        Language::EnUs => match clean {
            "Usuário ou senha incorretos" | "Invalid username or password" => "Invalid username or password".to_string(),
            "Preencha todos os campos obrigatórios" | "Please fill in all required fields" => "Please fill in all required fields".to_string(),
            "As senhas não conferem" | "Passwords do not match" => "Passwords do not match".to_string(),
            "Usuário e senha são obrigatórios" | "Username and password are required" => "Username and password are required".to_string(),
            "Nome de usuário deve ter no mínimo 3 caracteres" | "Username must be at least 3 characters" => "Username must be at least 3 characters".to_string(),
            "A senha deve ter no mínimo 4 caracteres" | "Password must be at least 4 characters" => "Password must be at least 4 characters".to_string(),
            "A senha deve ter no mínimo 8 caracteres." | "Password must be at least 8 characters." => "Password must be at least 8 characters.".to_string(),
            "A senha não pode exceder 128 caracteres." | "Password cannot exceed 128 characters." => "Password cannot exceed 128 characters.".to_string(),
            "A senha deve conter pelo menos uma letra e um número." | "Password must contain at least one letter and one number." => "Password must contain at least one letter and one number.".to_string(),
            "Muitas tentativas de cadastro. Aguarde 1 minuto antes de tentar novamente." | "Too many registration attempts. Please wait 1 minute before trying again." => "Too many registration attempts. Please wait 1 minute before trying again.".to_string(),
            "Muitas tentativas de login. Aguarde 1 minuto antes de tentar novamente." | "Too many login attempts. Please wait 1 minute before trying again." => "Too many login attempts. Please wait 1 minute before trying again.".to_string(),
            "Este nome de usuário já está em uso" | "This username is already taken" => "This username is already taken".to_string(),
            "Falha temporária ao conectar ao banco de dados" | "Temporary database connection error" => "Temporary database connection error".to_string(),
            "Conexão com o banco de dados indisponível" | "Database connection unavailable" => "Database connection unavailable".to_string(),
            "Erro ao verificar credenciais" | "Error verifying credentials" => "Error verifying credentials".to_string(),
            _ => clean.to_string(),
        },
    }
}
