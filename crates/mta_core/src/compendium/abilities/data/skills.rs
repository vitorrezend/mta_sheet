//! Perícias Canônicas de M20 (Core Skills).
//!
//! Capítulo 6: Criação do Personagem (pp. 282-288).

use super::super::models::{AbilityCategory, AbilityDefinition, AbilityRating, AbilityScope};

pub const CRAFTS: AbilityDefinition = AbilityDefinition {
    id: "crafts",
    name: "Crafts",
    name_pt: "Ofícios",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 282-283",
    description: "You create, repair, customize, and maintain physical objects with your hands and tools: carpentry, mechanics, electronics, leatherwork, metallurgy, and artisanal trades.",
    description_pt: "Você cria, conserta, customiza e mantém objetos físicos com suas mãos e ferramentas: marcenaria, mecânica, eletrônica, trabalho em couro, metalurgia e artes manuais.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Handy with basic tools; can fix a leaky pipe.",
            description_pt: "Jeitoso com ferramentas básicas; conserta um cano furado.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Competent trade skills and garage tinkering.",
            description_pt: "Prática em marcenaria caseira ou mecânica de garagem.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Skilled artisan or professional mechanic.",
            description_pt: "Artesão qualificado ou mecânico profissional de oficina.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Master craftsperson producing high-grade equipment.",
            description_pt: "Mestre artesão criando itens de alta durabilidade e acabamento.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Legendary forger or engineer whose works approach perfection.",
            description_pt: "Ferreiro ou engenheiro lendário cujas criações beiram a perfeição.",
        },
    ],
    possessed_by: "Mechanics, Blacksmiths, Artists, Technicians, Tinkers, Inventors, Survivalists.",
    possessed_by_pt: "Mecânicos, Ferreiros, Artistas, Técnicos, Engenheiros Fazedores, Inventores.",
    suggested_specialties: &[
        "Carpentry", "Mechanics", "Metalwork", "Electronics", "Weaponsmithing", "Custom Tuning", "Armor",
    ],
    suggested_specialties_pt: &[
        "Marcenaria", "Mecânica", "Metalurgia", "Eletrônica", "Armaria", "Customização de Motores", "Armaduras",
    ],
};

pub const DRIVE: AbilityDefinition = AbilityDefinition {
    id: "drive",
    name: "Drive",
    name_pt: "Condução",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 283-284",
    description: "Your competence behind the wheel of automobiles, trucks, motorcycles, and everyday motor vehicles in normal and hazardous driving conditions.",
    description_pt: "Sua competência ao volante de automóveis, caminhões, motocicletas e veículos motorizados convencionais, tanto em tráfego pacífico quanto sob perseguições extremas.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Licensed commuter who drives safely in good weather.",
            description_pt: "Motorista habilitado que dirige sem sustos com tempo bom.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Experienced driver accustomed to bad traffic and foul weather.",
            description_pt: "Acostumado ao trânsito pesado, estradas de terra e chuva forte.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Taxi or delivery driver; sharp reflexes in chases.",
            description_pt: "Taxista veterano ou piloto com reflexos afiados em perseguição.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Professional stunt driver or circuit racer.",
            description_pt: "Piloto de fuga profissional ou corredor de pistas fechadas.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Total vehicular mastery; performs impossible automotive stunts.",
            description_pt: "Domínio veicular absoluto; realiza manobras que desafiam a física.",
        },
    ],
    possessed_by: "Chauffeurs, Truckers, Getaway Drivers, Stunt Drivers, Police Officers, Racers.",
    possessed_by_pt: "Choferes, Caminhoneiros, Pilotos de Fuga, Dublês Automotivos, Policiais, Corredores.",
    suggested_specialties: &[
        "Chases", "Motorcycles", "Heavy Trucks", "Off-Road", "Stunts", "Wet Weather", "High Speed",
    ],
    suggested_specialties_pt: &[
        "Perseguições", "Motos", "Caminhões Pesados", "Off-Road", "Manobras Radicais", "Chuva e Lama", "Alta Velocidade",
    ],
};

