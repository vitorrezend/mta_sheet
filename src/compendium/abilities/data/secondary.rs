//! Habilidades Secundárias Canônicas de M20 (Secondary Abilities).
//!
//! M20 pp. 296-301: Talentos Secundários, Perícias Secundárias e Conhecimentos Secundários.

use super::super::models::{AbilityCategory, AbilityDefinition, AbilityRating, AbilityScope};

// ============================================================================
// TALENTOS SECUNDÁRIOS
// ============================================================================

pub const ANIMAL_KINSHIP: AbilityDefinition = AbilityDefinition {
    id: "animal_kinship",
    name: "Animal Kinship",
    name_pt: "Afinidade Animal",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 296-297",
    description: "You have a natural affinity for animals, connecting through body language, primal vocalizations, and instinctive empathy to calm beasts, train creatures, and read mood or intent.",
    description_pt: "Você possui uma afinidade natural com animais, conectando-se através de linguagem corporal, vocalizações primitivas e empatia instintiva para acalmar feras, adestrar e ler intenções.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Domestic animals like you.", description_pt: "Animais domésticos gostam de você." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Skittish animals trust your gentle presence.", description_pt: "Animais ariscos confiam na sua presença calma." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Wild beasts treat you with respectful caution.", description_pt: "Feras selvagens o tratam com respeito e cautela." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Renowned beast whisperer capable of bonding with predators.", description_pt: "Conhecido como encantador de feras; doma predadores." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Your soul resonates more with beasts than with humans.", description_pt: "Sua alma ressoa mais com as feras do que com humanos." },
    ],
    possessed_by: "Shapeshifters, Rangers, Animal Trainers, Druids, Verbena, Feral Folk.",
    possessed_by_pt: "Transmorfos, Mateiros, Adestradores, Druidas, Verbena, Pessoas Selvagens.",
    suggested_specialties: &["Calming", "Training", "Canines", "Big Cats", "Horses", "Predators", "Bonding"],
    suggested_specialties_pt: &["Acalmar", "Adestramento", "Canídeos", "Felinos", "Cavalos", "Predadores", "Vínculo"],
};

pub const BLATANCY: AbilityDefinition = AbilityDefinition {
    id: "blatancy",
    name: "Blatancy",
    name_pt: "Descaramento",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 297",
    description: "Through theatrical flair, bold excuses ('We're filming a movie!'), and sheer audacity, you disguise blatant magical occurrences as special effects, stunts, or optical illusions.",
    description_pt: "Com talento teatral, desculpas esfarrapadas ('Estamos gravando um filme!') e audácia, você disfarça efeitos mágicos vulgares como efeitos especiais, pegadinhas ou ilusões de ótica.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Can fool small crowds with quick excuses.", description_pt: "Engana pequenos grupos com desculpas rápidas." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Uses props and camera crews to fake movie sets.", description_pt: "Usa câmeras e adereços para simular gravações." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Stage magician aura; people buy absurd explanations.", description_pt: "Aura de ilusionista; todos acreditam no truque." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "You make vulgar fireballs seem like flash-mob stunts.", description_pt: "Faz bolas de fogo parecerem pirotecnia de pegadinha." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Reality bends and Sleepers applaud the marvelous 'show'.", description_pt: "A realidade se rasga e os Adormecidos aplaudem o 'show'." },
    ],
    possessed_by: "Cultists of Ecstasy, Hollow Ones, Street Magicians, Pranksters, Actors.",
    possessed_by_pt: "Cultistas do Êxtase, Vazios, Ilusionistas de Rua, Trapaceiros, Atores.",
    suggested_specialties: &["Movie Dodges", "Street Magic", "Flash Mobs", "Special Effects", "Quick Distraction"],
    suggested_specialties_pt: &["Desculpas de Cinema", "Mágica de Rua", "Flash Mobs", "Efeitos Especiais", "Distração Rápida"],
};

