//! Conhecimentos Canônicos de M20 (Core Knowledges).
//!
//! Capítulo 6: Criação do Personagem (pp. 288-295).

use super::super::models::{AbilityCategory, AbilityDefinition, AbilityRating, AbilityScope};

pub const ACADEMICS: AbilityDefinition = AbilityDefinition {
    id: "academics",
    name: "Academics",
    name_pt: "Acadêmicos",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 288-289",
    description: "Scholastic education and humanities: history, literature, philosophy, languages, sociology, and classical liberal arts that expand a character's intellectual horizon.",
    description_pt: "Educação formal e humanidades: história, literatura, filosofia, línguas clássicas, sociologia e erudição que expandem os horizontes intelectuais do personagem.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Solid high school education; broad general culture.",
            description_pt: "Boa educação de ensino médio; cultura geral sólida.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "College undergraduate with clear specialization.",
            description_pt: "Estudante universitário com especialização acadêmica em andamento.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Master's degree level; publishes scholarly essays.",
            description_pt: "Nível de mestrado; publica artigos acadêmicos com rigor metodológico.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Tenured professor whose insights are cited across academia.",
            description_pt: "Professor titular com pesquisas citadas internacionalmente.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "World-renowned scholar reshaping historical or philosophical paradigms.",
            description_pt: "Erudito mundialmente consagrado que redefine paradigmas históricos.",
        },
    ],
    possessed_by: "Scholars, Professors, Hermetics, Philosophers, Writers, Technocrats.",
    possessed_by_pt: "Acadêmicos, Professores, Herméticos, Filósofos, Escritores, Tecnocratas.",
    suggested_specialties: &[
        "History", "Philosophy", "Literature", "Art History", "Anthropology", "Classical Latin", "Sociology",
    ],
    suggested_specialties_pt: &[
        "História", "Filosofia", "Literatura", "História da Arte", "Antropologia", "Latim Clássico", "Sociologia",
    ],
};

pub const COMPUTER: AbilityDefinition = AbilityDefinition {
    id: "computer",
    name: "Computer",
    name_pt: "Computador",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 289-290",
    description: "Expertise in IT, coding, operating systems, networking, digital security, programming languages, cryptography, and navigating the Digital Web.",
    description_pt: "Domínio de informática, desenvolvimento de software, arquitetura de redes, segurança digital, criptografia, sistemas operacionais e navegação na Rede Digital.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Comfortable with office suites, web search, and basic settings.",
            description_pt: "Familiarizado com suítes de escritório, navegação e configurações comuns.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Can write scripts, manage databases, and troubleshoot home networks.",
            description_pt: "Escreve scripts, gerencia bancos de dados e resolve panes de rede.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Professional software engineer or network sysadmin.",
            description_pt: "Engenheiro de software ou administrador de redes corporativas.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Elite hacker or cybersecurity architect breaking complex firewalls.",
            description_pt: "Hacker de elite ou arquiteto de segurança que rompe firewalls avançados.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Digital architect who bends cyberspace and algorithms to their will.",
            description_pt: "Arquiteto digital lendário que molda o ciberespaço e a Rede Digital à vontade.",
        },
    ],
    possessed_by: "Virtual Adepts, NWO Agents, Programmers, Hackers, IT Analysts, Technomancers.",
    possessed_by_pt: "Adeptos da Virtualidade, Agentes da NOV, Programadores, Hackers, Analistas de TI.",
    suggested_specialties: &[
        "Hacking", "Network Architecture", "Cryptography", "Digital Web", "Artificial Intelligence", "Cyber-Forensics",
    ],
    suggested_specialties_pt: &[
        "Hacking", "Arquitetura de Redes", "Criptografia", "Rede Digital", "Inteligência Artificial", "Perícia Cibernética",
    ],
};

