//! Talentos Canônicos de M20 (Core Talents).
//!
//! Capítulo 6: Criação do Personagem (pp. 276-281).

use super::super::models::{AbilityCategory, AbilityDefinition, AbilityRating, AbilityScope};

pub const ALERTNESS: AbilityDefinition = AbilityDefinition {
    id: "alertness",
    name: "Alertness",
    name_pt: "Prontidão",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 275",
    description: "You sense things coming long before anyone else does. A combination of keen instincts and sharp thinking, this Ability helps you size up situations and respond accordingly. Beast-folk, survivors, bright courtiers and battle-honed veterans have high Alertness ratings.",
    description_pt: "Você pressente as coisas se aproximando muito antes de qualquer outra pessoa. Uma fusão de instintos aguçados e raciocínio rápido, esta Habilidade ajuda a avaliar situações e reagir em conformidade. Transmorfos, sobreviventes, cortesãos brilhantes e veteranos de batalha possuem altos índices de Prontidão.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "“I’ve got a bad feeling about this…”",
            description_pt: "“Tenho um mau pressentimento sobre isso…”",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Your hackles rise at the first sign of trouble.",
            description_pt: "Seus pelos se arrepiam ao primeiro sinal de encrenca.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You catch the threat beneath a whispered voice.",
            description_pt: "Você percebe a ameaça por trás de uma voz sussurrada.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "You’ve already noted three attackers and mapped four avenues of escape.",
            description_pt: "Você já notou três agressores e traçou quatro rotas de fuga.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Those ninjas might as well be wearing pink neon clown suits.",
            description_pt: "Para você, aqueles ninjas poderiam muito bem estar usando fantasias de palhaço neon rosa.",
        },
    ],
    possessed_by: "Spies, Bodyguards, Feral Kids, Street Survivors, Shapeshifters, War Veterans.",
    possessed_by_pt: "Espiões, Guarda-costas, Crianças Ferais, Sobreviventes das Ruas, Transmorfos, Veteranos de Guerra.",
    suggested_specialties: &[
        "Ambushes", "Instincts", "Omens", "Urban Wastelands", "Combat Zones", "Covert Pursuit", "Warning Signs",
    ],
    suggested_specialties_pt: &[
        "Emboscadas", "Instintos", "Presságios", "Terrenos Baldios Urbanos", "Zonas de Combate", "Perseguição Oculta", "Sinais de Alerta",
    ],
};

pub const ART: AbilityDefinition = AbilityDefinition {
    id: "art",
    name: "Art",
    name_pt: "Arte",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 275-276",
    description: "Also known as Artistic Expression, this Talent reflects your ability with a visual or performing art: drawing, singing, acting, writing, painting, dance, sculpture, and so forth. Given such talents, you can produce works of startling power and strike a chord of Truth in your audience.",
    description_pt: "Também conhecido como Expressão Artística, este Talento reflete sua habilidade com uma arte visual ou performática: desenho, canto, atuação, escrita, pintura, dança, escultura e afins. Com tais dons, você produz obras de impressionante impacto e toca o acorde da Verdade no público.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You’re creative but unskilled.",
            description_pt: "Você é criativo, mas sem técnica refinada.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "You’ve put some time and work into your chosen medium.",
            description_pt: "Você dedicou tempo e trabalho ao meio artístico escolhido.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Professional art could be a viable career option.",
            description_pt: "A arte profissional seria uma carreira perfeitamente viável.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "You’re a working pro whose work displays fascinating appeal and skill.",
            description_pt: "Você é um profissional atuante cujo trabalho exibe apelo e destreza fascinantes.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "You’re an acknowledged master of your field.",
            description_pt: "Você é um mestre consagrado e aclamado em sua área.",
        },
    ],
    possessed_by: "Art Students, Fine Artists, Professional Artists, Talented Kids, Dedicated Hobbyists, Reclusive Geniuses, Meditative Ascetics, Mages Who Focus Magick Through Art.",
    possessed_by_pt: "Estudantes de Arte, Artistas Plásticos, Artistas Profissionais, Crianças Talentosas, Hobbistas Dedicados, Gênios Reclusos, Ascetas Meditativos, Magos que Focam Mágika através da Arte.",
    suggested_specialties: &[
        "Painting", "Video", "Singing", "Acting", "Dance", "Mixed Media", "CGI", "Model-Making", "Classical Art", "Avant-Garde", "Unrefined Intuition", "Mad Brilliance", "Uncanny Truths", "Primitive Techniques", "specific musical instruments (Guitar, Drums, Piano, etc.)",
    ],
    suggested_specialties_pt: &[
        "Pintura", "Vídeo", "Canto", "Atuação", "Dança", "Mídia Mista", "CGI", "Confecção de Modelos", "Arte Clássica", "Vanguarda", "Intuição Não Lapidada", "Brilhantismo Insano", "Verdades Perturbadoras", "Técnicas Primitivas", "instrumentos musicais específicos (Guitarra, Bateria, Piano, etc.)",
    ],
};

