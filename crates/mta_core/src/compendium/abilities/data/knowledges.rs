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
    page_ref: "M20, p. 283",
    description: "Scholastic education and humanities: history, literature, philosophy, languages, sociology, and classical liberal arts that expand a character's intellectual horizon.",
    description_pt: "Educação formal e humanidades: história, literatura, filosofia, línguas clássicas, sociologia e erudição que expandem os horizontes intelectuais do personagem.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Basic primary and secondary education.",
            description_pt: "Educação primária e secundária básica.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Basic university education, equivalent to a two-year degree.",
            description_pt: "Educação universitária básica, equivalente a um curso de dois anos.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Advanced university education, equivalent to a four-year degree.",
            description_pt: "Educação universitária avançada, equivalente a uma graduação de quatro anos.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Advanced specialized study, plus an advanced university-level education.",
            description_pt: "Estudo especializado avançado, além de formação em nível universitário avançado.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Refined and advanced scholarship, with highly respected academic achievements.",
            description_pt: "Erudição refinada e avançada, com realizações acadêmicas altamente respeitadas.",
        },
    ],
    possessed_by: "Students, Scholars, Scientists, Journalists, Authors, Wizards.",
    possessed_by_pt: "Estudantes, Eruditos, Cientistas, Jornalistas, Autores, Feiticeiros.",
    suggested_specialties: &[
        "Teaching", "Liberal Arts", "specific field of study (History, Language, Folklore, Politics, etc.)",
    ],
    suggested_specialties_pt: &[
        "Ensino", "Artes Liberais", "campo de estudo específico (História, Idioma, Folclore, Política, etc.)",
    ],
};

pub const COMPUTER: AbilityDefinition = AbilityDefinition {
    id: "computer",
    name: "Computer",
    name_pt: "Computador",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 283-284",
    description: "Expertise in IT, coding, operating systems, networking, digital security, programming languages, cryptography, and navigating the Digital Web.",
    description_pt: "Domínio de informática, desenvolvimento de software, arquitetura de redes, segurança digital, criptografia, sistemas operacionais e navegação na Rede Digital.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "You know basic computer skills for your time and place.",
            description_pt: "Você conhece habilidades básicas de informática para sua época e lugar.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "You’ve studied IT systems and can do basic programming.",
            description_pt: "Você estudou sistemas de TI e consegue fazer programação básica.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Study and experience make you pretty decent with essential systems.",
            description_pt: "Estudo e experiência tornam você muito bom com sistemas essenciais.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "You’re a respected pro in the computer tech field.",
            description_pt: "Você é um profissional respeitado na área de tecnologia da computação.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "When you’re not making existing systems play fetch for you, you’re crafting new stuff, reworking hardware, and being an overall master of your trade.",
            description_pt: "Quando não está fazendo sistemas existentes obedecerem a você como cãezinhos, você cria coisas novas, refaz hardware e é um mestre absoluto em seu ofício.",
        },
    ],
    possessed_by: "Virtual Adepts, Technocrats, Technomancers, Students, IT Pros, Hackers, Military Personnel, Almost Anyone in Current Tech Cultures.",
    possessed_by_pt: "Adeptos da Virtualidade, Tecnocratas, Tecnomantes, Estudantes, Profissionais de TI, Hackers, Militares, Praticamente Qualquer Pessoa nas Culturas Tecnológicas Atuais.",
    suggested_specialties: &[
        "General Use", "Internet", "Programming", "Repair", "IT Culture", "Office Tech", "Entertainment Tech", "CGI", "Data Systems", "Hacking", "Cracking", "Organization", "Current Market", "Underground", "Advanced Tech", "Cryptanalysis",
    ],
    suggested_specialties_pt: &[
        "Uso Geral", "Internet", "Programação", "Reparo", "Cultura de TI", "Tecnologia de Escritório", "Tecnologia de Entretenimento", "CGI", "Sistemas de Dados", "Hacking", "Cracking", "Organização", "Mercado Atual", "Subsolo/Underground", "Tecnologia Avançada", "Criptoanálise",
    ],
};

