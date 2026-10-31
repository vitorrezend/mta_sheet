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
    page_ref: "M20, pp. 279-280",
    description: "You create, repair, customize, and maintain physical objects with your hands and tools: carpentry, mechanics, electronics, leatherwork, metallurgy, and artisanal trades.",
    description_pt: "Você cria, conserta, customiza e mantém objetos físicos com suas mãos e ferramentas: marcenaria, mecânica, eletrônica, trabalho em couro, metalurgia e artes manuais.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You sorta know what you’re doing.",
            description_pt: "Você mais ou menos sabe o que está fazendo.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Basic skills have given way to working knowledge.",
            description_pt: "Habilidades básicas deram lugar a conhecimento prático.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You’re pretty damn good, if you must say so yourself!",
            description_pt: "Você é bom pra caramba, se tiver que admitir!",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "You’re highly respected within your field.",
            description_pt: "Você é altamente respeitado em sua área.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "There’s little you can’t do when you set your mind to doing it.",
            description_pt: "Há muito pouco que você não consiga fazer quando se dispõe a isso.",
        },
    ],
    possessed_by: "Inventors, Mechanics, Artisans, Bohemians, Servants, Recreationists, Technicians, Wanderers, Survivalists, Mad Scientists, Obsessed Superheroes.",
    possessed_by_pt: "Inventores, Mecânicos, Artesãos, Boêmios, Servos, Recriadores Históricos, Técnicos, Andarilhos, Sobreviventes, Cientistas Loucos, Super-heróis Obcecados.",
    suggested_specialties: &[
        "Armor", "Weaponsmith", "Carpentry", "Invention", "Tailoring", "Design", "Vehicle Repair", "Home Repair", "Leatherwork", "High-Tech Tinkering",
    ],
    suggested_specialties_pt: &[
        "Armaduras", "Forja de Armas", "Carpintaria", "Invenção", "Alfaiataria", "Design", "Reparo de Veículos", "Reparos Domésticos", "Trabalho em Couro", "Engenhoca de Alta Tecnologia",
    ],
};

pub const DRIVE: AbilityDefinition = AbilityDefinition {
    id: "drive",
    name: "Drive",
    name_pt: "Condução",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 280",
    description: "Your competence behind the wheel of automobiles, trucks, motorcycles, and everyday motor vehicles in normal and hazardous driving conditions.",
    description_pt: "Sua competência ao volante de automóveis, caminhões, motocicletas e veículos motorizados convencionais, tanto em tráfego pacífico quanto sob condições de direção arriscadas.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Sunday driver.",
            description_pt: "Motorista de domingo.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Typical American.",
            description_pt: "Americano típico.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "New York taxi driver.",
            description_pt: "Taxista de Nova York.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Professional racer.",
            description_pt: "Piloto profissional de corrida.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Bond — James Bond.",
            description_pt: "Bond — James Bond.",
        },
    ],
    possessed_by: "Chauffeurs, Instructors, Truckers, Daredevils, Delivery Drivers, Secret Agents, Getaway Specialists, Stunt Artists.",
    possessed_by_pt: "Choferes, Instrutores, Caminhoneiros, Destemidos, Entregadores, Agentes Secretos, Especialistas em Fuga, Dublês.",
    suggested_specialties: &[
        "Racing", "Pursuit", "Shadowing", "Stunts", "Vintage Cars", "Heavy Trucks", "Long-Distance", "Shaking Tails", "Bad Conditions", "Snow and Ice", "Off-Road", "Showing Off",
    ],
    suggested_specialties_pt: &[
        "Corridas", "Perseguição", "Seguir Alvos", "Manobras Radicais", "Carros Clássicos", "Caminhões Pesados", "Longa Distância", "Despistar Perseguidores", "Condições Adversas", "Neve e Gelo", "Fora de Estrada", "Exibicionismo",
    ],
};

