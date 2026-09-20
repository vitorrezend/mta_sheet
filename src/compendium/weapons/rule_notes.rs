//! Notas Canônicas de Regras de Combate (M20 pp. 450-453)

use crate::i18n::Language;
use super::models::{WeaponClass, WeaponRuleNote};

// --- Notas de Armas Brancas / Melee (M20, pp. 450-451) ---
pub const NOTE_1: WeaponRuleNote = WeaponRuleNote {
    code: "#1",
    title: "Two-Handed Heavy Weapon",
    title_pt: "Arma Pesada de Duas Mãos",
    description: "Two-handed weapon; very heavy – requires min. Strength 3 to employ.",
    description_pt: "Arma de duas mãos; muito pesada – requer Força mín. 3 para empunhar.",
};

pub const NOTE_2: WeaponRuleNote = WeaponRuleNote {
    code: "#2",
    title: "Two Hands Required",
    title_pt: "Exigência de Duas Mãos",
    description: "Requires two hands to employ properly; +2 difficulty if used one-handed.",
    description_pt: "Requer duas mãos para usar adequadamente; +2 de dificuldade se usada com uma mão.",
};

pub const NOTE_3: WeaponRuleNote = WeaponRuleNote {
    code: "#3",
    title: "Stabbing / Impaling",
    title_pt: "Estocada / Empalação",
    description: "May be used for stabbing or impaling at +1 difficulty.",
    description_pt: "Pode ser usada para perfurar ou empalar com +1 de dificuldade.",
};

pub const NOTE_4: WeaponRuleNote = WeaponRuleNote {
    code: "#4",
    title: "Slicing / Slashing",
    title_pt: "Corte / Fatiamento",
    description: "May be used for slicing or slashing attacks at -1 difficulty.",
    description_pt: "Pode ser usada para ataques de corte ou golpe deslizante com -1 de dificuldade.",
};

pub const NOTE_5: WeaponRuleNote = WeaponRuleNote {
    code: "#5",
    title: "Disarm Maneuver",
    title_pt: "Manobra de Desarme",
    description: "May be used to disarm an opponent without botch penalties.",
    description_pt: "Pode ser usada para desarmar o oponente sem penalidade de falha crítica.",
};

pub const NOTE_6: WeaponRuleNote = WeaponRuleNote {
    code: "#6",
    title: "Sweep / Trip Maneuver",
    title_pt: "Manobra de Rasteira / Derrubada",
    description: "May be used to execute a sweep or trip maneuver.",
    description_pt: "Pode ser usada para executar rasteiras ou manobras de derrubada.",
};

pub const NOTE_7: WeaponRuleNote = WeaponRuleNote {
    code: "#7",
    title: "Thrown Weapon Option",
    title_pt: "Opção de Arma Arremessável",
    description: "Can be thrown up to (Strength x 3) yards at difficulty 6.",
    description_pt: "Pode ser arremessada a uma distância de até (Força x 3) jardas com dificuldade 6.",
};

pub const NOTE_8: WeaponRuleNote = WeaponRuleNote {
    code: "#8",
    title: "Shove / Knockdown",
    title_pt: "Empurrão / Derrubada Contundente",
    description: "Allows a shove or knockdown maneuver inflicting standard bashing damage.",
    description_pt: "Permite empurrões ou manobras de derrubada causando dano contundente padrão.",
};

pub const NOTE_9: WeaponRuleNote = WeaponRuleNote {
    code: "#9",
    title: "Self-Harm on Botch",
    title_pt: "Dano Próprio em Falha Crítica",
    description: "On a botch, user inflicts damage upon self.",
    description_pt: "Em uma falha crítica, o usuário inflige dano a si mesmo.",
};

pub const NOTE_10: WeaponRuleNote = WeaponRuleNote {
    code: "#10",
    title: "Entangle Limb",
    title_pt: "Prender Membro",
    description: "May be used to entangle an enemy’s limb at +1 difficulty.",
    description_pt: "Pode ser usado para prender um membro do inimigo com +1 de dificuldade.",
};

pub const ALL_MELEE_RULE_NOTES: &[WeaponRuleNote] = &[
    NOTE_1, NOTE_2, NOTE_3, NOTE_4, NOTE_5,
    NOTE_6, NOTE_7, NOTE_8, NOTE_9, NOTE_10,
];

pub const ALL_RULE_NOTES: &[WeaponRuleNote] = ALL_MELEE_RULE_NOTES;