pub const COSMOLOGY: AbilityDefinition = AbilityDefinition {
    id: "cosmology",
    name: "Cosmology / Subdimensions",
    name_pt: "Cosmologia / Subdimensões",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 290-291",
    description: "Understanding the structure of the Otherworlds: the Penumbra, Near Umbra, Horizon, Deep Umbra, Astral Planes, and Technocratic subdimensional manifolds.",
    description_pt: "Compreensão da estrutura dos Outros Mundos: a Penumbra, Umbra Próxima, Horizonte, Umbra Profunda, Planos Astrais e os coletores subdimensionais da Tecnocracia.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Knows that an invisible spirit world exists beyond the Gauntlet.",
            description_pt: "Sabe que um mundo espiritual invisível existe além da Película.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Familiar with Gauntlet ratings, Moon-paths, and basic Umbral geography.",
            description_pt: "Familiarizado com densidades da Película, Trilhas Lunares e geografia umbral.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Understands Astral courts, Deep Umbra hazards, and Horizon Realms.",
            description_pt: "Compreende cortes astrais, perigos da Umbra Profunda e Reinos do Horizonte.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Navigator of distant Umbral spheres, Shard Realms, and Paradox domains.",
            description_pt: "Navegador de esferas cósmicas remotas, Reinos Fragmentados e domínios de Paradoxo.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Cosmic cartographer whose knowledge spans the totality of the Tellurian.",
            description_pt: "Cartógrafo cósmico cujo saber abrange a totalidade misteriosa do Telúrico.",
        },
    ],
    possessed_by: "Dreamspeakers, Hermetics, Void Engineers, Shamans, Explorers of the Beyond.",
    possessed_by_pt: "Oradores dos Sonhos, Herméticos, Engenheiros do Vazio, Xamãs, Navegadores.",
    suggested_specialties: &[
        "Astral Plane", "Deep Umbra", "Spirit Courts", "Subdimensions", "Horizon Realms", "Avatar Storm",
    ],
    suggested_specialties_pt: &[
        "Plano Astral", "Umbra Profunda", "Cortes Espirituais", "Subdimensões", "Reinos do Horizonte", "Tempestade de Avatares",
    ],
};

pub const ENIGMAS: AbilityDefinition = AbilityDefinition {
    id: "enigmas",
    name: "Enigmas",
    name_pt: "Enigmas",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 291-292",
    description: "The capacity for solving brain-teasers, decoding cryptograms, untangling paradoxes, answering riddles of the Sphinx, and unravelling mystic koans.",
    description_pt: "A capacidade de desvendar quebra-cabeças, decodificar criptogramas, desatar paradoxos, responder a charadas da Esfinge e solucionar koans místicos.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Solves daily newspaper crosswords and simple logic puzzles.",
            description_pt: "Resolve palavras cruzadas de jornal e passatempos de lógica simples.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Untangles ciphers and spots hidden patterns in riddles.",
            description_pt: "Decifra códigos elementares e percebe pegadinhas em charadas.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Can solve ancient labyrinth puzzles and complex mathematical ciphers.",
            description_pt: "Soluciona enigmas de labirintos antigos e cifras matemáticas complexas.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Zen master or cryptanalyst unraveling multidimensional mysteries.",
            description_pt: "Mestre zen ou criptoanalista capaz de desvendar mistérios multidimensionais.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "The riddles of creation and existence surrender their answers to you.",
            description_pt: "As maiores charadas da existência revelam seus segredos à sua mente.",
        },
    ],
    possessed_by: "Cryptographers, Hermetics, Akashic Monks, Detectives, Mystics, Chess Masters.",
    possessed_by_pt: "Criptógrafos, Herméticos, Monges Akashicos, Detetives, Místicos, Enxadristas.",
    suggested_specialties: &[
        "Riddles", "Ciphers", "Zen Koans", "Mystic Conundrums", "Mathematical Logic", "Symbolism",
    ],
    suggested_specialties_pt: &[
        "Charadas", "Cifras", "Koans Zen", "Dilemas Místicos", "Lógica Matemática", "Simbolismo",
    ],
};