pub const ETIQUETTE: AbilityDefinition = AbilityDefinition {
    id: "etiquette",
    name: "Etiquette",
    name_pt: "Etiqueta",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 280-281",
    description: "Understanding social customs, diplomatic protocols, high-society codes, subcultural mores, and how to conduct yourself gracefully in formal or delicate gatherings.",
    description_pt: "Compreensão de costumes sociais, protocolos diplomáticos, códigos da alta sociedade e tradições de subculturas, sabendo comportar-se com elegância e diplomacia em reuniões formais ou delicadas.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You can attend fine restaurants without embarrassing yourself.",
            description_pt: "Você frequenta restaurantes finos sem passar vergonha.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "You’ve got impressive manners and a sense for saying the right things.",
            description_pt: "Você tem maneiras impressionantes e tato para dizer a coisa certa.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Wonderfully cultured, you know your way around a meal, party, or conversation.",
            description_pt: "Maravilhosamente culto, você sabe como se portar em banquetes, festas ou conversas.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "A social chameleon, you put folks at ease almost anywhere.",
            description_pt: "Um camaleão social, você deixa as pessoas à vontade em quase qualquer lugar.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "You’re a valued guest in nearly any situation.",
            description_pt: "Você é um convidado estimado em quase qualquer situação.",
        },
    ],
    possessed_by: "Diplomats, Emissaries, Negotiators, Nobility, Executives, World Travelers, Social Climbers, Old-Money Heirs, Spies.",
    possessed_by_pt: "Diplomatas, Emissários, Negociadores, Nobreza, Executivos, Viajantes do Mundo, Alpinistas Sociais, Herdeiros de Famílias Tradicionais, Espiões.",
    suggested_specialties: &[
        "High Society", "Royalty", "Dining", "Subcultures", "International Travel", "Umbral Courts", "Master Mages", "Internet Culture", "Digital Web", "Technocratic Politesse",
    ],
    suggested_specialties_pt: &[
        "Alta Sociedade", "Realeza", "Jantares Formais", "Subculturas", "Viagens Internacionais", "Cortes Umbrais", "Mestres Magos", "Cultura da Internet", "Teia Digital", "Polidez Tecnocrática",
    ],
};

pub const FIREARMS: AbilityDefinition = AbilityDefinition {
    id: "firearms",
    name: "Firearms",
    name_pt: "Armas de Fogo",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 281",
    description: "Training in shooting, cleaning, reloading, clearing jams, and maintaining pistols, rifles, shotguns, and submachine guns under combat pressure.",
    description_pt: "Treinamento em disparar, limpar, recarregar, destravar e manter pistolas, rifles, espingardas e submetralhadoras sob a pressão do combate.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Gun range newbie.",
            description_pt: "Iniciante em estande de tiro.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Hunter.",
            description_pt: "Caçador.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Typical cop or soldier.",
            description_pt: "Policial ou soldado comum.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Sharpshooter.",
            description_pt: "Atirador de precisão.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Frank Castle.",
            description_pt: "Frank Castle.",
        },
    ],
    possessed_by: "Assassins, Survivalists, Washed-Up Rock Stars, Police, Military Personnel, Enforcers, Bodyguards, Militia Members, Gang Members, Black Suits, Self-Defense Enthusiasts, Pissed-Off Crazy People.",
    possessed_by_pt: "Assassinos, Sobrevivencialistas, Astros do Rock Decadentes, Policiais, Militares, Capangas, Guarda-costas, Membros de Milícias, Membros de Gangues, Ternos Pretos, Entusiastas de Autodefesa, Loucos Furiosos.",
    suggested_specialties: &[
        "Sniper", "Quick-Draw", "Target Shooting", "Rifles", "Handguns", "Drive-Bys", "Military Hardware", "Heavy Weapons", "Vintage Firearms", "Cybernetic Weaponry",
    ],
    suggested_specialties_pt: &[
        "Atirador de Elite", "Saque Rápido", "Tiro ao Alvo", "Rifles", "Pistolas", "Disparos em Trânsito", "Equipamento Militar", "Armas Pesadas", "Armas de Fogo Antigas", "Armamento Cibernético",
    ],
};