pub const COSMOLOGY: AbilityDefinition = AbilityDefinition {
    id: "cosmology",
    name: "Cosmology / Subdimensions",
    name_pt: "Cosmologia / Subdimensões",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 284",
    description: "Understanding the structure of the Otherworlds: the Penumbra, Near Umbra, Horizon, Deep Umbra, Astral Planes, and Technocratic subdimensional manifolds.",
    description_pt: "Compreensão da estrutura dos Outros Mundos: a Penumbra, Umbra Próxima, Horizonte, Umbra Profunda, Planos Astrais e os coletores subdimensionais da Tecnocracia.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Student of the mysteries.",
            description_pt: "Estudante dos mistérios.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Occasional voyager.",
            description_pt: "Viajante ocasional.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Experienced traveler.",
            description_pt: "Viajante experiente.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Seasoned explorer.",
            description_pt: "Explorador calejado.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "World-walker.",
            description_pt: "Caminhante dos mundos.",
        },
    ],
    possessed_by: "Shamans, Witches, Hermetics, Void Engineers, Etherites, Werewolves, Astral Travelers, Archmages.",
    possessed_by_pt: "Xamãs, Bruxas, Herméticos, Engenheiros do Vazio, Eteritas, Lobisomens, Viajantes Astrais, Arquimagos.",
    suggested_specialties: &[
        "Astral Voyaging", "Courts", "Paths", "Hazards", "Omens", "Realms", "Navigation", "Dimensional Exploration", "Threat Factors", "Otherworldly Etiquette",
    ],
    suggested_specialties_pt: &[
        "Viagem Astral", "Cortes", "Caminhos", "Perigos", "Presságios", "Reinos", "Navegação", "Exploração Dimensional", "Fatores de Ameaça", "Etiqueta Transmundana",
    ],
};

pub const ENIGMAS: AbilityDefinition = AbilityDefinition {
    id: "enigmas",
    name: "Enigmas",
    name_pt: "Enigmas",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 284",
    description: "The capacity for solving brain-teasers, decoding cryptograms, untangling paradoxes, answering riddles of the Sphinx, and unravelling mystic koans.",
    description_pt: "A capacidade de desvendar quebra-cabeças, decodificar criptogramas, desatar paradoxos, responder a charadas da Esfinge e solucionar koans místicos.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Puzzles intrigue you.",
            description_pt: "Quebra-cabeças intrigam você.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Zen koans do not piss you off.",
            description_pt: "Koans zen não tiram você do sério.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "“You were not put here to ‘Get it,’ Mr. Burton.”",
            description_pt: "“Você não foi colocado aqui para ‘entender’, Sr. Burton.”",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "You clap with one hand inside Schrödinger’s box.",
            description_pt: "Você bate palmas com uma mão só dentro da caixa de Schrödinger.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "You are an enigma.",
            description_pt: "Você é um enigma.",
        },
    ],
    possessed_by: "Akashics, Ecstatics, Hermetics, Shamans, Mad Scientists, Detectives, Weird-Ass Metaphysicians, Gamers, Sifus, Professors, Crazy Fuckers, Annoying Old Masters.",
    possessed_by_pt: "Akáshicos, Extáticos, Herméticos, Xamãs, Cientistas Loucos, Detetives, Metafísicos Excêntricos, Jogadores, Mestres (Sifus), Professores, Malucos Totais, Velhos Mestres Irritantes.",
    suggested_specialties: &[
        "Riddles", "Puzzles", "Metaphysics", "Deduction", "Zen", "Arcane Enigmas", "Codes and Ciphers", "Classic Mysteries", "Intuitive Leaps", "Deciphering Madness",
    ],
    suggested_specialties_pt: &[
        "Charadas", "Quebra-Cabeças", "Metafísica", "Dedução", "Zen", "Enigmas Arcanos", "Códigos e Cifras", "Mistérios Clássicos", "Saltos Intuitivos", "Decifrar a Loucura",
    ],
};