pub const ATHLETICS: AbilityDefinition = AbilityDefinition {
    id: "athletics",
    name: "Athletics",
    name_pt: "Esportes",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 276",
    description: "Fitness is your passion. While other folks sit on the sidelines, you’re running, climbing, swimming, biking, and developing physical prowess with intense practice. 'A sound mind in a sound body' is your motto.",
    description_pt: "O condicionamento físico é sua paixão. Enquanto outros assistem da arquibancada, você corre, escala, nada, pedala e desenvolve vigor com treino intenso. 'Mente sã em corpo são' é o seu lema.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You were pretty decent in PE class.",
            description_pt: "Você se saía muito bem nas aulas de educação física.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Regular exercise is essential to your life.",
            description_pt: "Exercícios regulares são essenciais na sua vida.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You are one fit sonofabitch!",
            description_pt: "Você tem um físico incrivelmente em forma!",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Gym-rats and pro athletes take notes from you.",
            description_pt: "Frequentadores assíduos de academia e atletas profissionais aprendem com você.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "You could qualify for the Olympics… and you might actually compete!",
            description_pt: "Você poderia se classificar para as Olimpíadas… e talvez realmente competir!",
        },
    ],
    possessed_by: "Jocks, Cyborgs, Fitness Gurus, Personal Trainers, Professional Dancers, Martial Artists, Underwear Models, TV Stars.",
    possessed_by_pt: "Esportistas, Ciborgues, Gurus Fitness, Treinadores Pessoais, Dançarinos Profissionais, Artistas Marciais, Modelos de Roupa Íntima, Estrelas de TV.",
    suggested_specialties: &[
        "Bodybuilding", "Cycling", "Climbing", "Equestrian", "Swimming", "Survival Training", "Extreme Endurance", "Flow Arts", "Parkour", "Skiing", "specific sports (Football, Basketball, Hockey, etc.)",
    ],
    suggested_specialties_pt: &[
        "Musculação", "Ciclismo", "Escalada", "Hipismo", "Natação", "Treinamento de Sobrevivência", "Resistência Extrema", "Artes Circenses (Flow Arts)", "Parkour", "Esqui", "esportes específicos (Futebol, Basquete, Hóquei, etc.)",
    ],
};

pub const AWARENESS: AbilityDefinition = AbilityDefinition {
    id: "awareness",
    name: "Awareness",
    name_pt: "Consciência",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 276-277",
    description: "You have uncanny perceptions attuned to supernatural energies, mystical resonance, invisible entities, and the subtle warping of reality that ordinary mortals miss entirely.",
    description_pt: "Você possui percepções insólitas sintonizadas com energias sobrenaturais, ressonância mágica, entidades invisíveis e as distorções sutis da realidade que os mortais comuns ignoram totalmente.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You possess a gift for spotting hidden truths.",
            description_pt: "Você possui o dom de notar verdades ocultas.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Magic exists, and you often sense its presence.",
            description_pt: "A mágica existe e você costuma sentir a presença dela.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You can see auras, and you understand what those colors mean.",
            description_pt: "Você consegue ver auras e compreende o que essas cores significam.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "The supernatural world reveals itself to you in disturbing detail.",
            description_pt: "O mundo sobrenatural se revela a você em detalhes perturbadores.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "You’re so attuned to the mystic realm that many folks consider you insane.",
            description_pt: "Você é tão sintonizado com o reino místico que muitos o consideram insano.",
        },
    ],
    possessed_by: "Canny Grannies, Wise Spies, Crazy Shamans, Demented Seers, Insightful Kids, Mad Scientists, Those Touched By God.",
    possessed_by_pt: "Avós Astutas, Espiões Sábios, Xamãs Loucos, Videntes Dementes, Crianças Perspicazes, Cientistas Loucos, Aqueles Tocados por Deus.",
    suggested_specialties: &[
        "Omens", "Auras", "Resonance", "Weird Feelings", "Mystic Instincts", "Hidden Magic", "Spiritual Vidare",
    ],
    suggested_specialties_pt: &[
        "Presságios", "Auras", "Ressonância", "Sensações Estranhas", "Instintos Místicos", "Mágica Oculta", "Vidare Espiritual",
    ],
};