pub const CAROUSING: AbilityDefinition = AbilityDefinition {
    id: "carousing",
    name: "Carousing",
    name_pt: "Farra",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 297",
    description: "The fine art of partying, drinking without collapsing, making fast friends in bars, discovering illicit nightlife hotspots, and turning revelry into social currency.",
    description_pt: "A arte de festejar intensamente, beber sem desmaiar, fazer amigos rápidos em baladas e bares, descobrir festas clandestinas e transformar noitadas em capital social.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Fun drinking buddy who handles beer well.", description_pt: "Bom parceiro de copo que aguenta cerveja." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Party animal familiar with nightlife etiquette.", description_pt: "Festeiro assíduo que conhece o circuito noturno." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Life of the party; gains trust and secrets while drinking.", description_pt: "Alma da festa; arranca segredos de copos cheios." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Legendary reveler drinking hardcore bikers under the table.", description_pt: "Bebedor lendário que aguenta uísque com motoqueiros." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Dionysian god of intoxication and unrestrained ecstasy.", description_pt: "Avatar dionisíaco do delírio, embriaguez e festa eterna." },
    ],
    possessed_by: "Cultists of Ecstasy, Bikers, Rock Stars, Bohemians, Sailors, College Students.",
    possessed_by_pt: "Cultistas do Êxtase, Motociclistas, Estrelas do Rock, Boêmios, Marinheiros.",
    suggested_specialties: &["Drinking Contests", "Club Crawling", "Schmoozing", "Endurance", "Secret Parties"],
    suggested_specialties_pt: &["Disputas de Bebida", "Roteiro de Baladas", "Entrosamento", "Resistência a Álcool", "Festas Secretas"],
};

pub const DO: AbilityDefinition = AbilityDefinition {
    id: "do",
    name: "Do",
    name_pt: "Do",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 297-298",
    description: "The sacred, esoteric martial art of the Akashic Brotherhood. Do blends fluid physical motion, meditation, Chi manipulation, pressure point mastery, and reality-shaping focus.",
    description_pt: "A arte marcial sagrada e esotérica da Irmandade de Akasha. O Do une movimento físico harmônico, meditação, canalização de Chi, pontos vitais e foco místico de alteração da realidade.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Student practicing foundational stances and breathing.", description_pt: "Iniciante dominando posturas básicas e respiração." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Graceful movements; strikes with focused balance.", description_pt: "Movimentos graciosos; desfere golpes em perfeito equilíbrio." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Adept whose strikes bypass armor using inner Chi.", description_pt: "Adepto cujos golpes canalizam Chi através da armadura." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Master able to deflect projectiles and leap impossible spans.", description_pt: "Mestre que deflete projéteis e salta distâncias surreais." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Enlightened warrior whose body and the cosmos move as one.", description_pt: "Guerreiro iluminado; seu corpo e o cosmos movem-se em uníssono." },
    ],
    possessed_by: "Akashic Brotherhood, Martial Hermits, Chi Masters.",
    possessed_by_pt: "Irmandade de Akasha, Eremitas Marciais, Mestres de Chi.",
    suggested_specialties: &["Chi Focus", "Deflection", "Pressure Points", "Fluid Kata", "Aerial Acrobatics"],
    suggested_specialties_pt: &["Foco de Chi", "Deflexão", "Pontos de Pressão", "Katas Fluídos", "Acrobacias Aéreas"],
};

