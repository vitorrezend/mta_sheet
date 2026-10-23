-- Versão do mapa tático de cada sala: cada lote de edições incrementa o número.
-- O servidor grava só se a versão não mudou desde a leitura (concorrência otimista)
-- e os clientes detectam eventos perdidos pela sequência.
ALTER TABLE rooms ADD COLUMN map_version INTEGER NOT NULL DEFAULT 0;