pub const ESOTERICA: AbilityDefinition = AbilityDefinition {
    id: "esoterica",
    name: "Esoterica",
    name_pt: "Esotérica",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 292-293",
    description: "Specialized, highly specific occult traditions: Alchemy, Astrology, Kabbalah, I Ching, Tarot, Herbalism, Feng Shui, Tantra, Goetia, and other ritual lore.",
    description_pt: "Tradições ocultas altamente especializadas e específicas: Alquimia, Astrologia, Cabala, I Ching, Tarot, Herbalismo, Feng Shui, Tantra, Goécia e saberes rituais.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Understands basic terminology and symbols of your chosen lore.",
            description_pt: "Conhece termos e símbolos básicos da tradição escolhida.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Can draw astrological charts or perform accurate Tarot readings.",
            description_pt: "Sabe traçar mapas astrais ou realizar tiragens coerentes de Tarot.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Practicing occultist skilled in esoteric formulas and correspondences.",
            description_pt: "Ocultista praticante com domínio de fórmulas e tabelas de correspondência.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Respected hierophant or alchemical adept who crafts intricate workings.",
            description_pt: "Hierofante respeitado ou adepto alquímico que elabora ritos intrincados.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Master magus whose esoteric understanding mirrors the cosmos.",
            description_pt: "Mago mestre cuja compreensão esotérica reflete a ordem cósmica.",
        },
    ],
    possessed_by: "Hermetics, Solificati, Astrologers, Kabbalists, Alchemists, Diviners.",
    possessed_by_pt: "Herméticos, Solificati, Astrólogos, Cabalistas, Alquimistas, Adivinhos.",
    suggested_specialties: &[
        "Alchemy", "Astrology", "Kabbalah", "Tarot", "Herbalism", "I Ching", "Goetia", "Feng Shui",
    ],
    suggested_specialties_pt: &[
        "Alquimia", "Astrologia", "Cabala", "Tarot", "Herbalismo", "I Ching", "Goécia", "Feng Shui",
    ],
};

pub const INVESTIGATION: AbilityDefinition = AbilityDefinition {
    id: "investigation",
    name: "Investigation",
    name_pt: "Investigação",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 293",
    description: "The science and instinct of finding clues, analyzing crime scenes, questioning witnesses, reconstructing events, and connecting disparate threads of a mystery.",
    description_pt: "A ciência e o instinto de encontrar pistas, examinar cenas de crime, interrogar testemunhas, reconstituir ocorrências e conectar fios soltos de um mistério.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Observant hobbyist who notices missing items or footprints.",
            description_pt: "Observador amador que nota pegadas ou gavetas reviradas.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Competent patrol officer or insurance claims investigator.",
            description_pt: "Policial atento ou perito de sinistros de seguradora.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Seasoned homicide detective who reconstructs violent struggles.",
            description_pt: "Detetive de homicídios calejado que recria cenas de crime com exatidão.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Master forensic criminalist whose deductions break stone-cold cases.",
            description_pt: "Perito criminal renomado cujas deduções resolvem casos arquivados.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Sherlock Holmes reborn; a speck of ash tells you the culprit's life history.",
            description_pt: "Sherlock Holmes encarnado; uma partícula de cinza revela todo o ocorrido.",
        },
    ],
    possessed_by: "Detectives, Cops, NWO Investigators, Spies, Journalists, Private Eyes.",
    possessed_by_pt: "Detetives, Policiais, Investigadores da NOV, Espiões, Jornalistas, Peritos.",
    suggested_specialties: &[
        "Crime Scenes", "Forensics", "Interrogation", "Deduction", "Search Patterns", "Hidden Objects",
    ],
    suggested_specialties_pt: &[
        "Cenas de Crime", "Perícia Forense", "Interrogatório", "Dedução", "Padrões de Busca", "Objetos Ocultos",
    ],
};

pub const LAW: AbilityDefinition = AbilityDefinition {
    id: "law",
    name: "Law",
    name_pt: "Direito",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 293-294",
    description: "Understanding legal codes, judicial procedures, courtroom dynamics, corporate regulations, police procedures, precedents, and the loopholes that save defendants.",
    description_pt: "Compreensão da legislação, processos judiciais, dinâmica de tribunais, regulamentos corporativos, direitos civis, jurisprudência e brechas na lei.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Knows basic rights, speeding ticket laws, and courtroom etiquette.",
            description_pt: "Conhece direitos civis básicos e como portar-se perante um juiz.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Paralegal or law student who can prepare briefs and legal forms.",
            description_pt: "Estagiário de direito ou paralegal que redige petições simples.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Practicing defense attorney or prosecutor who wins cases in court.",
            description_pt: "Advogado atuante ou promotor que vence julgamentos no tribunal.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Senior partner or judge with encyclopedic mastery of statutory codes.",
            description_pt: "Sócio sênior ou juiz experiente com domínio enciclopédico de códigos.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Supreme Court justice or legal architect drafting national constitutions.",
            description_pt: "Ministro de suprema corte ou jurista que redige cartas constitucionais.",
        },
    ],
    possessed_by: "Lawyers, Judges, Syndicate Financiers, Politicians, Corporate Executives, Cops.",
    possessed_by_pt: "Advogados, Juízes, Financiadores do Sindicato, Políticos, Executivos.",
    suggested_specialties: &[
        "Criminal Law", "Corporate", "Civil Rights", "Contracts", "International Law", "Loopholes",
    ],
    suggested_specialties_pt: &[
        "Direito Penal", "Corporativo", "Direitos Civis", "Contratos", "Direito Internacional", "Brechas Legais",
    ],
};