pub const SEDUCTION: AbilityDefinition = AbilityDefinition {
    id: "seduction",
    name: "Seduction",
    name_pt: "Sedução",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 298",
    description: "The subtle craft of enticing, flirting, arousing desire, intoxicating minds, and making others crave your company, approval, or romantic intimacy.",
    description_pt: "A arte sutil de encantar, flertar, despertar desejos, fascinar mentes e fazer com que os outros anseiem por sua companhia, aprovação e intimidade.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Playful flirt who catches appreciative eyes.", description_pt: "Flertador divertido que chama a atenção em festas." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Alluring presence; knows how to flatter and tease.", description_pt: "Presença atraente; sabe elogiar e provocar no ponto certo." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Magnetic charmer; rarely leaves a venue alone.", description_pt: "Sedutor magnético; conquista pretendentes com facilidade." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Fatal attraction; targets obsess over your words.", description_pt: "Atração fatal; pessoas ficam obcecadas por seu toque." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Siren or Incubus; kings and mages sacrifice thrones for your kiss.", description_pt: "Sirene irresistível; reis e magos abrem mão de impérios por você." },
    ],
    possessed_by: "Cultists of Ecstasy, Courtesans, Spies, Models, Performers, Socialites.",
    possessed_by_pt: "Cultistas do Êxtase, Cortesãos, Espiões, Modelos, Socialites.",
    suggested_specialties: &["Subtle Flirting", "Dangerous Allure", "Sweet Talk", "Body Language", "Romance"],
    suggested_specialties_pt: &["Flerte Sutil", "Atração Perigosa", "Conversa Doce", "Linguagem Corporal", "Romance"],
};

pub const STYLE: AbilityDefinition = AbilityDefinition {
    id: "style",
    name: "Style",
    name_pt: "Estilo",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 298",
    description: "You project an unforgettable persona. Through couture, grooming, aesthetics, posture, and striking panache, your presence defines coolness and command.",
    description_pt: "Você projeta uma personalidade inesquecível. Por meio de moda, postura, elegância e atitude marcante, sua presença define o ápice do bom gosto e charme.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Dresses neatly and avoids fashion faux pas.", description_pt: "Veste-se bem e evita mancadas de moda." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Distinct personal aesthetic that stands out in a crowd.", description_pt: "Possui identidade visual marcante e bom gosto." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Trendsetter followed by friends and colleagues.", description_pt: "Dita moda entre amigos e colegas; sempre impecável." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Fashion icon; photographers and influencers emulate your look.", description_pt: "Ícone estético; fotógrafos e influencers copiam seu visual." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Living masterpiece of aesthetic perfection.", description_pt: "Obra de arte viva de refinamento e presença estética." },
    ],
    possessed_by: "Designers, Syndicate Dappermen, Hollow Ones, Rockers, Celebrities.",
    possessed_by_pt: "Estilistas, Homens de Terno do Sindicato, Vazios, Celebridades.",
    suggested_specialties: &["High Fashion", "Goth / Cyberpunk", "Corporate Chic", "Retro Vintage", "Hairstyling"],
    suggested_specialties_pt: &["Alta Costura", "Gótico / Cyberpunk", "Chique Corporativo", "Retro Vintage", "Cabelo e Acessórios"],
};

// ============================================================================
// PERÍCIAS SECUNDÁRIAS
// ============================================================================

pub const ACROBATICS: AbilityDefinition = AbilityDefinition {
    id: "acrobatics",
    name: "Acrobatics",
    name_pt: "Acrobacia",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 298-299",
    description: "Tumbling, gymnastic flips, tightrope balance, safe falls, trapeze routines, and leaping across rooftops with gravity-defying athletic precision.",
    description_pt: "Cambalhotas, saltos mortais, equilíbrio sobre cordas, amortecimento de quedas, trapézio e parkour acrobático desafiando a gravidade.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Can do cartwheels and breakfalls without injury.", description_pt: "Dá estrelinhas e sabe amortecer quedas sem se ferir." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "High-school gymnast capable of graceful vaulting.", description_pt: "Ginasta com saltos seguros e bom equilíbrio." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Professional circus acrobat or parkour runner.", description_pt: "Acrobata de circo profissional ou praticante avançado de parkour." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Olympic-level contender defying physical limits.", description_pt: "Nível olímpico executando acrobacias que parecem mágicas." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Cirque du Soleil headliner; dances through the air.", description_pt: "Estrela do Cirque du Soleil; parece voar sem asas." },
    ],
    possessed_by: "Gymnasts, Circus Performers, Parkour Runners, Stunt Artists, Ninjas.",
    possessed_by_pt: "Ginastas, Artistas Circenses, Praticantes de Parkour, Dublês, Ninjas.",
    suggested_specialties: &["Parkour", "Safe Falls", "Tightrope", "Flips", "Trapeze", "Aerial Arts"],
    suggested_specialties_pt: &["Parkour", "Quedas Seguras", "Corda Bamba", "Saltos Mortais", "Trapézio", "Tecido Acrobático"],
};

