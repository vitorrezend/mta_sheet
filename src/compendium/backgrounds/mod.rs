//! M20 Compêndio Modular de Antecedentes (Backgrounds)
//!
//! Capítulo 6: Criação do Personagem (pp. 301-311).
//!
//! Organizado em submódulos coesos (< 500 linhas cada):
//! - `models`: Tipos de dados, enums e contratos
//! - `data`: Catálogo unificado dos 33 Antecedentes Canônicos e Regras de Criação
//! - `unabridged`: Textos completos carregados sob demanda via Markdown

pub mod models;
pub mod data;
pub mod unabridged;

pub use models::*;
pub use data::*;
pub use unabridged::find_unabridged_text;

// ============================================================================
// Catálogo Unificado Canônico de M20 (33 Antecedentes Completos)
// ============================================================================

pub const ALL_BACKGROUNDS: &[BackgroundDefinition] = &[
    ALLIES,
    ALTERNATE_IDENTITY,
    ARCANE,
    AVATAR,
    BACKUP,
    BLESSING,
    CERTIFICATION,
    CHANTRY,
    CONTACTS,
    CULT,
    DEMESNE,
    DESTINY,
    DREAM,
    ENHANCEMENT,
    FAME,
    FAMILIAR,
    INFLUENCE,
    LEGEND,
    LIBRARY,
    MENTOR,
    NODE,
    PAST_LIVES,
    PATRON,
    RANK,
    REQUISITIONS,
    RESOURCES,
    RETAINERS,
    SANCTUM,
    SECRET_WEAPONS,
    SPIES,
    STATUS,
    TOTEM,
    WONDER,
];

/// Retorna todos os antecedentes canônicos catalogados.
pub fn get_all_backgrounds() -> &'static [BackgroundDefinition] {
    ALL_BACKGROUNDS
}

/// Normaliza uma string para comparação case-insensitive sem acentos básicos.
fn normalize_query(q: &str) -> String {
    q.trim()
        .to_lowercase()
        .replace(['á', 'à', 'ã', 'â'], "a")
        .replace(['é', 'ê'], "e")
        .replace(['í'], "i")
        .replace(['ó', 'õ', 'ô'], "o")
        .replace(['ú'], "u")
        .replace(['ç'], "c")
}