pub const ESOTERICA: AbilityDefinition = AbilityDefinition {
    id: "esoterica",
    name: "Esoterica",
    name_pt: "Esotérica",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 284-286",
    description: "Specialized, highly specific occult traditions: Alchemy, Astrology, Kabbalah, I Ching, Tarot, Herbalism, Feng Shui, Tantra, Goetia, and other ritual lore.",
    description_pt: "Tradições ocultas altamente especializadas e específicas: Alquimia, Astrologia, Cabala, I Ching, Tarot, Herbalismo, Feng Shui, Tantra, Goécia e saberes rituais.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "You’ve skimmed some New Age books or taken a handful of classes in the subject.",
            description_pt: "Você folheou alguns livros da Nova Era ou fez algumas aulas sobre o assunto.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "You’ve devoted some time and energy to esoteric subjects.",
            description_pt: "Você dedicou algum tempo e energia a temas esotéricos.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "A devoted esotericist, you’ve spent years building up a working knowledge of odd subjects.",
            description_pt: "Um esoterista dedicado, você passou anos construindo conhecimento prático sobre assuntos insólitos.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Now you understand the interconnected patterns of arcane disciplines; knowing one allows you to master others more easily.",
            description_pt: "Agora você compreende os padrões interconectados das disciplinas arcanas; conhecer uma permite que você domine outras com maior facilidade.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "You possess a profound understanding of esoteric subjects and the practical uses behind them.",
            description_pt: "Você possui uma compreensão profunda de assuntos esotéricos e dos usos práticos por trás deles.",
        },
    ],
    possessed_by: "Monks, Professors, Clergy, New Agers, Witches, Weirdoes, Artists, Occultists, Teachers, Transhumanists, Paranormal Researchers, Monster Hunters, Fantasy Authors and Editors, Students and Masters of Arcane Arts.",
    possessed_by_pt: "Monges, Professores, Clérigos, Adeptos da Nova Era, Bruxas, Tipos Excêntricos, Artistas, Ocultistas, Educadores, Transumanistas, Pesquisadores Paranormais, Caçadores de Monstros, Autores e Editores de Fantasia, Estudantes e Mestres de Artes Arcanas.",
    suggested_specialties: &[
        "Yoga", "Tantra", "Herbalism", "Kabbalah", "Fortune-Telling", "Hypnosis", "Astrology", "Celestiography", "Demonology", "Sacred Geometry", "Gematria", "Goetia", "Prophecies", "Omens", "T’ai Chi", "I Ching", "Stone Lore", "Alchemy", "Symbolism", "Transhumanist Theory", "Esoteric Musicology", "Bakemono-Jutsu", "Iconology", "Numerology", "Voodoo", "Crystalmancy", "Tarot", "specific arcane languages (Enochian, In-o-Musubi, the Language of Flowers, etc.)",
    ],
    suggested_specialties_pt: &[
        "Ioga", "Tantra", "Herbalismo", "Cabala", "Adivinhação", "Hipnose", "Astrologia", "Celestiografia", "Demonologia", "Geometria Sagrada", "Gematria", "Goécia", "Profecias", "Presságios", "Tai Chi", "I Ching", "Litologia/Sabedoria das Pedras", "Alquimia", "Simbolismo", "Teoria Transumanista", "Musicologia Esotérica", "Bakemono-Jutsu", "Iconologia", "Numerologia", "Vodu", "Cristalomancia", "Tarot", "idiomas arcanos específicos (Enoquiano, In-o-Musubi, Linguagem das Flores, etc.)",
    ],
};

pub const INVESTIGATION: AbilityDefinition = AbilityDefinition {
    id: "investigation",
    name: "Investigation",
    name_pt: "Investigação",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 286",
    description: "The science and instinct of finding clues, analyzing crime scenes, questioning witnesses, reconstructing events, and connecting disparate threads of a mystery.",
    description_pt: "A ciência e o instinto de encontrar pistas, examinar cenas de crime, interrogar testemunhas, reconstituir ocorrências e conectar fios soltos de um mistério.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "You spend lots of time in the Mystery section of your favorite bookstore.",
            description_pt: "Você passa muito tempo na seção de Mistério da sua livraria favorita.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "You’ve learned basic criminology.",
            description_pt: "Você aprendeu criminologia básica.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Detective work is your trade and passion.",
            description_pt: "Trabalho investigativo é o seu ofício e paixão.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Clarice Starling comes to you for advice.",
            description_pt: "Clarice Starling vem pedir conselhos a você.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Batman.",
            description_pt: "Batman.",
        },
    ],
    possessed_by: "Crime Writers, Cops, Detectives, Stalkers, Journalists, Intelligence Agents, Smart Criminals, Black Suits, Refined Cannibal Geniuses With a Taste For Chianti.",
    possessed_by_pt: "Escritores de Policiais, Policiais, Detetives, Perseguidores, Jornalistas, Agentes de Inteligência, Criminosos Inteligentes, Ternos Pretos, Gênios Canibais Refinados com Gosto por Chianti.",
    suggested_specialties: &[
        "Forensics", "Deduction", "Interrogation", "Crime Scenes", "Data Sorting", "Criminal Psychology", "Spotting Clues", "Personal Details", "Paranormal Investigations",
    ],
    suggested_specialties_pt: &[
        "Perícia Forense", "Dedução", "Interrogatório", "Cenas de Crime", "Triagem de Dados", "Psicologia Criminal", "Identificar Pistas", "Detalhes Pessoais", "Investigações Paranormais",
    ],
};

