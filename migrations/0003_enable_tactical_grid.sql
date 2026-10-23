-- O mapa tático montável sai do modo de testes (só administradores) e fica disponível
-- para todas as salas. Quem desativou o recurso de propósito ('disabled') é respeitado.
UPDATE system_settings SET value = 'enabled' WHERE key = 'feature_tactical_grid' AND value = 'admin_only';