pub const MARTIAL_ARTS: AbilityDefinition = AbilityDefinition {
    id: "martial_arts",
    name: "Martial Arts",
    name_pt: "Artes Marciais",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 281",
    description: "Disciplined systems of combat: Karate, Kung Fu, Krav Maga, Aikido, Capoeira, or Jujitsu. Unlike Brawl, Martial Arts represents formalized katas, inner focus, and precise bio-kinetic techniques.",
    description_pt: "Sistemas formais e disciplinados de combate: Karatê, Kung Fu, Krav Magá, Aikidô, Capoeira ou Jiu-Jitsu. Diferente da Briga, envolve katas formalizados, foco interior e técnicas biomecânicas precisas.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Raw beginner.",
            description_pt: "Iniciante cru.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Experienced student.",
            description_pt: "Estudante experiente.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Devotee.",
            description_pt: "Devoto.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Accomplished combatant.",
            description_pt: "Combatente consumado.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Formidable master.",
            description_pt: "Mestre formidável.",
        },
    ],
    possessed_by: "Cops, Soldiers, Gang Members, Old Masters, Black Suits, Self-Defense Advocates, Athletes, Stunt Artists, The Most Unlikely People.",
    possessed_by_pt: "Policiais, Soldados, Membros de Gangues, Velhos Mestres, Ternos Pretos, Praticantes de Autodefesa, Atletas, Dublês, As Pessoas Mais Improváveis.",
    suggested_specialties: &[
        "Soft Style (snake, monkey, drunken style, etc.)", "Hard Style (tiger, mantis, etc.)", "Aikido", "Judo", "Karate", "Muay Thai", "Kuei Lung Chuan", "etc",
    ],
    suggested_specialties_pt: &[
        "Estilo Suave (serpente, macaco, estilo bêbado, etc.)", "Estilo Duro (tigre, louva-a-deus, etc.)", "Aikidô", "Judô", "Karatê", "Muay Thai", "Kuei Lung Chuan", "etc",
    ],
};

pub const MEDITATION: AbilityDefinition = AbilityDefinition {
    id: "meditation",
    name: "Meditation",
    name_pt: "Meditação",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 281",
    description: "The mental discipline of calming thoughts, channeling inner energies, enduring pain or sensory overload, restoring Willpower, and entering focused trance states.",
    description_pt: "A disciplina mental de silenciar os pensamentos, canalizar energias interiores, suportar dor ou sobrecarga sensorial, recuperar Força de Vontade e entrar em estados de transe focado.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You can ground and center for short periods of time.",
            description_pt: "Você consegue se aterrar e se centrar por períodos curtos.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Regular practice has deepened and extended your abilities.",
            description_pt: "A prática regular aprofundou e expandiu suas habilidades.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Even under distracting conditions, you know how to find your center.",
            description_pt: "Mesmo sob condições desconcertantes, você sabe encontrar seu centro.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "“Use the Force, Luke…”",
            description_pt: "“Use a Força, Luke…”",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "An island of calm is yours whenever you want or need it.",
            description_pt: "Uma ilha de calma é sua sempre que você quiser ou precisar.",
        },
    ],
    possessed_by: "Akashics, Yogis, Monks, Hypnotists, Therapists, Clergy, Martial Artists, Hippies.",
    possessed_by_pt: "Akáshicos, Iogues, Monges, Hipnotizadores, Terapeutas, Clérigos, Artistas Marciais, Hippies.",
    suggested_specialties: &[
        "Zen", "Yoga", "Do", "Trance", "Nodes", "Relaxation", "Therapy", "Tantra", "T’ai Chi", "Virtual Reality", "Focus", "Meditative Movement", "Grace Under Pressure",
    ],
    suggested_specialties_pt: &[
        "Zen", "Ioga", "Dô", "Transe", "Nodos", "Relaxamento", "Terapia", "Tantra", "Tai Chi", "Realidade Virtual", "Foco", "Movimento Meditativo", "Graça sob Pressão",
    ],
};

pub const MELEE: AbilityDefinition = AbilityDefinition {
    id: "melee",
    name: "Melee",
    name_pt: "Armas Brancas",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 281-282",
    description: "Expertise in hand-held weapons: knives, swords, axes, clubs, staves, spears, and improvised weapons like broken bottles or lead pipes.",
    description_pt: "Proficiência com armas empunhadas de combate corpo a corpo: facas, espadas, machados, porretes, bastões, lanças e armas improvisadas como garrafas quebradas ou canos de chumbo.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You can swing a club or blade around without hurting yourself… much.",
            description_pt: "Você consegue brandir um porrete ou lâmina sem se cortar… muito.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "You’ve earned a bit of skill… maybe even enough for a real-life fight.",
            description_pt: "Você adquiriu alguma destreza… talvez até suficiente para uma luta real.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You’re kinda badass.",
            description_pt: "Você é bem casca-grossa.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "You’re seriously badass.",
            description_pt: "Você é seriamente casca-grossa.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Your skill intimidates the pros.",
            description_pt: "Sua habilidade intimida os profissionais.",
        },
    ],
    possessed_by: "Cops, Military Personnel, Spies, Killers, Outlaws, Martial Artists, Street Folks, Barroom Brawlers, SCA Members.",
    possessed_by_pt: "Policiais, Militares, Espiões, Assassinos, Foras da Lei, Artistas Marciais, Pessoal das Ruas, Brigões de Bar, Membros da SCA.",
    suggested_specialties: &[
        "Blades", "Polearms", "Staves", "Stunts", "Combat Sports", "Weapons of Opportunity", "Kendo", "Fencing", "Martial Arts Weapons",
    ],
    suggested_specialties_pt: &[
        "Lâminas", "Armas de Haste", "Bastões", "Acrobacias", "Esportes de Combate", "Armas de Oportunidade", "Kendô", "Esgrima", "Armas de Artes Marciais",
    ],
};