pub const LAW: AbilityDefinition = AbilityDefinition {
    id: "law",
    name: "Law",
    name_pt: "Direito",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 286",
    description: "Understanding legal codes, judicial procedures, courtroom dynamics, corporate regulations, police procedures, precedents, and the loopholes that save defendants.",
    description_pt: "Compreensão da legislação, processos judiciais, dinâmica de tribunais, regulamentos corporativos, procedimentos policiais, jurisprudência e brechas que salvam os réus.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Beat cop.",
            description_pt: "Policial de ronda.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Police sergeant.",
            description_pt: "Sargento da polícia.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Chief or DA.",
            description_pt: "Chefe de polícia ou promotor distrital.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Federal bureau official.",
            description_pt: "Oficial de agência federal.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Attorney General.",
            description_pt: "Procurador-Geral.",
        },
    ],
    possessed_by: "Cops, Lawyers, Government Agents, Criminals, Gang Bosses, Reporters.",
    possessed_by_pt: "Policiais, Advogados, Agentes do Governo, Criminosos, Chefes de Gangue, Repórteres.",
    suggested_specialties: &[
        "Local Beat", "Crimes", "Customs", "Procedures", "Bribery", "Internal Affairs", "Courts", "International Agencies", "Occult Crimes", "Jurisdiction", "Chain of Command", "Bending the Rules",
    ],
    suggested_specialties_pt: &[
        "Ronda Local", "Crimes", "Costumes", "Procedimentos", "Suborno", "Corregedoria", "Tribunais", "Agências Internacionais", "Crimes Ocultos", "Jurisdição", "Cadeia de Comando", "Burlar as Regras",
    ],
};

pub const MEDICINE: AbilityDefinition = AbilityDefinition {
    id: "medicine",
    name: "Medicine",
    name_pt: "Medicina",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 286",
    description: "Human anatomy, diagnosis, pharmacology, pathology, emergency triage, surgical procedures, and medical technology that preserves life or treats injuries.",
    description_pt: "Anatomia humana, diagnóstico clínico, farmacologia, patologia, triagem de emergência, procedimentos cirúrgicos e tecnologia médica aplicada à preservação da vida ou tratamento de lesões.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "You’ve learned basic First Aid and CPR.",
            description_pt: "Você aprendeu Primeiros Socorros básicos e RCP.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "You can treat minor traumas and illnesses.",
            description_pt: "Você consegue tratar pequenos traumas e doenças.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "Medical training allows you to pursue more serious treatments.",
            description_pt: "A formação médica permite a você realizar tratamentos mais sérios.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "You’ve got a professional degree of skill in advanced medicine and surgical procedures.",
            description_pt: "Você tem um nível profissional de habilidade em medicina avançada e procedimentos cirúrgicos.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "A noted and respected healer, you’re in demand for your extensive expertise.",
            description_pt: "Um curandeiro notável e respeitado, você é requisitado por sua vasta experiência.",
        },
    ],
    possessed_by: "Healers, Paramedics, Medical Professionals, Survivalists, Midwives, Torturers, Witches, Euthanatos, Progenitors.",
    possessed_by_pt: "Curandeiros, Paramédicos, Profissionais Médicos, Sobrevivencialistas, Parteiras, Torturadores, Bruxas, Eutanatos, Progenitores.",
    suggested_specialties: &[
        "First Aid", "Surgery", "Nursing", "Bare-Bones Facilities", "Improvised Techniques", "Traditional Remedies", "Alternative Medicine", "Physical Deconstruction", "Magickal Healing", "Frankensteinian Techniques", "“Live, Damn You — LIVE!”",
    ],
    suggested_specialties_pt: &[
        "Primeiros Socorros", "Cirurgia", "Enfermagem", "Instalações Precárias", "Técnicas Improvisadas", "Remédios Tradicionais", "Medicina Alternativa", "Desconstrução Física", "Cura Mágica", "Técnicas Frankensteinianas", "“Viva, Droga — VIVA!”",
    ],
};