pub const ETIQUETTE: AbilityDefinition = AbilityDefinition {
    id: "etiquette",
    name: "Etiquette",
    name_pt: "Etiqueta",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 284-285",
    description: "Understanding social customs, diplomatic protocols, high-society codes, subcultural mores, and how to conduct yourself gracefully in formal or delicate gatherings.",
    description_pt: "Compreensão de normas sociais, protocolos diplomáticos, códigos da alta sociedade e tradições de subculturas, sabendo comportar-se com elegância e diplomacia.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Polite and doesn't embarrass himself at fancy dinners.",
            description_pt: "Educado; sabe usar talheres e não passa vergonha em jantares finos.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Cultured and knows the unwritten rules of business gatherings.",
            description_pt: "Culto; compreende as regras implícitas de reuniões e coquetéis.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Social chameleon who navigates elite balls and underground clubs.",
            description_pt: "Camaleão social que circula com desenvoltura na alta roda e em clubes secretos.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Seasoned diplomat or royal courtier effortlessly defusing gaffes.",
            description_pt: "Diplomata refinado capaz de desarmar gafes graves com um sorriso.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "The gold standard of poise; shapes global protocol and fashion.",
            description_pt: "O padrão máximo de refinamento; dita tendências de elegância internacional.",
        },
    ],
    possessed_by: "Diplomats, Nobles, Courtiers, Politicians, Spies, High Executives.",
    possessed_by_pt: "Diplomatas, Nobres, Cortesãos, Políticos, Espiões, Altos Executivos.",
    suggested_specialties: &[
        "High Society", "Corporate", "Occult Gatherings", "Technocratic Protocol", "Formal Dining", "Diplomacy",
    ],
    suggested_specialties_pt: &[
        "Alta Sociedade", "Corporativo", "Encontros Ocultistas", "Protocolo Tecnocrata", "Jantares Formais", "Diplomacia",
    ],
};

pub const FIREARMS: AbilityDefinition = AbilityDefinition {
    id: "firearms",
    name: "Firearms",
    name_pt: "Armas de Fogo",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 285",
    description: "Training in shooting, cleaning, reloading, clearing jams, and maintaining pistols, rifles, shotguns, and submachine guns under combat pressure.",
    description_pt: "Habilidade de disparar, recarregar, destravar, limpar e manter pistolas, rifles, espingardas e submetralhadoras com precisão mortal sob fogo cruzado.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Gun-range hobbyist; knows firearm safety basics.",
            description_pt: "Atirador de clube de tiro; conhece noções básicas de segurança.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Competent hunter or armed security guard.",
            description_pt: "Caçador habilidoso ou segurança patrimonial armado.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Combat cop or infantry soldier who shoots straight under fire.",
            description_pt: "Policial de choque ou fuzileiro que acerta o alvo sob estresse de combate.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "SWAT marksman or seasoned mercenary assassin.",
            description_pt: "Atirador de elite de forças táticas ou assassino mercenário calejado.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Legendary sniper capable of pinpoint accuracy at extraordinary range.",
            description_pt: "Sniper lendário que atinge alvos milimétricos a distâncias extremas.",
        },
    ],
    possessed_by: "Soldiers, Police Officers, Hunters, Mercenaries, Bodyguards, Assassins.",
    possessed_by_pt: "Soldados, Policiais, Caçadores, Mercenários, Guarda-costas, Atiradores.",
    suggested_specialties: &[
        "Pistols", "Rifles", "Shotguns", "Sniper", "Quick Draw", "Trick Shots", "Submachine Guns",
    ],
    suggested_specialties_pt: &[
        "Pistolas", "Fuzis", "Espingardas", "Sniper", "Saque Rápido", "Tiros de Precisão", "Submetralhadoras",
    ],
};