pub const RESEARCH: AbilityDefinition = AbilityDefinition {
    id: "research",
    name: "Research",
    name_pt: "Pesquisa",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 282",
    description: "The methodical art of tracking down facts, documents, obscure grimoires, archives, databases, historical records, and missing information.",
    description_pt: "A arte metódica de rastrear dados, documentos, grimórios antigos, arquivos, bancos de dados, registros históricos e informações desaparecidas.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Buffy.",
            description_pt: "Buffy.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Anya.",
            description_pt: "Anya.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Willow.",
            description_pt: "Willow.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Giles.",
            description_pt: "Giles.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Fred.",
            description_pt: "Fred.",
        },
    ],
    possessed_by: "Monster Hunters, Research Assistants, Archive Keepers, Librarians, Detectives, Writers, Journalists, Scientists.",
    possessed_by_pt: "Caçadores de Monstros, Assistentes de Pesquisa, Guardiões de Arquivos, Bibliotecários, Detetives, Escritores, Jornalistas, Cientistas.",
    suggested_specialties: &[
        "Arcane Collections", "Forbidden Lore", "Hidden Sources", "Secrets", "Net Searches", "Oral Traditions", "specific topics (Crime, Medicine, Politics, History, etc.)",
    ],
    suggested_specialties_pt: &[
        "Coleções Arcanas", "Conhecimento Proibido", "Fontes Ocultas", "Segredos", "Buscas na Rede", "Tradições Orais", "tópicos específicos (Crime, Medicina, Política, História, etc.)",
    ],
};

pub const STEALTH: AbilityDefinition = AbilityDefinition {
    id: "stealth",
    name: "Stealth",
    name_pt: "Furtividade",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 282-283",
    description: "The skill of moving quietly, utilizing cover, shadowing targets, hiding in shadows, evading surveillance cameras, and passing unnoticed.",
    description_pt: "A perícia de mover-se sem fazer barulho, aproveitar coberturas, seguir alvos sem ser notado, sumir nas sombras, burlar câmeras de vigilância e passar despercebido.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "With dark clothes, you might not be spotted at night.",
            description_pt: "Com roupas escuras, você talvez não seja visto à noite.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "You often sneak out past your parents.",
            description_pt: "Você costuma sair de fininho sem seus pais notarem.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "A genuine sneak, you elude casual detection.",
            description_pt: "Um verdadeiro furtivo, você escapa da detecção casual.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Just doin’ what a ninja’s gotta do…",
            description_pt: "Apenas fazendo o que um ninja precisa fazer…",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Batman’s taking notes.",
            description_pt: "O Batman está tomando notas.",
        },
    ],
    possessed_by: "Spies, Burglars, Runaways, Star-Crossed Lovers, Hunters, Assassins, Vigilantes.",
    possessed_by_pt: "Espiões, Ladrões, Fugitivos, Amantes Secretos, Caçadores, Assassinos, Vigilantes.",
    suggested_specialties: &[
        "Blending Into Shadows", "Woodlands", "Alleys", "Concealment", "Squeaky Floors", "Moving Silently", "Shadowing a Target",
    ],
    suggested_specialties_pt: &[
        "Misturar-se às Sombras", "Florestas", "Becos", "Ocultação", "Pisos Rangentes", "Movimento Silencioso", "Seguir um Alvo",
    ],
};