pub const ARCHERY: AbilityDefinition = AbilityDefinition {
    id: "archery",
    name: "Archery",
    name_pt: "Arquearia",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 299",
    description: "Shooting humanity's oldest ranged weapon: traditional longbows, compound hunting bows, and modern mechanical crossbows with silent, deadly accuracy.",
    description_pt: "Disparar a mais antiga arma de longo alcance da humanidade: arcos longos tradicionais, arcos compostos de caça e balestras mecânicas silenciosas e letais.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Summer-camp archer who hits targets at modest distance.", description_pt: "Arqueiro de clube de campo; acerta o alvo a distâncias curtas." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Hunting bowman; accurately bags game in the woods.", description_pt: "Caçador experiente; abate animais com tiros precisos." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Competitive marksman; splits arrows on the bullseye.", description_pt: "Arqueiro de competição; atinge a mosca do alvo repetidamente." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Master sniper silent as a shadow; shoots on the run.", description_pt: "Franco-atirador silencioso que dispara em corrida ou a cavalo." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Zen archer; arrows strike invisible targets through intuition.", description_pt: "Mestre zen; suas flechas acertam alvos ocultos guiadas pelo espírito." },
    ],
    possessed_by: "Hunters, Traditionalists, Zen Archers, Snipers, Medieval Reenactors.",
    possessed_by_pt: "Caçadores, Tradicionalistas, Arqueiros Zen, Atiradores de Emboscada.",
    suggested_specialties: &["Longbow", "Compound Bow", "Crossbow", "Trick Shots", "Horseback Archery"],
    suggested_specialties_pt: &["Arco Longo", "Arco Composto", "Besta / Balestra", "Tiros Especiais", "Arquearia Montada"],
};

pub const BIOTECH: AbilityDefinition = AbilityDefinition {
    id: "biotech",
    name: "Biotech",
    name_pt: "Biotecnologia",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 299",
    description: "Hands-on engineering of cybernetic implants, prosthetic interfaces, cloned tissue grafts, synthetic organs, and biological wetware.",
    description_pt: "Engenharia prática de implantes cibernéticos, próteses biônicas, enxertos de tecidos clonados, órgãos sintéticos e biomaquinário conectado ao sistema nervoso.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Can clean and calibrate basic cyber-prosthetics.", description_pt: "Sabe calibrar e higienizar próteses eletrônicas básicas." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Installs neural-jack adapters and diagnostic bio-chips.", description_pt: "Instala chips de diagnóstico e conectores neurais." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Progenitor or Iteration X engineer crafting cybernetic limbs.", description_pt: "Técnico da Iteração X capaz de fabricar membros cibernéticos." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Builds advanced organic wetware and combat enhancements.", description_pt: "Cria aprimoramentos orgânicos e biocircuitos de combate." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Merges biology and hypertech seamlessly into immortal transhumans.", description_pt: "Funde biologia e tecnologia de ponta criando trans-humanos perfeitos." },
    ],
    possessed_by: "Iteration X Bio-Engineers, Progenitors, Cyber-Surgeons, Transhumanists.",
    possessed_by_pt: "Engenheiros da Iteração X, Progenitores, Cirurgiões Cibernéticos.",
    suggested_specialties: &["Cyber-Limbs", "Neural Interfaces", "Synthetic Organs", "Cloning", "Bio-Sensors"],
    suggested_specialties_pt: &["Membros Biônicos", "Interfaces Neurais", "Órgãos Sintéticos", "Clonagem", "Biossensores"],
};