pub const MARTIAL_ARTS: AbilityDefinition = AbilityDefinition {
    id: "martial_arts",
    name: "Martial Arts",
    name_pt: "Artes Marciais",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 285-286",
    description: "Disciplined systems of combat: Karate, Kung Fu, Krav Maga, Aikido, Capoeira, or Jujitsu. Unlike Brawl, Martial Arts represents formalized katas, inner focus, and precise bio-kinetic techniques.",
    description_pt: "Sistemas formais de combate e defesa pessoal: Karatê, Kung Fu, Krav Magá, Aikidô, Capoeira, Jiu-Jitsu. Diferente da Briga, envolve katas disciplinados, foco mental e precisão biomecânica.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Yellow-belt student practicing standard forms.",
            description_pt: "Faixa amarela praticando formas básicas na academia.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Competent martial artist with sharp defensive maneuvers.",
            description_pt: "Praticante com bons reflexos defensivos e esquivas precisas.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Black belt; dangerous in sparring and real engagements.",
            description_pt: "Faixa preta; oponente perigoso tanto no dojo quanto na rua.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Dojo sensei or competitive martial arts champion.",
            description_pt: "Mestre de dojo ou campeão em torneios de artes marciais.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Grandmaster whose strikes harness bio-energy and lethal economy.",
            description_pt: "Grão-mestre lendário que canaliza energia chi e eficiência fulminante.",
        },
    ],
    possessed_by: "Akashics, Monks, Special Forces, Bodyguards, Tournament Fighters.",
    possessed_by_pt: "Irmãos de Akasha, Monges, Forças Especiais, Guarda-costas, Lutadores de Torneio.",
    suggested_specialties: &[
        "Joint Locks", "Kicks", "Internal Energy", "Defensive Counters", "Fluid Forms", "Vital Strikes",
    ],
    suggested_specialties_pt: &[
        "Chaves de Articulação", "Chutes Circulares", "Energia Interna", "Contragolpes Defensivos", "Formas Fluídas", "Golpes em Pontos Vitais",
    ],
};

pub const MEDITATION: AbilityDefinition = AbilityDefinition {
    id: "meditation",
    name: "Meditation",
    name_pt: "Meditação",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 286",
    description: "The mental discipline of calming thoughts, channeling inner energies, enduring pain or sensory overload, restoring Willpower, and entering focused trance states.",
    description_pt: "A disciplina mental de silenciar os pensamentos, canalizar energia interior, suportar dores ou sobrecarga sensorial, recuperar Força de Vontade e entrar em estados de transe reflexivo.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You can calm breathing and relax after a stressful day.",
            description_pt: "Consegue relaxar a respiração e desacelerar após um dia estressante.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Capable of screening out minor environmental distractions.",
            description_pt: "Capaz de isolar distrações ambientais moderadas ao concentrar-se.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You achieve deep contemplative states even under noise.",
            description_pt: "Atinge transe contemplativo profundo mesmo em locais barulhentos.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "You regulate heart rate and master pain through iron focus.",
            description_pt: "Controla batimentos cardíacos e transcende a dor física por puro foco.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Bodhisattva serenity; inner stillness completely unshakeable.",
            description_pt: "Serenidade de um Bodhisattva; harmonia interna imune a qualquer caos.",
        },
    ],
    possessed_by: "Monks, Akashics, Hermetics, Shamans, Ascetics, Cyber-Zen Enthusiasts.",
    possessed_by_pt: "Monges, Akashicos, Herméticos, Xamãs, Ascetas, Entusiastas do Zen.",
    suggested_specialties: &[
        "Trance States", "Breathing Exercises", "Enduring Pain", "Restoring Willpower", "Sensory Deprivation",
    ],
    suggested_specialties_pt: &[
        "Estados de Transe", "Exercícios Respiratórios", "Suportar Dor", "Recuperar Força de Vontade", "Privação Sensorial",
    ],
};

pub const MELEE: AbilityDefinition = AbilityDefinition {
    id: "melee",
    name: "Melee",
    name_pt: "Armas Brancas",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 286-287",
    description: "Expertise in hand-held weapons: knives, swords, axes, clubs, staves, spears, and improvised weapons like broken bottles or lead pipes.",
    description_pt: "Proficiência com armas brancas de combate corpo a corpo: facas, espadas, machados, bastões, lanças e armas improvisadas como garrafas quebradas ou canos de aço.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Can swing a bat or brandish a pocket knife effectively.",
            description_pt: "Sabe brandir um taco de beisebol ou canivete sem se cortar.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Trained in fencing, escrima, or street blade fighting.",
            description_pt: "Praticante de esgrima, eskrima ou combate de facas de rua.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Lethal blade user who parries, disarms, and strikes vital zones.",
            description_pt: "Lutador afiado que apara golpes, desarma e golpeia áreas letais.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Master swordsman or historical weapons combat master.",
            description_pt: "Mestre espadachim ou instrutor de combate com armas brancas medievais.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Legendary blade master; weapon feels like a natural limb.",
            description_pt: "Guerreiro lendário; a lâmina é uma extensão perfeita de seu corpo.",
        },
    ],
    possessed_by: "Duelists, Gang Members, Historical Reenactors, Martial Artists, Soldiers.",
    possessed_by_pt: "Duelistas, Faccionados, Praticantes de HEMA, Artistas Marciais, Assassinos.",
    suggested_specialties: &[
        "Swords", "Knives", "Staves", "Improvised Weapons", "Disarming", "Parries", "Axes",
    ],
    suggested_specialties_pt: &[
        "Espadas", "Facas", "Bastões", "Armas Improvisadas", "Desarme", "Aparar", "Machados",
    ],
};