/// Busca inteligente e resiliente por um Antecedente no Compêndio.
/// Suporta IDs, nomes em inglês, nomes em português, variantes da Tecnocracia e buscas aproximadas.
pub fn find_background(query: &str) -> Option<&'static BackgroundDefinition> {
    let q_norm = normalize_query(query);
    if q_norm.is_empty() {
        return None;
    }

    // 1. Match exato por ID
    if let Some(bg) = ALL_BACKGROUNDS.iter().find(|b| b.id == q_norm) {
        return Some(bg);
    }

    // 2. Match exato por nome normalizado (EN ou PT)
    if let Some(bg) = ALL_BACKGROUNDS.iter().find(|b| {
        normalize_query(b.name) == q_norm || normalize_query(b.name_pt) == q_norm
    }) {
        return Some(bg);
    }

    // 3. Match em nomes da Tecnocracia (ex: "Cloaking", "Camuflagem", "Genius", "Gênio", "Construct", "Constructo", "Hypercram", "Hiperaprendizado")
    if let Some(bg) = ALL_BACKGROUNDS.iter().find(|b| {
        b.technocracy_name.map(|t| normalize_query(t) == q_norm).unwrap_or(false)
            || b.technocracy_name_pt.map(|t| normalize_query(t) == q_norm).unwrap_or(false)
    }) {
        return Some(bg);
    }

    // 4. Substring nos nomes principais ou nos nomes da Tecnocracia
    ALL_BACKGROUNDS.iter().find(|b| {
        normalize_query(b.name).contains(&q_norm)
            || normalize_query(b.name_pt).contains(&q_norm)
            || b.technocracy_name.map(|t| normalize_query(t).contains(&q_norm)).unwrap_or(false)
            || b.technocracy_name_pt.map(|t| normalize_query(t).contains(&q_norm)).unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_all_backgrounds_count_and_uniqueness() {
        assert_eq!(ALL_BACKGROUNDS.len(), 33, "M20 deve conter exatamente 33 antecedentes no catálogo canônico completo");
        let mut ids = HashSet::new();
        for bg in ALL_BACKGROUNDS {
            assert!(ids.insert(bg.id), "ID duplicado encontrado: {}", bg.id);
            assert!(bg.page_ref.starts_with("M20, p"), "Referência de página inválida para {}: {}", bg.name, bg.page_ref);
            assert!(!bg.ratings.is_empty(), "O antecedente {} deve conter níveis de pontuação", bg.name);
            assert!(!bg.name_pt.is_empty(), "Nome em PT não pode ser vazio para {}", bg.name);
            assert!(!bg.description.is_empty(), "Descrição em EN não pode ser vazia para {}", bg.name);
            assert!(!bg.description_pt.is_empty(), "Descrição em PT não pode ser vazia para {}", bg.name_pt);
            assert!(!bg.system.is_empty(), "Sistema em EN não pode ser vazio para {}", bg.name);
            assert!(!bg.system_pt.is_empty(), "Sistema em PT não pode ser vazio para {}", bg.name_pt);
        }
    }

    #[test]
    fn test_find_background_resolves_bilingual_and_technocracy() {
        // Testes em inglês (1 a 13)
        assert_eq!(find_background("Allies").unwrap().id, "allies");
        assert_eq!(find_background("Alternate Identity").unwrap().id, "alternate_identity");
        assert_eq!(find_background("Arcane").unwrap().id, "arcane");
        assert_eq!(find_background("Avatar").unwrap().id, "avatar");
        assert_eq!(find_background("Backup").unwrap().id, "backup");
        assert_eq!(find_background("Blessing").unwrap().id, "blessing");
        assert_eq!(find_background("Certification").unwrap().id, "certification");
        assert_eq!(find_background("Chantry").unwrap().id, "chantry");
        assert_eq!(find_background("Contacts").unwrap().id, "contacts");
        assert_eq!(find_background("Cult").unwrap().id, "cult");
        assert_eq!(find_background("Demesne").unwrap().id, "demesne");
        assert_eq!(find_background("Destiny").unwrap().id, "destiny");
        assert_eq!(find_background("Dream").unwrap().id, "dream");

        // Testes em inglês (14 a 25)
        assert_eq!(find_background("Enhancement").unwrap().id, "enhancement");
        assert_eq!(find_background("Fame").unwrap().id, "fame");
        assert_eq!(find_background("Familiar").unwrap().id, "familiar");
        assert_eq!(find_background("Influence").unwrap().id, "influence");
        assert_eq!(find_background("Legend").unwrap().id, "legend");
        assert_eq!(find_background("Library").unwrap().id, "library");
        assert_eq!(find_background("Mentor").unwrap().id, "mentor");
        assert_eq!(find_background("Node").unwrap().id, "node");
        assert_eq!(find_background("Past Lives").unwrap().id, "past_lives");
        assert_eq!(find_background("Patron").unwrap().id, "patron");
        assert_eq!(find_background("Rank").unwrap().id, "rank");
        assert_eq!(find_background("Requisitions").unwrap().id, "requisitions");

        // Testes em inglês (26 a 33)
        assert_eq!(find_background("Resources").unwrap().id, "resources");
        assert_eq!(find_background("Retainers").unwrap().id, "retainers");
        assert_eq!(find_background("Sanctum").unwrap().id, "sanctum");
        assert_eq!(find_background("Secret Weapons").unwrap().id, "secret_weapons");
        assert_eq!(find_background("Spies").unwrap().id, "spies");
        assert_eq!(find_background("Status").unwrap().id, "status");
        assert_eq!(find_background("Totem").unwrap().id, "totem");
        assert_eq!(find_background("Wonder").unwrap().id, "wonder");

        // Testes em português com ou sem acentos (1 a 13)
        assert_eq!(find_background("Aliados").unwrap().id, "allies");
        assert_eq!(find_background("aliados").unwrap().id, "allies");
        assert_eq!(find_background("Identidade Alternativa").unwrap().id, "alternate_identity");
        assert_eq!(find_background("Arcano").unwrap().id, "arcane");
        assert_eq!(find_background("Reforço").unwrap().id, "backup");
        assert_eq!(find_background("reforco").unwrap().id, "backup");
        assert_eq!(find_background("Bênção").unwrap().id, "blessing");
        assert_eq!(find_background("bencao").unwrap().id, "blessing");
        assert_eq!(find_background("Certificação").unwrap().id, "certification");
        assert_eq!(find_background("Capela").unwrap().id, "chantry");
        assert_eq!(find_background("Contatos").unwrap().id, "contacts");
        assert_eq!(find_background("Culto").unwrap().id, "cult");
        assert_eq!(find_background("Domínio").unwrap().id, "demesne");
        assert_eq!(find_background("Destino").unwrap().id, "destiny");
        assert_eq!(find_background("Sonho").unwrap().id, "dream");

        // Testes em português com ou sem acentos (14 a 25)
        assert_eq!(find_background("Aprimoramento").unwrap().id, "enhancement");
        assert_eq!(find_background("Fama").unwrap().id, "fame");
        assert_eq!(find_background("Familiar").unwrap().id, "familiar");
        assert_eq!(find_background("Influência").unwrap().id, "influence");
        assert_eq!(find_background("influencia").unwrap().id, "influence");
        assert_eq!(find_background("Lenda").unwrap().id, "legend");
        assert_eq!(find_background("Biblioteca").unwrap().id, "library");
        assert_eq!(find_background("Mentor").unwrap().id, "mentor");
        assert_eq!(find_background("Nó").unwrap().id, "node");
        assert_eq!(find_background("Vidas Passadas").unwrap().id, "past_lives");
        assert_eq!(find_background("Patrono").unwrap().id, "patron");
        assert_eq!(find_background("Posto").unwrap().id, "rank");
        assert_eq!(find_background("Requisições").unwrap().id, "requisitions");
        assert_eq!(find_background("requisicoes").unwrap().id, "requisitions");

        // Testes em português com ou sem acentos (26 a 33)
        assert_eq!(find_background("Recursos").unwrap().id, "resources");
        assert_eq!(find_background("recursos").unwrap().id, "resources");
        assert_eq!(find_background("Lacaios").unwrap().id, "retainers");
        assert_eq!(find_background("lacaios").unwrap().id, "retainers");
        assert_eq!(find_background("Santuário").unwrap().id, "sanctum");
        assert_eq!(find_background("santuario").unwrap().id, "sanctum");
        assert_eq!(find_background("Armas Secretas").unwrap().id, "secret_weapons");
        assert_eq!(find_background("Espiões").unwrap().id, "spies");
        assert_eq!(find_background("espioes").unwrap().id, "spies");
        assert_eq!(find_background("Status").unwrap().id, "status");
        assert_eq!(find_background("Totem").unwrap().id, "totem");
        assert_eq!(find_background("Maravilha").unwrap().id, "wonder");
        assert_eq!(find_background("maravilha").unwrap().id, "wonder");

        // Nomes da Tecnocracia
        assert_eq!(find_background("Cloaking").unwrap().id, "arcane");
        assert_eq!(find_background("Camuflagem").unwrap().id, "arcane");
        assert_eq!(find_background("Genius").unwrap().id, "avatar");
        assert_eq!(find_background("Gênio").unwrap().id, "avatar");
        assert_eq!(find_background("Construct").unwrap().id, "chantry");
        assert_eq!(find_background("Constructo").unwrap().id, "chantry");
        assert_eq!(find_background("Hypercram").unwrap().id, "dream");
        assert_eq!(find_background("Biotech").unwrap().id, "enhancement");
        assert_eq!(find_background("Companion").unwrap().id, "familiar");
        assert_eq!(find_background("Data Retrieval").unwrap().id, "library");
        assert_eq!(find_background("Supervisor").unwrap().id, "mentor");
        assert_eq!(find_background("Power Plant").unwrap().id, "node");
        assert_eq!(find_background("Shadow Benefactor").unwrap().id, "patron");
        assert_eq!(find_background("Clearance").unwrap().id, "rank");
        assert_eq!(find_background("Capital").unwrap().id, "resources");
        assert_eq!(find_background("Assistants").unwrap().id, "retainers");
        assert_eq!(find_background("Laboratory").unwrap().id, "sanctum");
        assert_eq!(find_background("Laboratório").unwrap().id, "sanctum");
        assert_eq!(find_background("Q Division").unwrap().id, "secret_weapons");
        assert_eq!(find_background("Surveillance Grid").unwrap().id, "spies");
        assert_eq!(find_background("Device").unwrap().id, "wonder");
        assert_eq!(find_background("Dispositivo").unwrap().id, "wonder");
    }

    #[test]
    fn test_special_tables_backup_chantry_requisitions() {
        let backup = find_background("backup").unwrap();
        assert!(backup.backup_teams.is_some(), "Backup deve ter lista de equipes de exemplo");
        assert_eq!(backup.backup_teams.unwrap().len(), 15);

        let chantry = find_background("chantry").unwrap();
        assert!(chantry.chantry_pools.is_some(), "Capela deve ter tabela de piscinas de construção");
        assert_eq!(chantry.chantry_pools.unwrap().len(), 5);

        let req = find_background("requisitions").unwrap();
        assert!(req.requisitions_chart.is_some(), "Requisições deve ter tabela de lealdade/dificuldade");
        assert_eq!(req.requisitions_chart.unwrap().len(), 5);
        assert_eq!(req.requisitions_chart.unwrap()[0].difficulty, 9);
        assert_eq!(req.requisitions_chart.unwrap()[4].difficulty, 5);
    }
}