pub const BRAWL: AbilityDefinition = AbilityDefinition {
    id: "brawl",
    name: "Brawl",
    name_pt: "Briga",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 277",
    description: "When words fail, hands and feet do the talking. Brawl represents raw, practical hand-to-hand combat: punching, kicking, wrestling, biting, gouging, and dirty infighting.",
    description_pt: "Quando as palavras falham, mãos e pés resolvem. Briga representa o combate desarmado cru e prático: socos, chutes, agarramentos, cabeçadas, mordidas e luta suja corpo a corpo.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You had a tough childhood.",
            description_pt: "Você teve uma infância difícil.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "You’re from The Bad Part of Town.",
            description_pt: "Você veio da Parte Perigosa da Cidade.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "When punches fly, you’re among the last folks standing.",
            description_pt: "Quando os socos voam, você é um dos últimos a permanecer de pé.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Extreme fighting champs shiver when you enter the ring.",
            description_pt: "Campeões de luta extrema tremem quando você entra no ringue.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "“See me hit you, you go down.”",
            description_pt: "“Se eu te acertar, você cai.”",
        },
    ],
    possessed_by: "Bikers, Brawlers, Bullies, Former Punching Bags, Bodyguards, Military Veterans, Scary Folks With Bad Tempers.",
    possessed_by_pt: "Motociclistas, Valentões, Brigões, Antigos Sacos de Pancada, Guarda-costas, Veteranos Militares, Tipos Assustadores com Pavio Curto.",
    suggested_specialties: &[
        "Barroom", "Boxing", "Beast-Form Fighting", "Dirty Fighting", "Disarming", "Peaceful Warrior",
    ],
    suggested_specialties_pt: &[
        "Briga de Bar", "Boxe", "Combate em Forma Bestial", "Luta Suja", "Desarme", "Guerreiro Pacífico",
    ],
};

pub const EMPATHY: AbilityDefinition = AbilityDefinition {
    id: "empathy",
    name: "Empathy",
    name_pt: "Empatia",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 277",
    description: "You understand how people feel, why they feel that way, and what drives their emotions. Empathy allows you to read moods, detect deception, offer solace, and build deep connections.",
    description_pt: "Você compreende o que os outros sentem, as motivações por trás de suas emoções e seus anseios mais profundos. A Empatia permite ler humores, detectar mentiras, consolar e criar laços sinceros.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You’re pretty decent at noticing emotional cues.",
            description_pt: "Você é muito bom em notar pistas emocionais.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "You’re the crying shoulder for your friends.",
            description_pt: "Você é o ombro amigo onde seus amigos vão chorar.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "It’s fairly hard to deceive you.",
            description_pt: "É bem difícil enganar você.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Even below the surface, a person’s true feelings seem obvious to you.",
            description_pt: "Mesmo sob a superfície, os verdadeiros sentimentos de alguém parecem óbvios para você.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "To your eyes, most people are open books.",
            description_pt: "Aos seus olhos, a maioria das pessoas é um livro aberto.",
        },
    ],
    possessed_by: "Ecstatic Mages, Shamans, Healers, Seducers, Sales Reps, Therapists, Trusted Friends, Nephandi.",
    possessed_by_pt: "Magos Extáticos, Xamãs, Curandeiros, Sedutores, Representantes de Vendas, Terapeutas, Amigos Confiáveis, Nefandi.",
    suggested_specialties: &[
        "Hidden Feelings", "Subtle Cues", "Emotional Manipulation", "Trust", "Sensing Emotions", "Life-Force Fluctuations", "Interpersonal Psychology", "Seeing Past the Mask", "“I Know What You Need”",
    ],
    suggested_specialties_pt: &[
        "Sentimentos Ocultos", "Pistas Sutis", "Manipulação Emocional", "Confiança", "Sentir Emoções", "Flutuações da Força Vital", "Psicologia Interpessoal", "Enxergar Além da Máscara", "“Eu Sei do que Você Precisa”",
    ],
};