pub const RESEARCH: AbilityDefinition = AbilityDefinition {
    id: "research",
    name: "Research",
    name_pt: "Pesquisa",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 287",
    description: "The methodical art of tracking down facts, documents, obscure grimoires, archives, databases, historical records, and missing information.",
    description_pt: "A arte metódica de rastrear dados, documentos, grimórios antigos, arquivos secretos, bancos de dados digitais, certidões históricas e pistas investigativas.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Adefficient web searcher and public library visitor.",
            description_pt: "Pesquisador digital competente em motores de busca e bibliotecas.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Capable of unearthing hard-to-find public records.",
            description_pt: "Capaz de desenterrar certidões e registros públicos obscuros.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Investigative journalist or scholarly archivist.",
            description_pt: "Nível de jornalista investigativo ou arquivista acadêmico minucioso.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "You cross-reference obscure leads across multiple hidden sources.",
            description_pt: "Cruza pistas herméticas em arquivos lacrados com velocidade impressionante.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "No historical or digital secret stays hidden from your inquiry.",
            description_pt: "Nenhum segredo histórico ou documento lacrado permanece oculto a você.",
        },
    ],
    possessed_by: "Scholars, Journalists, Private Detectives, Hermetics, Technocrats, Archivists.",
    possessed_by_pt: "Acadêmicos, Jornalistas, Detetives Particulares, Herméticos, Tecnocratas, Arquivistas.",
    suggested_specialties: &[
        "Old Grimoires", "Digital Archives", "Genealogy", "Corporate Records", "Cross-referencing",
    ],
    suggested_specialties_pt: &[
        "Grimórios Antigos", "Arquivos Digitais", "Genealogia", "Registros Corporativos", "Cruzamento de Dados",
    ],
};

pub const STEALTH: AbilityDefinition = AbilityDefinition {
    id: "stealth",
    name: "Stealth",
    name_pt: "Furtividade",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 287-288",
    description: "The skill of moving quietly, utilizing cover, shadowing targets, hiding in shadows, evading surveillance cameras, and passing unnoticed.",
    description_pt: "A perícia de mover-se sem fazer barulho, aproveitar coberturas, seguir alvos sem ser notado, sumir nas sombras e burlar câmeras e sensores de vigilância.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Can tiptoe past sleeping guards in dimly lit corridors.",
            description_pt: "Consegue andar na ponta dos pés em corredores escuros.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Understands shadows, vantage points, and sound masking.",
            description_pt: "Aproveita sombras, pontos cegos e ruídos de fundo para se ocultar.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Skilled burglar or military scout moving unseen.",
            description_pt: "Ladrão experiente ou batedor militar que desliza sem deixar vestígios.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Ninja or black-ops operative who ghosts past high-tech sensors.",
            description_pt: "Operador de operações secretas que passa por sensores como um fantasma.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "You can vanish before someone's eyes in plain sight.",
            description_pt: "Você parece desaparecer diante dos olhos de alguém em plena luz do dia.",
        },
    ],
    possessed_by: "Thieves, Commandos, Spies, Assassins, Hunters, Investigators.",
    possessed_by_pt: "Ladrões, Comandos Militares, Espiões, Assassinos, Caçadores, Infiltradores.",
    suggested_specialties: &[
        "Shadowing", "Silent Movement", "Camouflage", "Hiding Objects", "Urban Cover", "Wilderness",
    ],
    suggested_specialties_pt: &[
        "Seguir Alvos", "Passos Silenciosos", "Camuflagem", "Ocultar Objetos", "Cobertura Urbana", "Ermos",
    ],
};

