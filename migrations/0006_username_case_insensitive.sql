-- Nomes de usuário passam a ser únicos sem diferenciar maiúsculas de minúsculas.
-- Antes, a coluna só barrava nomes idênticos: com "vitor" cadastrado, ainda era possível
-- criar "Vitor" ou "VITOR". Como ADMIN_USERNAMES e o login comparam sem diferenciar
-- maiúsculas, essa segunda conta herdava o papel de administrador e confundia o login.
--
-- Bancos que já tenham nomes repetidos nessa comparação: a conta mais antiga mantém o
-- nome; as outras ganham um sufixo com o início do id e perdem o papel de administrador
-- (que só pode ter vindo da colisão com um nome da lista ADMIN_USERNAMES).
UPDATE users
SET username = username || '_' || substr(id, 1, 8),
    is_admin = 0
WHERE rowid NOT IN (
    SELECT MIN(rowid) FROM users GROUP BY username COLLATE NOCASE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_username_nocase ON users (username COLLATE NOCASE);