pub const MEDICINE: AbilityDefinition = AbilityDefinition {
    id: "medicine",
    name: "Medicine",
    name_pt: "Medicina",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 294",
    description: "Human anatomy, diagnosis, pharmacology, pathology, emergency triage, surgical procedures, and medical technology that preserves life or treats injuries.",
    description_pt: "Anatomia humana, diagnóstico clínico, farmacologia, patologia, primeiros socorros, cirurgia e tecnologia médica aplicada à cura e sobrevivência física.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Certified in CPR and basic first aid; can bandage wounds.",
            description_pt: "Certificado em primeiros socorros e RCP; sabe estancar sangramentos.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Paramedic or nurse who stabilizes critical trauma in ambulances.",
            description_pt: "Paramédico ou enfermeiro que estabiliza vítimas graves de trauma.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "General practitioner physician; diagnoses illnesses and prescribes cures.",
            description_pt: "Médico clínico geral; diagnostica patologias e prescreve tratamentos.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Specialist trauma surgeon performing high-stakes emergency operations.",
            description_pt: "Cirurgião especialista em trauma que opera procedimentos delicados.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Pioneering medical authority performing miraculous medical interventions.",
            description_pt: "Autoridade médica revolucionária que salva vidas nos casos mais desenganados.",
        },
    ],
    possessed_by: "Doctors, Nurses, Paramedics, Progenitors, Verbena Healers, Combat Medics.",
    possessed_by_pt: "Médicos, Enfermeiros, Paramédicos, Progenitores, Curandeiros Verbena.",
    suggested_specialties: &[
        "Emergency Triage", "Surgery", "Pharmacology", "Pathology", "Holistic Healing", "Trauma",
    ],
    suggested_specialties_pt: &[
        "Triagem de Emergência", "Cirurgia", "Farmacologia", "Patologia", "Cura Holística", "Trauma",
    ],
};

pub const OCCULT: AbilityDefinition = AbilityDefinition {
    id: "occult",
    name: "Occult",
    name_pt: "Ocultismo",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 294-295",
    description: "Lore concerning supernatural creatures and folklore: vampires, werewolves, ghosts, fae, secret societies, urban legends, and mystical rumors throughout history.",
    description_pt: "Saberes sobre criaturas da noite e folclore sobrenatural: vampiros, lobisomens, espectros, fadas, sociedades secretas, lendas urbanas e conspirações místicas.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Familiar with pop-culture myths and campfire ghost tales.",
            description_pt: "Conhece mitos populares, lendas urbanas e contos de terror.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Knows that werewolves hate silver and vampires shun sunlight.",
            description_pt: "Sabe que lobisomens temem prata e vampiros perecem ao sol.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Recognizes vampire clan marks, werewolf tribes, and ghostly banes.",
            description_pt: "Reconhece clãs vampíricos, tribos de lobisomens e fraquezas de aparições.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Expert on the secret politics and weaknesses of World of Darkness creatures.",
            description_pt: "Especialista nas intrigas secretas e fraquezas dos monstros do Mundo das Trevas.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Ancient lore master who knows secrets forgotten even by elders.",
            description_pt: "Mestre dos saberes arcanos que conhece segredos esquecidos até por anciões.",
        },
    ],
    possessed_by: "Occultists, Hermetics, Witches, Paranormal Hunters, Scholars of the Night.",
    possessed_by_pt: "Ocultistas, Herméticos, Bruxas, Caçadores Noturnos, Eruditos do Sobrenatural.",
    suggested_specialties: &[
        "Vampires", "Werewolves", "Ghosts", "Fae", "Secret Societies", "Banes and Wards", "Demonology",
    ],
    suggested_specialties_pt: &[
        "Vampiros", "Lobisomens", "Fantasmas", "Fadas", "Sociedades Secretas", "Maldições e Proteções", "Demonologia",
    ],
};