pub const OCCULT: AbilityDefinition = AbilityDefinition {
    id: "occult",
    name: "Occult",
    name_pt: "Ocultismo",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 286-287",
    description: "Lore concerning supernatural creatures and folklore: vampires, werewolves, ghosts, fae, secret societies, urban legends, and mystical rumors throughout history.",
    description_pt: "Saberes sobre criaturas sobrenaturais e folclore: vampiros, lobisomens, fantasmas, fadas, sociedades secretas, lendas urbanas e boatos místicos ao longo da história.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "You think you know who Crowley was, and you have some New Age books on your shelves.",
            description_pt: "Você acha que sabe quem foi Crowley e tem alguns livros da Nova Era nas estantes.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Some of the stuff you’ve heard or read is actually true.",
            description_pt: "Algumas das coisas que você ouviu ou leu são realmente verdadeiras.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "You’ve got first-hand experience in the world of shadows.",
            description_pt: "Você tem experiência direta no mundo das sombras.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Study and experience have taught you a respectable amount of genuine lore.",
            description_pt: "Estudo e experiência lhe ensinaram uma quantidade respeitável de conhecimento genuíno.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Knowledgeable parties respect your accomplishments and consider you an authority in your own right.",
            description_pt: "Partes bem-informadas respeitam suas realizações e o consideram uma autoridade por mérito próprio.",
        },
    ],
    possessed_by: "Moral Crusaders, Disaffected Teens, Metalheads, Goths, Pagans, New Agers, Parapsychologists, Fantasy Authors, RPG Designers, Students of Unhallowed Arts, Folks Who Spend Too Much Time on the Internet.",
    possessed_by_pt: "Cruzados Morais, Adolescentes Rebeldes, Metaleiros, Góticos, Pagãos, Adeptos da Nova Era, Parapsicólogos, Autores de Fantasia, Designers de RPG, Estudantes de Artes Profanas, Pessoas que Passam Tempo Demais na Internet.",
    suggested_specialties: &[
        "Neopaganism", "Occult History", "Conspiracy Theories", "Secret Societies", "New Age", "Alternative Sciences", "Mystic Lore", "Folk Magic", "Moral Panic", "Urban Legends", "Satanic Folklore", "Pop-Culture Satanism", "Actual Satanism", "any specific occult discipline or field (Freemasonry, Voodoo, Stage Magic, etc.)",
    ],
    suggested_specialties_pt: &[
        "Neopaganismo", "História Oculta", "Teorias da Conspiração", "Sociedades Secretas", "Nova Era", "Ciências Alternativas", "Sabedoria Mística", "Magia Popular", "Pânico Moral", "Lendas Urbanas", "Folclore Satânico", "Satanismo da Cultura Pop", "Satanismo Real", "qualquer disciplina ou campo oculto específico (Maçonaria, Vodu, Ilusionismo de Palco, etc.)",
    ],
};

pub const POLITICS: AbilityDefinition = AbilityDefinition {
    id: "politics",
    name: "Politics",
    name_pt: "Política",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 287-288",
    description: "Understanding government structures, bureaucratic machinations, lobbying, political alliances, elections, ideological currents, and power dynamics.",
    description_pt: "Compreensão de estruturas governamentais, maquinações burocráticas, lobby, alianças políticas, eleições, correntes ideológicas e dinâmicas de poder.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "Amateur power-broker.",
            description_pt: "Negociador amador de poder.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "Devotee of influence.",
            description_pt: "Devoto da influência.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "A skillful manipulator of civil institutions.",
            description_pt: "Um manipulador habilidoso de instituições civis.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Magnificent bastard.",
            description_pt: "Um canalha magnífico.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Machiavelli’s ghost whispers in your ear.",
            description_pt: "O fantasma de Maquiavel sussurra no seu ouvido.",
        },
    ],
    possessed_by: "Politicians, Activists, Pundits, Lawyers, Journalists, Kingpins, Tycoons, Celebrities, Syndicate Ops, NWO Agents, Cult Gurus, Leaders of All Kinds.",
    possessed_by_pt: "Políticos, Ativistas, Comentaristas, Advogados, Jornalistas, Chefões do Crime, Magnatas, Celebridades, Agentes do Sindicato, Agentes da NOV, Gurus de Culto, Líderes de Todos os Tipos.",
    suggested_specialties: &[
        "Legal Codes", "Law Enforcement", "Gang Culture", "Local Government", "National Government", "International Politics", "Corporate Influence", "Media Influence", "Blogosphere", "Loopholes", "Dog Whistles", "Radical Politics", "Celebrity Media", "Organized Crime", "Power-Brokering", "Specific Groups (U.S. Congress, News Corporation, Anonymous, the Mafia, etc.)", "Specific Cultures (Militia Movements, Neo-Communists, Conspiracy-Theorists, Jihadis, Anarchists)", "Specific Supernatural Groups (Traditions, Technocracy, Faerie Courts, Vampire Courts, etc.)",
    ],
    suggested_specialties_pt: &[
        "Códigos Legais", "Aplicação da Lei", "Cultura de Gangues", "Governo Local", "Governo Nacional", "Política Internacional", "Influência Corporativa", "Influência na Mídia", "Blogosfera", "Brechas Legais", "Apitos de Cachorro", "Política Radical", "Mídia de Celebridades", "Crime Organizado", "Negociação de Poder", "Grupos Específicos (Congresso dos EUA, News Corporation, Anonymous, a Máfia, etc.)", "Culturas Específicas (Movimentos de Milícia, Neocomunistas, Teóricos da Conspiração, Jihadistas, Anarquistas)", "Grupos Sobrenaturais Específicos (Tradições, Tecnocracia, Cortes das Fadas, Cortes Vampíricas, etc.)",
    ],
};