pub const EXPRESSION: AbilityDefinition = AbilityDefinition {
    id: "expression",
    name: "Expression",
    name_pt: "Expressão",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 279",
    description: "The gift of articulate communication. Whether through passionate speeches, poetry, prose, or debate, Expression allows you to move hearts, sway crowds, and capture minds.",
    description_pt: "O dom da comunicação articulada e magnética. Seja por meio de discursos inflamados, poesia, prosa ou debate, a Expressão permite tocar corações, comover multidões e convencer mentes.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "When you speak, folks notice.",
            description_pt: "Quando você fala, as pessoas prestam atenção.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Your passions capture other people’s interest.",
            description_pt: "Suas paixões capturam o interesse das outras pessoas.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You rock the crowd with a voice that’s loud.",
            description_pt: "Você agita a multidão com uma voz potente.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "In this media-glutted world, you still blaze bright.",
            description_pt: "Neste mundo saturado de mídia, você ainda brilha intensamente.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Poets, prophets, and politicians wish they were you.",
            description_pt: "Poetas, profetas e políticos gostariam de ser você.",
        },
    ],
    possessed_by: "Rock Stars, Media Pundits, Ecstatic Mages, Executives, Poets, Leaders, Politicians, Bloggers, Rabble-Rousers, Hip-Hop MCs, Professional Celebrities.",
    possessed_by_pt: "Astros do Rock, Comentaristas de Mídia, Magos Extáticos, Executivos, Poetas, Líderes, Políticos, Blogueiros, Agitadores de Multidões, MCs de Hip-Hop, Celebridades Profissionais.",
    suggested_specialties: &[
        "Passion Plays", "Poetic Eloquence", "Motivational Speaking", "Tactical Rhetoric", "Preaching", "Command", "Shut-Downs", "Rocking the Mic",
    ],
    suggested_specialties_pt: &[
        "Dramas Passionais", "Eloquência Poética", "Palestras Motivacionais", "Retórica Tática", "Pregação", "Comando", "Respostas Demolidoras", "Dominar o Microfone",
    ],
};

pub const INTIMIDATION: AbilityDefinition = AbilityDefinition {
    id: "intimidation",
    name: "Intimidation",
    name_pt: "Intimidação",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 279",
    description: "You project authority, dread, or sheer menace. When you choose to, you dominate situations through cold stares, veiled threats, towering presence, or raw aggression.",
    description_pt: "Você projeta autoridade, pavor ou pura ameaça. Quando decide se impor, domina a situação através de olhares gélidos, ameaças veladas, postura imponente ou agressão iminente.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Playground terror.",
            description_pt: "Terror do parquinho.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Clique leader.",
            description_pt: "Líder de grupinho.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Professional dominant.",
            description_pt: "Dominador profissional.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Alpha wolf.",
            description_pt: "Lobo alfa.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Queen or king of all you survey.",
            description_pt: "Rainha ou rei de tudo o que contempla.",
        },
    ],
    possessed_by: "Men in Black, Cyborgs, Cops, Witches, Mad Scientists, Outlaws, Bullies, Beast-Folk, Pro Doms, Drill Sergeants, Samuel L. Jackson.",
    possessed_by_pt: "Homens de Preto, Ciborgues, Policiais, Bruxas, Cientistas Loucos, Criminosos, Valentões, Transmorfos, Dominadores Profissionais, Sargentos Instrutores, Samuel L. Jackson.",
    suggested_specialties: &[
        "Bad Cop", "BDSM", "Pack Hierarchy", "Cold Stare", "Brutal Threats", "Topping From Below", "Pants-Crapping Terror",
    ],
    suggested_specialties_pt: &[
        "Policial Mau", "BDSM", "Hierarquia de Matilha", "Olhar Gélido", "Ameaças Brutais", "Dominar por Baixo", "Terror de Borrar as Calças",
    ],
};

pub const LEADERSHIP: AbilityDefinition = AbilityDefinition {
    id: "leadership",
    name: "Leadership",
    name_pt: "Liderança",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 279",
    description: "A refined art of command, Leadership inspires loyalty, organization, and courage. You rally troops, manage crisis situations, and motivate others to follow your vision.",
    description_pt: "Uma arte refinada de comando, a Liderança inspira lealdade, disciplina e coragem. Você organiza grupos, gerencia situações de crise e motiva os outros a seguirem sua visão.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Head cheerleader.",
            description_pt: "Líder de torcida principal.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Quarterback.",
            description_pt: "Quarterback.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Executive.",
            description_pt: "Executivo.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Tycoon.",
            description_pt: "Magnata.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Nick Fury.",
            description_pt: "Nick Fury.",
        },
    ],
    possessed_by: "Officers, Managers, Pundits, Preachers, Teachers, Team Leaders, Cult Leaders, Gang Leaders, Politicians.",
    possessed_by_pt: "Oficiais Militares, Gerentes, Comentaristas, Pregadores, Professores, Líderes de Equipe, Líderes de Culto, Chefes de Gangue, Políticos.",
    suggested_specialties: &[
        "Competent", "Inspiring", "Regal", "Harsh", "Do-Or-Die", "Soft-Voiced Command",
    ],
    suggested_specialties_pt: &[
        "Competente", "Inspirador", "Régio", "Severo", "Vencer ou Morrer", "Comando em Voz Suave",
    ],
};