// --- Notas de Armas de Fogo, Arcos & Militares (M20, pp. 452-453) ---
pub const RANGED_NOTE_1: WeaponRuleNote = WeaponRuleNote {
    code: "#1",
    title: "Full-Auto, Bursts & Strafing Sprays",
    title_pt: "Fogo Automático, Rajadas de 3 Tiros & Disparos em Varredura",
    description: "Gun may fire full-auto, three-round bursts, and strafing sprays (see Firearms Maneuvers, pp. 434-436).",
    description_pt: "A arma pode disparar em modo totalmente automático, rajadas de três tiros e fogo em varredura (ver Manobras de Armas de Fogo, pp. 434-436).",
};

pub const RANGED_NOTE_2: WeaponRuleNote = WeaponRuleNote {
    code: "#2",
    title: "Technocracy Laser Sights & Citizen Clearance",
    title_pt: "Mira Laser da Tecnocracia & Autorização de Cidadão",
    description: "All category weapons except chain-gun feature laser sights and can be used by Sleepers and extraordinary citizens; issued only to Technocracy personnel of extraordinary citizen level or higher.",
    description_pt: "Todas as armas desta categoria (exceto a chain-gun) possuem miras laser integradas e podem ser operadas por Adormecidos; emitidas apenas para agentes da Tecnocracia com classificação de cidadão extraordinário ou superior.",
};

pub const RANGED_NOTE_3: WeaponRuleNote = WeaponRuleNote {
    code: "#3",
    title: "Composite Revolver & Rapid Chambering",
    title_pt: "Revólver de Compósito & Carregamento Seletivo",
    description: "Revolver; at the touch of a button, can chamber any type of ammunition loaded in the cylinder. Both Model R and Model A made of composite materials – invisible to metal detectors.",
    description_pt: "Revólver; ao simples toque de um botão, carrega na câmara qualquer munição disposta no tambor. Modelos R e A são forjados em compósito de polímero/carbono — invisíveis a detectores de metal comuns.",
};

pub const RANGED_NOTE_4: WeaponRuleNote = WeaponRuleNote {
    code: "#4",
    title: "Enlightened Consensus Sensitivity",
    title_pt: "Sensibilidade ao Consenso / Risco para Não-Iluminados",
    description: "Rarely issued to unEnlightened personnel, as it tends to jam on a failed roll during unEnlightened use; still, more attuned to Consensus than it was years ago.",
    description_pt: "Raramente distribuída a pessoal Não-Iluminado, pois tende a emperrar em falhas de jogadas quando empunhada por adormecidos; ainda assim, encontra-se mais sintonizada com o Consenso do que em décadas passadas.",
};

pub const RANGED_NOTE_5: WeaponRuleNote = WeaponRuleNote {
    code: "#5",
    title: "Bolan Mk. 13 Heavy Hybrid System",
    title_pt: "Sistema Híbrido Pesado Bolan Mk. 13",
    description: "Combines a submachine gun main weapon (as per AK-47; 50-shot clip) with auto-shotgun capabilities (as MPS AA-12; 20-shot clip) through the main barrel, plus a grenade launcher under that barrel (as a MP-79/ six-shot magazine). Laser sights. Heavy; requires Strength 3 or better to employ but has relatively low recoil.",
    description_pt: "Combina fuzil/submetralhadora principal (estilo AK-47; pente 50 tiros) com espingarda automática (estilo MPS AA-12; pente 20 tiros) pelo cano principal, e lança-granadas acoplado inferior (estilo MP-79; pente 6 tiros). Miras laser. Muito pesada; requer Força 3+ para empunhar com recuo moderado.",
};

pub const RANGED_NOTE_6: WeaponRuleNote = WeaponRuleNote {
    code: "#6",
    title: "Draw, Nock & Reload Automatic Action",
    title_pt: "Ação de Armar, Puxar & Recarregar de Arcos/Bestas",
    description: "Long and short bows take an automatic action to nock and draw; crossbows require two automatic actions to reload. A character with Archery 3 or higher can nock, draw, and loose an arrow as a single action but cannot do the same thing with a crossbow.",
    description_pt: "Arcos longos e curtos consomem uma ação automática para encaixar e puxar a flecha; bestas requerem duas ações automáticas para recarregar. Personagens com Arquearia 3+ podem armar, puxar e disparar em uma única ação, mas o mesmo benefício não se aplica a bestas mecânicas.",
};

pub const RANGED_NOTE_7: WeaponRuleNote = WeaponRuleNote {
    code: "#7",
    title: "Collapsible Stealth Frame",
    title_pt: "Estrutura Tática Retrátil / Dobrável",
    description: "Collapsible; requires one turn to unfold from storage configuration, plus one action to load once it has been unfolded.",
    description_pt: "Dobrável; requer um turno para desdobrar da maleta ou configuração oculta de transporte, mais uma ação para municiar assim que montada.",
};