pub const SCIENCE: AbilityDefinition = AbilityDefinition {
    id: "science",
    name: "Science",
    name_pt: "Ciência",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 288",
    description: "The systematic pursuit of empirical knowledge: physics, chemistry, biology, mathematics, astronomy, quantum theory, engineering, and scientific methodology.",
    description_pt: "A busca sistemática por conhecimento empírico: física, química, biologia, matemática, astronomia, teoria quântica, engenharia e metodologia científica.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Dabbler",
            title_pt: "Curioso",
            description: "You’ve got a typical high school science education and understand the basic principles of your field.",
            description_pt: "Você tem a educação científica típica de ensino médio e compreende os princípios básicos da sua área.",
        },
        AbilityRating {
            dots: 2,
            title: "Student",
            title_pt: "Estudante",
            description: "You possess an advanced science education and understand the major theories, principles, and concepts of your preferred field.",
            description_pt: "Você possui formação científica avançada e compreende as principais teorias, princípios e conceitos da sua área de preferência.",
        },
        AbilityRating {
            dots: 3,
            title: "Scholar",
            title_pt: "Erudito",
            description: "You’ve dedicated yourself to scientific inquiry and possess a detailed knowledge of your chosen discipline.",
            description_pt: "Você se dedicou à investigação científica e possui conhecimento detalhado da disciplina escolhida.",
        },
        AbilityRating {
            dots: 4,
            title: "Professor",
            title_pt: "Professor",
            description: "Impressive scholarship, plus devoted study and research, make you an expert in your field and a master of generalized science.",
            description_pt: "Erudição impressionante, somada a estudo e pesquisa dedicados, fazem de você um especialista na sua área e um mestre em ciência geral.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Very few peers can match your expertise, understanding, and respect within the field.",
            description_pt: "Pouquíssimos colegas conseguem igualar sua competência, compreensão e respeito dentro da área.",
        },
    ],
    possessed_by: "Teachers, Technicians, Would-Be Superheroes, Tech Geeks, Any Technocrat or Technomantic Mage.",
    possessed_by_pt: "Professores, Técnicos, Aspirantes a Super-heróis, Nerds de Tecnologia, Qualquer Tecnocrata ou Mago Tecnomântico.",
    suggested_specialties: &[
        "Aeronautics", "Astronomy", "Biology", "Biopsychology", "Chemistry", "Computer Science", "Cybernetics", "Electronics",
        "Engineering", "Forensic Pathology", "Genetics", "Geology", "Hypermathematics", "Mathematics", "Metallurgy",
        "Paraphysics", "Phylogeny", "Physics", "Psychodynamics", "Psychology", "Psychopharmacology", "Psychoprojection",
        "Sociobiology", "Xenobiology",
    ],
    suggested_specialties_pt: &[
        "Aeronáutica", "Astronomia", "Biologia", "Biopsicologia", "Química", "Ciência da Computação", "Cibernética", "Eletrônica",
        "Engenharia", "Patologia Forense", "Genética", "Geologia", "Hipermatemática", "Matemática", "Metalurgia",
        "Parafísica", "Filogenia", "Física", "Psicodinâmica", "Psicologia", "Psicofarmacologia", "Psicoprojeção",
        "Sociobiologia", "Xenobiologia",
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