pub const POLITICS: AbilityDefinition = AbilityDefinition {
    id: "politics",
    name: "Politics",
    name_pt: "Política",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 295",
    description: "Understanding government structures, bureaucratic machinations, lobbying, political alliances, elections, ideological currents, and power dynamics.",
    description_pt: "Compreensão de estruturas governamentais, burocracia estatal, lobby, alianças partidárias, eleições, correntes ideológicas e engrenagens de poder.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Votes regularly and follows local political debates.",
            description_pt: "Vota com regularidade e acompanha debates eleitorais nos jornais.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Knows local city hall mechanisms and key council members.",
            description_pt: "Conhece o funcionamento da câmara de vereadores e secretarias municipais.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Political campaign manager or seasoned civil servant.",
            description_pt: "Coordenador de campanha eleitoral ou assessor parlamentar experiente.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "High-ranking political strategist pulling strings in state capitals.",
            description_pt: "Estrategista político de topo que articula votações e emendas cruciais.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Machiavellian kingmaker shaping geopolitical alliances across nations.",
            description_pt: "Eminência parda que molda alianças geopolíticas internacionais.",
        },
    ],
    possessed_by: "Politicians, NWO Operatives, Syndicate Lobbyists, Activists, Diplomats.",
    possessed_by_pt: "Políticos, Operativos da NOV, Lobistas do Sindicato, Ativistas, Diplomatas.",
    suggested_specialties: &[
        "Lobbying", "Local Government", "International Relations", "Scandals", "Elections", "Bureaucracy",
    ],
    suggested_specialties_pt: &[
        "Lobby", "Governo Local", "Relações Internacionais", "Escândalos", "Eleições", "Burocracia",
    ],
};

pub const SCIENCE: AbilityDefinition = AbilityDefinition {
    id: "science",
    name: "Science",
    name_pt: "Ciência",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 295",
    description: "The systematic pursuit of empirical knowledge: physics, chemistry, biology, mathematics, astronomy, quantum theory, engineering, and scientific methodology.",
    description_pt: "A busca sistemática por conhecimento empírico: física, química, biologia, matemática, astronomia, mecânica quântica, engenharia e método científico.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Understands the scientific method and high school physics/chemistry.",
            description_pt: "Entende o método científico e noções básicas de física e química.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "College science major conducting laboratory experiments.",
            description_pt: "Estudante universitário de ciências exatas conduzindo testes laboratoriais.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Research scientist with published peer-reviewed papers.",
            description_pt: "Pesquisador com artigos publicados em periódicos científicos.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Senior laboratory director developing innovative technical solutions.",
            description_pt: "Líder de laboratório desenvolvendo soluções tecnológicas de vanguarda.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Nobel laureate whose theoretical breakthroughs redefine physical laws.",
            description_pt: "Laureado com o Nobel cujas teorias revolucionam as leis da física.",
        },
    ],
    possessed_by: "Sons of Ether, Progenitors, Iteration X, Void Engineers, Researchers, Physicists.",
    possessed_by_pt: "Filhos do Éter, Progenitores, Iteração X, Engenheiros do Vazio, Cientistas.",
    suggested_specialties: &[
        "Quantum Physics", "Chemistry", "Biology", "Astronomy", "Mathematics", "Hyper-Mathematics", "Engineering",
    ],
    suggested_specialties_pt: &[
        "Física Quântica", "Química", "Biologia", "Astronomia", "Matemática", "Hipermatemática", "Engenharia",
    ],
};

pub const CORE_KNOWLEDGES: &[AbilityDefinition] = &[
    ACADEMICS,
    COMPUTER,
    COSMOLOGY,
    ENIGMAS,
    ESOTERICA,
    INVESTIGATION,
    LAW,
    MEDICINE,
    OCCULT,
    POLITICS,
    SCIENCE,
];