pub const ENERGY_WEAPONS: AbilityDefinition = AbilityDefinition {
    id: "energy_weapons",
    name: "Energy Weapons",
    name_pt: "Armas de Energia",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 299",
    description: "Operating, aiming, recharging, and maintaining Directed Energy Weapons: laser carbines, plasma projectors, particle blasters, and lightning cannons.",
    description_pt: "Operar, mirar, recarregar e calibrar Armas de Energia Direcionada: carabinas laser, projetores de plasma, blasters de partículas e canhões de raios.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Knows safety protocols for experimental blasters.", description_pt: "Conhece travas de segurança de blasters experimentais." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Accustomed to battery packs and plasma discharge recoil.", description_pt: "Acostumado ao recuo térmico e baterias de laser." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Void Engineer or Etherite marine firing with deadly aim.", description_pt: "Soldado do Éter ou do Vazio disparando rajadas letais." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Sharpshooter with heavy laser cannons and beam weapons.", description_pt: "Atirador de elite com canhões de feixe e desintegradores." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Vaporizes targets across dimensions with unerring beam fire.", description_pt: "Vaporiza alvos através de dimensões com disparos cirúrgicos." },
    ],
    possessed_by: "Sons of Ether, Void Engineers, Iteration X Shock Troops, Men in Black.",
    possessed_by_pt: "Filhos do Éter, Engenheiros do Vazio, Tropas da Iteração X.",
    suggested_specialties: &["Lasers", "Plasma Projectors", "Particle Beams", "Overcharging", "Heavy Blasters"],
    suggested_specialties_pt: &["Lasers", "Projetores de Plasma", "Feixes de Partículas", "Sobrecarga", "Blasters Pesados"],
};

pub const PILOT: AbilityDefinition = AbilityDefinition {
    id: "pilot",
    name: "Pilot",
    name_pt: "Pilotagem",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 300",
    description: "Flying aircraft, helicopters, drones, jets, submersibles, and experimental Void Engineer Void-craft through terrestrial skies or deep space.",
    description_pt: "Pilotar aviões, helicópteros, caças a jato, drones, submarinos e naves espaciais dos Engenheiros do Vazio na atmosfera ou no espaço profundo.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Small single-prop Cessna pilot in clear daylight.", description_pt: "Piloto de monomotor Cessna em pistas civis de dia." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Commercial twin-engine pilot flying through storms.", description_pt: "Piloto comercial bimotor enfrentando turbulência." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Helicopter combat pilot or acrobatic flyer.", description_pt: "Piloto militar de helicóptero ou acrobata aéreo." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Fighter jet ace maneuvering in high-G dogfights.", description_pt: "Ás da aviação a jato executando manobras extremas." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Voidcraft commander steering through black holes and nebula storms.", description_pt: "Comandante estelar navegando por anomalias cósmicas." },
    ],
    possessed_by: "Void Engineers, Aviators, Military Pilots, Drone Operators, Smugglers.",
    possessed_by_pt: "Engenheiros do Vazio, Aviadores, Pilotos de Caça, Contrabandistas.",
    suggested_specialties: &["Jets", "Helicopters", "Void-craft", "Combat Maneuvers", "Bad Weather"],
    suggested_specialties_pt: &["Caças a Jato", "Helicópteros", "Naves do Vazio", "Dogfights", "Tempestades"],
};

// ============================================================================
// CONHECIMENTOS SECUNDÁRIOS
// ============================================================================