pub const SURVIVAL: AbilityDefinition = AbilityDefinition {
    id: "survival",
    name: "Survival",
    name_pt: "Sobrevivência",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 283",
    description: "Staying alive in wilderness and harsh conditions: foraging for food, purifying water, building shelters, tracking beasts, building fires, and navigating without GPS.",
    description_pt: "Permanecer vivo em ambientes selvagens ou hostis: coletar alimentos, purificar água, construir abrigos, rastrear animais, acender fogueiras e navegar sem GPS.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You’ve done some day-hikes on established trails.",
            description_pt: "Você já fez algumas caminhadas diurnas em trilhas conhecidas.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Plenty of time outdoors has taught you decent skills.",
            description_pt: "Muito tempo ao ar livre lhe ensinou boas habilidades.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You can live off the land, make traps, and spot most obvious hazards.",
            description_pt: "Você consegue viver da terra, criar armadilhas e notar perigos óbvios.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "You’d probably survive the Hunger Games.",
            description_pt: "Você provavelmente sobreviveria a Jogos Vorazes.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "The wilderness is your full-time home.",
            description_pt: "A natureza selvagem é o seu lar em tempo integral.",
        },
    ],
    possessed_by: "Hunters, Hikers, Shamans, Survivalists, Fugitives, Reality TV Contestants, Rural Folk, Rangers, Members of Indigenous Cultures.",
    possessed_by_pt: "Caçadores, Trilheiros, Xamãs, Sobrevivencialistas, Fugitivos, Participantes de Reality Shows, Habitantes Rurais, Guardas Florestais, Membros de Culturas Indígenas.",
    suggested_specialties: &[
        "Old-School Wisdom", "Traps", "Shelters", "Weather Hazards", "End-Times Preparedness", "specific environments (Desert, Jungle, Woodlands, Suburbs, etc.)",
    ],
    suggested_specialties_pt: &[
        "Sabedoria Tradicional", "Armadilhas", "Abrigos", "Perigos Climáticos", "Preparação para o Fim dos Tempos", "ambientes específicos (Deserto, Selva, Bosques, Subúrbios, etc.)",
    ],
};

pub const TECHNOLOGY: AbilityDefinition = AbilityDefinition {
    id: "technology",
    name: "Technology",
    name_pt: "Tecnologia",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 283",
    description: "Understanding, building, modifying, and operating contemporary machines, electrical grids, communication arrays, sensors, robotics, and industrial apparatus.",
    description_pt: "Compreender, operar, modificar e construir maquinários modernos, redes elétricas, sistemas de comunicação, sensores, robótica e aparatos industriais.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Normal household technology is a snap for you.",
            description_pt: "Tecnologia doméstica comum é moleza para você.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Basic household, IT, and automotive maintenance and repairs are easy enough to manage, and you know how to tinker around with simple electronic tech.",
            description_pt: "Manutenção básica doméstica, de TI e automotiva é fácil de gerenciar, e você sabe fazer pequenos reparos com tecnologia eletrônica simples.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You’ve got enough mechanics savvy to create your own minor-league tech, repair most everyday items, and work simple technomagick with the goods at your command.",
            description_pt: "Você tem conhecimento mecânico suficiente para criar sua própria tecnologia de pequeno porte, consertar itens cotidianos e praticar tecnomágika simples com os materiais ao seu dispor.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "You’re a whiz with typical technology, invent some pretty advanced stuff, and whip up impressive gadgets outside the range of everyday Sleepertech.",
            description_pt: "Você é um prodígio com tecnologia comum, inventa coisas bastante avançadas e desenvolve engenhocas impressionantes muito além da tecnologia dos Adormecidos.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "A technological artisan, you can design, construct, operate, and repair unique devices and astonishing inventions. If you employ technomagick, then your creations surpass Sleepertech by a pretty wide margin.",
            description_pt: "Um artesão tecnológico, você pode projetar, construir, operar e consertar dispositivos únicos e invenções espantosas. Se empregar tecnomágika, suas criações superam a tecnologia dos Adormecidos por uma margem impressionante.",
        },
    ],
    possessed_by: "Technicians, Inventors, Repair Specialists, Technomancers.",
    possessed_by_pt: "Técnicos, Inventores, Especialistas em Reparos, Tecnomantes.",
    suggested_specialties: &[
        "Electronics", "Vehicles", "Modification", "Invention", "Media Tech", "Technomagick",
    ],
    suggested_specialties_pt: &[
        "Eletrônica", "Veículos", "Modificação", "Invenção", "Tecnologia de Mídia", "Tecnomágika",
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