pub const SURVIVAL: AbilityDefinition = AbilityDefinition {
    id: "survival",
    name: "Survival",
    name_pt: "Sobrevivência",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 288",
    description: "Staying alive in wilderness and harsh conditions: foraging for food, purifying water, building shelters, tracking beasts, building fires, and navigating without GPS.",
    description_pt: "Permanecer vivo em ambientes selvagens ou hostis: coletar alimentos, purificar água, construir abrigos, rastrear animais, acender fogueiras e navegar sem GPS.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Weekend camper who can pitch a tent and start a campfire.",
            description_pt: "Campista de fim de semana que monta barraca e acende fogueira.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Hunter who can navigate woods and find potable water.",
            description_pt: "Caçador que se orienta pela mata e encontra água potável.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Forest ranger or survivalist capable of thriving off the grid.",
            description_pt: "Guarda florestal ou mateiro capaz de viver meses isolado da civilização.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Wilderness master who survives deserts, tundras, and jungles.",
            description_pt: "Especialista que sobrevive em desertos, tundras árticas ou pântanos.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Nature's child; survives anywhere on Earth or exotic Umbral realms.",
            description_pt: "Filho da natureza; sobrevive em qualquer canto da Terra ou da Umbra.",
        },
    ],
    possessed_by: "Rangers, Scouts, Shamans, Explorers, Primitive Tribes, Hermits.",
    possessed_by_pt: "Guardas Florestais, Batedores, Xamãs, Exploradores, Mateiros, Eremitas.",
    suggested_specialties: &[
        "Foraging", "Shelter", "Tracking", "Extreme Cold", "Desert", "Jungle", "Navigation",
    ],
    suggested_specialties_pt: &[
        "Forragear", "Construção de Abrigos", "Rastreamento", "Frio Extremo", "Deserto", "Selva", "Navegação Estelar",
    ],
};

pub const TECHNOLOGY: AbilityDefinition = AbilityDefinition {
    id: "technology",
    name: "Technology",
    name_pt: "Tecnologia",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 288",
    description: "Understanding, building, modifying, and operating contemporary machines, electrical grids, communication arrays, sensors, robotics, and industrial apparatus.",
    description_pt: "Compreender, operar, modificar e construir maquinários modernos, redes elétricas, sistemas de telecomunicações, sensores, robótica e aparatos industriais.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Can rewire a household appliance or replace car fuses.",
            description_pt: "Sabe trocar fiação de eletrodomésticos e substituir fusíveis.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Understands circuit boards, power tools, and basic transmitters.",
            description_pt: "Compreende placas de circuito, ferramentas elétricas e transmissores.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Skilled technician; custom-builds radio transmitters and rigs.",
            description_pt: "Técnico qualificado; constrói rádios, geradores e bancadas eletrônicas.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Robotics engineer or hardware architect modifying complex devices.",
            description_pt: "Engenheiro de robótica que projeta hardwares e modifica sensores avançados.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Visionary technologist creating revolutionary machinery.",
            description_pt: "Tecnólogo visionário capaz de criar maquinários revolucionários.",
        },
    ],
    possessed_by: "Engineers, Iteration X Cyborgs, Sons of Ether, Virtual Adepts, Repair Techs.",
    possessed_by_pt: "Engenheiros, Ciborgues da Iteração X, Filhos do Éter, Adeptos da Virtualidade.",
    suggested_specialties: &[
        "Robotics", "Sensors", "Circuits", "Communications", "Power Grids", "Hardware Modification",
    ],
    suggested_specialties_pt: &[
        "Robótica", "Sensores", "Circuitos", "Comunicações", "Redes de Energia", "Modificação de Hardware",
    ],
};

pub const CORE_SKILLS: &[AbilityDefinition] = &[
    CRAFTS,
    DRIVE,
    ETIQUETTE,
    FIREARMS,
    MARTIAL_ARTS,
    MEDITATION,
    MELEE,
    RESEARCH,
    STEALTH,
    SURVIVAL,
    TECHNOLOGY,
];