pub const AREA_KNOWLEDGE: AbilityDefinition = AbilityDefinition {
    id: "area_knowledge",
    name: "Area Knowledge",
    name_pt: "Conhecimento de Área",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 300",
    description: "Intimate local familiarity with a specific city, district, or wilderness zone: shortcuts, hideouts, local legends, power brokers, and hidden escape routes.",
    description_pt: "Familiaridade íntima com uma cidade, bairro ou região selvagem específica: atalhos, esconderijos, lendas locais, pessoas de influência e rotas de fuga.",
    ratings: &[
        AbilityRating { dots: 1, title: "Dabbler", title_pt: "Curioso", description: "Knows major avenues, hospitals, and landmarks.", description_pt: "Conhece avenidas principais, hospitais e pontos de referência." },
        AbilityRating { dots: 2, title: "Student", title_pt: "Estudante", description: "Knows side alleys, local gossip, and traffic bottlenecks.", description_pt: "Conhece ruelas, fofocas locais e gargalos de trânsito." },
        AbilityRating { dots: 3, title: "Scholar", title_pt: "Erudito", description: "Knows secret rooftops, abandoned basements, and police beats.", description_pt: "Conhece telhados secretos, porões e rondas policiais." },
        AbilityRating { dots: 4, title: "Professor", title_pt: "Professor", description: "Encyclopedic grasp of subterranean tunnels and corrupt officials.", description_pt: "Domínio de túneis subterrâneos e contatos corruptos locais." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "The city's heartbeat flows through your veins.", description_pt: "A cidade é o seu quintal; nada ocorre ali sem você saber." },
    ],
    possessed_by: "Vigilantes, Taxi Drivers, Local Cops, Gang Leaders, Informants, Smugglers.",
    possessed_by_pt: "Vigilantes, Taxistas, Policiais Locais, Chefes de Bairro, Informantes.",
    suggested_specialties: &["Shortcuts", "Hideouts", "Local Politics", "Underground Tunnels", "Gangs"],
    suggested_specialties_pt: &["Atalhos", "Esconderijos", "Política Local", "Túneis Subterrâneos", "Gangues"],
};

pub const BELIEF_SYSTEMS: AbilityDefinition = AbilityDefinition {
    id: "belief_systems",
    name: "Belief Systems",
    name_pt: "Sistemas de Crença",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 300-301",
    description: "Study of world religions, cultural worldviews, ideologies, and local dogmas. Essential for crafting coincidental magick aligned with local Consensus reality zones.",
    description_pt: "Estudo das religiões mundiais, visões culturais, ideologias e dogmas locais. Crucial para formular mágika coincidente alinhada às Zonas de Realidade do Consenso.",
    ratings: &[
        AbilityRating { dots: 1, title: "Dabbler", title_pt: "Curioso", description: "Basic understanding of major world religions.", description_pt: "Noções das grandes religiões mundiais e seus feriados." },
        AbilityRating { dots: 2, title: "Student", title_pt: "Estudante", description: "Understands local sectarian tensions and theological concepts.", description_pt: "Compreende tensões sectárias e ritos de fé locais." },
        AbilityRating { dots: 3, title: "Scholar", title_pt: "Erudito", description: "Anthropologist mapping cultural belief patterns.", description_pt: "Antropólogo mapeando crenças e mitos comunitários." },
        AbilityRating { dots: 4, title: "Professor", title_pt: "Professor", description: "Expert on Consensus mechanics and Reality Zones.", description_pt: "Especialista em Zonas de Realidade e Consenso coletivo." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "You know how to reshape beliefs to validate impossible wonders.", description_pt: "Sabe como remodelar a crença popular para validar milagres." },
    ],
    possessed_by: "Theologians, Anthropologists, Hermetics, Celestial Chorus, NWO Operatives.",
    possessed_by_pt: "Teólogos, Antropólogos, Herméticos, Coro Celestial, Agentes da NOV.",
    suggested_specialties: &["Reality Zones", "Christianity", "Paganism", "Technocracy Paradigm", "Islam", "Buddhism"],
    suggested_specialties_pt: &["Zonas de Realidade", "Cristianismo", "Paganismo", "Paradigma Tecnocrata", "Islã", "Budismo"],
};