pub const RANGED_NOTE_8: WeaponRuleNote = WeaponRuleNote {
    code: "#8",
    title: "Bashing Damage & Dice Pool Debuff",
    title_pt: "Dano Contundente & Penalidade de Parada de Dados",
    description: "Bashing damage, not lethal; adds no extra damage from successes scored. Tear gas and Technocratic pacification spray also reduce target’s dice pools by two dice for one turn per success.",
    description_pt: "Causa dano contundente, não letal; sucessos extras não aumentam o dano da parada. Gás lacrimogêneo e spray de pacificação da Tecnocracia reduzem a parada de dados da vítima em 2 dados durante 1 turno por sucesso obtido.",
};

pub const RANGED_NOTE_9: WeaponRuleNote = WeaponRuleNote {
    code: "#9",
    title: "Military License & Heavy Weapons Specialization",
    title_pt: "Licença Militar Classe C & Especialização em Armas Pesadas",
    description: "Requires Firearms specialization in Heavy Weapons and a Class C license to own legally in U.S.",
    description_pt: "Exige especialização de Armas de Fogo em 'Armas Pesadas' e uma licença governamental Classe C para posse/porte legal em território civil.",
};

pub const RANGED_NOTE_10: WeaponRuleNote = WeaponRuleNote {
    code: "#10",
    title: "Vehicle-Mounted Only (Non-Portable)",
    title_pt: "Montagem em Veículo / Tripé (Não-Portátil a Pé)",
    description: "Not man-portable; must be mounted on vehicle or fixed heavy tripod.",
    description_pt: "Incapaz de ser disparada por um indivíduo solto a pé; necessita ser instalada sobre torre veicular, afuste ou tripé fixo de solo.",
};

pub const RANGED_NOTE_11: WeaponRuleNote = WeaponRuleNote {
    code: "#11",
    title: "HIT Mark Chain-Gun (Halves Armor)",
    title_pt: "Metralhadora Rotativa HIT Mark (Perfura Armadura pela Metade)",
    description: "Chain-gun; not normally man-portable, but carried by some HIT Marks and heavy-duty cyborgs. Halves armor protection due to high-speed “drilling” effect.",
    description_pt: "Metralhadora rotativa (chain-gun); não manuseável por pessoas normais, empunhada por ciborgues pesados e HIT Marks. Corta a proteção da armadura do alvo pela metade devido ao efeito de perfuração em vórtice contínuo.",
};

pub const RANGED_NOTE_12: WeaponRuleNote = WeaponRuleNote {
    code: "#12",
    title: "Explosive Blast / Area Effect",
    title_pt: "Dano Explosivo em Área (Tabela de Explosivos)",
    description: "See Explosives chart (M20 pp. 454-455). Explodes upon impact inflicting area shockwave damage.",
    description_pt: "Consulte a tabela de Explosivos (M20 pp. 454-455). Detona com o impacto causando onda de choque e estilhaços em raio esférico.",
};

pub const ALL_RANGED_RULE_NOTES: &[WeaponRuleNote] = &[
    RANGED_NOTE_1, RANGED_NOTE_2, RANGED_NOTE_3, RANGED_NOTE_4,
    RANGED_NOTE_5, RANGED_NOTE_6, RANGED_NOTE_7, RANGED_NOTE_8,
    RANGED_NOTE_9, RANGED_NOTE_10, RANGED_NOTE_11, RANGED_NOTE_12,
];

pub fn explain_note(code: &str, lang: Language) -> Option<(&'static str, &'static str)> {
    let trimmed = code.trim();
    if let Some(n) = ALL_MELEE_RULE_NOTES.iter().find(|n| n.code == trimmed) {
        return Some((n.title(lang), n.description(lang)));
    }
    if let Some(n) = ALL_RANGED_RULE_NOTES.iter().find(|n| n.code == trimmed) {
        return Some((n.title(lang), n.description(lang)));
    }
    None
}

pub fn explain_weapon_note(code: &str, lang: Language, w_class: WeaponClass) -> Option<(&'static str, &'static str)> {
    let trimmed = code.trim();
    match w_class {
        WeaponClass::Ranged => {
            if let Some(n) = ALL_RANGED_RULE_NOTES.iter().find(|n| n.code == trimmed) {
                return Some((n.title(lang), n.description(lang)));
            }
        }
        _ => {
            if let Some(n) = ALL_MELEE_RULE_NOTES.iter().find(|n| n.code == trimmed) {
                return Some((n.title(lang), n.description(lang)));
            }
        }
    }
    explain_note(code, lang)
}