pub const STREETWISE: AbilityDefinition = AbilityDefinition {
    id: "streetwise",
    name: "Streetwise",
    name_pt: "Manha",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 279",
    description: "The underworld and back alleys are your home. You understand street politics, criminal codes, fences, drug networks, urban tribes, and how to survive where the law rarely reaches.",
    description_pt: "O submundo e os becos escuros são o seu território. Você domina os códigos das ruas, redes de receptação, gírias do crime, tribos urbanas e sabe sobreviver onde a lei não entra.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Rock musician.",
            description_pt: "Músico de rock.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Suburban dope dealer.",
            description_pt: "Traficante de subúrbio.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Hip-Hop MC.",
            description_pt: "MC de Hip-Hop.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Gang leader.",
            description_pt: "Líder de gangue.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Gang lord.",
            description_pt: "Chefão do crime.",
        },
    ],
    possessed_by: "Bartenders, Hookers, Hackers, Pimps, Street People, Cops, Detectives, Street Ministers, Musicians, Runaways, Drug Dealers, Activists, Journalists, Taxi Drivers.",
    possessed_by_pt: "Bartenders, Garotas de Programa, Hackers, Cafetões, Moradores de Rua, Policiais, Detetives, Pastores de Rua, Músicos, Fugitivos, Traficantes de Drogas, Ativistas, Jornalistas, Taxistas.",
    suggested_specialties: &[
        "Theft", "Survival Skills", "Outlaw Hacker Culture", "Crash Space", "Street Politics", "Urban Tribes", "Trust", "Hiding Spots", "Sex Trade", "Drug Trade", "Music Culture", "Gangs", "Seeing Nothing Yet Everything",
    ],
    suggested_specialties_pt: &[
        "Furto", "Habilidades de Sobrevivência", "Cultura Hacker Marginal", "Acolhimento Clandestino", "Política das Ruas", "Tribos Urbanas", "Confiança", "Esconderijos", "Comércio Sexual", "Tráfico de Drogas", "Cultura Musical", "Gangues", "Não Ver Nada e Ver Tudo",
    ],
};

pub const SUBTERFUGE: AbilityDefinition = AbilityDefinition {
    id: "subterfuge",
    name: "Subterfuge",
    name_pt: "Lábia",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 279",
    description: "You're adept at concealing your true motives, telling convincing lies, seeing through deception, and manipulating social dynamics to your benefit without getting caught.",
    description_pt: "Você domina a arte de disfarçar intenções, mentir com convicção absoluta, farejar farsas alheias e manipular dinâmicas sociais em seu proveito sem deixar rastros.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Cute kid.",
            description_pt: "Criança fofa.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Small-time hustler.",
            description_pt: "Trapaceiro pé-de-chinelo.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Successful lawyer.",
            description_pt: "Advogado bem-sucedido.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Career politician.",
            description_pt: "Político de carreira.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Master spy.",
            description_pt: "Mestre espião.",
        },
    ],
    possessed_by: "Syndicate Agents, Secret Agents, Home-Wreckers, Ad Agency Personnel, Lobbyists, Crooks, Shapeshifters, Infiltrators, Embezzlers, Hustlers, Attorneys, Sociopaths.",
    possessed_by_pt: "Agentes do Sindicato, Agentes Secretos, Destruidores de Lares, Funcionários de Agências de Publicidade, Lobistas, Escroques, Transmorfos, Infiltrados, Desviadores de Fundos, Trapaceiros, Advogados, Sociopatas.",
    suggested_specialties: &[
        "Con Games", "Sexual Manipulation", "False Innocence", "Politics", "Law", "Cheating", "Undermining Opposition", "Sensing Deceptions",
    ],
    suggested_specialties_pt: &[
        "Golpes", "Manipulação Sexual", "Falsa Inocência", "Política", "Direito", "Trapaça", "Solapar a Oposição", "Sentir Decepções",
    ],
};

pub const CORE_TALENTS: &[AbilityDefinition] = &[
    ALERTNESS,
    ART,
    ATHLETICS,
    AWARENESS,
    BRAWL,
    EMPATHY,
    EXPRESSION,
    INTIMIDATION,
    LEADERSHIP,
    STREETWISE,
    SUBTERFUGE,
];