pub const CRYPTOGRAPHY: AbilityDefinition = AbilityDefinition {
    id: "cryptography",
    name: "Cryptography",
    name_pt: "Criptografia",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 301",
    description: "The mathematical science of data encryption, key generation, steganography, digital hashing, and cracking top-secret ciphers.",
    description_pt: "A ciência matemática de encriptação de dados, geração de chaves, esteganografia, hashes digitais e quebra de cifras ultra-secretas.",
    ratings: &[
        AbilityRating { dots: 1, title: "Dabbler", title_pt: "Curioso", description: "Can use PGP keys and substitution ciphers.", description_pt: "Sabe usar chaves PGP e cifras de substituição." },
        AbilityRating { dots: 2, title: "Student", title_pt: "Estudante", description: "Understands RSA, hashing algorithms, and public-key systems.", description_pt: "Compreende algoritmos RSA, hashes e chaves assimétricas." },
        AbilityRating { dots: 3, title: "Scholar", title_pt: "Erudito", description: "Information security analyst auditing banking encryption.", description_pt: "Analista de segurança bancária auditando protocolos criptográficos." },
        AbilityRating { dots: 4, title: "Professor", title_pt: "Professor", description: "NSA cryptanalyst designing impenetrable quantum-resistant algorithms.", description_pt: "Criptoanalista sênior criando cifras resistentes a computação quântica." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Turing incarnate; decodes any digital or mystical secret.", description_pt: "Turing renascido; quebra qualquer chave binária ou hermética existente." },
    ],
    possessed_by: "Virtual Adepts, NWO Cryptographers, Syndicate Financiers, Hackers.",
    possessed_by_pt: "Adeptos da Virtualidade, Criptógrafos da NOV, Financiadores do Sindicato.",
    suggested_specialties: &["Quantum Cryptography", "Ciphers", "Steganography", "Decryption", "Blockchain"],
    suggested_specialties_pt: &["Criptografia Quântica", "Cifras", "Esteganografia", "Decriptação", "Blockchain"],
};

pub const DEMOLITIONS: AbilityDefinition = AbilityDefinition {
    id: "demolitions",
    name: "Demolitions",
    name_pt: "Demolições",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 301",
    description: "Handling, assembling, defusing, and detonating explosives: C4, dynamite, blasting caps, shape charges, and controlled structural demolition.",
    description_pt: "Armar, desarmar, fabricar e detonar explosivos: C4, dinamite, espoletas, cargas moldadas e implosão técnica de estruturas.",
    ratings: &[
        AbilityRating { dots: 1, title: "Dabbler", title_pt: "Curioso", description: "Understands gunpowder mixing and blast-distance safety.", description_pt: "Conhece misturas de pólvora e distâncias seguras de detonação." },
        AbilityRating { dots: 2, title: "Student", title_pt: "Estudante", description: "Can handle commercial dynamite and mining blasting caps.", description_pt: "Sabe manusear dinamite comercial e estopins de mineração." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Military combat engineer using C4 and breach charges.", description_pt: "Engenheiro militar de combate utilizando C4 e cargas de arrombamento." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Bomb squad defuser or controlled implosion architect.", description_pt: "Perito de esquadrão antibombas ou arquiteto de implosão predial." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Surgical demolitions master making mountains vanish cleanly.", description_pt: "Mestre explosivista capaz de derrubar arranha-céus sem trincar o vidro ao lado." },
    ],
    possessed_by: "Combat Engineers, Bomb Squads, Miners, Iteration X, Commando Operatives.",
    possessed_by_pt: "Engenheiros de Combate, Esquadrões Antibombas, Mineiros, Forças Especiais.",
    suggested_specialties: &["Bomb Disposal", "Shape Charges", "Controlled Implosion", "IEDs", "Industrial Blasting"],
    suggested_specialties_pt: &["Desarmar Bombas", "Cargas Moldadas", "Implosão Controlada", "Explosivos Improvisados", "Detonação Industrial"],
};

pub const SECONDARY_ABILITIES: &[AbilityDefinition] = &[
    ANIMAL_KINSHIP,
    BLATANCY,
    CAROUSING,
    DO,
    SEDUCTION,
    STYLE,
    ACROBATICS,
    ARCHERY,
    BIOTECH,
    ENERGY_WEAPONS,
    PILOT,
    AREA_KNOWLEDGE,
    BELIEF_SYSTEMS,
    CRYPTOGRAPHY,
    DEMOLITIONS,
];
