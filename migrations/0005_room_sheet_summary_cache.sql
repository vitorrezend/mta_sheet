-- Resumo do card da ficha na sala (vitalidade, força de vontade, esferas, foto...), já
-- calculado. Vale enquanto room_summary_version = version: toda gravação do conteúdo da
-- ficha sobe a versão, e a sala recalcula só as fichas que mudaram.
ALTER TABLE character_sheets ADD COLUMN room_summary_json TEXT;
ALTER TABLE character_sheets ADD COLUMN room_summary_version INTEGER;
