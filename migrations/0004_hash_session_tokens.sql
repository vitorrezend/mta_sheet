-- A partir desta versão o banco guarda só o SHA-256 do token de sessão (64 caracteres
-- hexadecimais), nunca o token do cookie. Sessões antigas, gravadas com o token em texto
-- puro (UUID de 36 caracteres), são removidas: quem estava logado entra de novo uma vez.
DELETE FROM sessions WHERE length(id) != 64;
