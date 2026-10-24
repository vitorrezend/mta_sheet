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
    page_ref: "M20, p. 276",
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
            description_pt: "Para você, aqueles ninjas poderiam muito bem estar usando fantasias de palhaço neon.",
        },
    ],
    possessed_by: "Spies, Bodyguards, Feral Kids, Street Survivors, Shapeshifters, War Veterans.",
    possessed_by_pt: "Espiões, Guarda-costas, Crianças Selvagens, Sobreviventes das Ruas, Transmorfos, Veteranos de Guerra.",
    suggested_specialties: &[
        "Ambushes", "Instincts", "Omens", "Urban Wastelands", "Combat Zones", "Covert Pursuit", "Warning Signs", "Combat Reflexes",
    ],
    suggested_specialties_pt: &[
        "Emboscadas", "Instintos", "Presságios", "Ruínas Urbanas", "Zonas de Combate", "Perseguição Oculta", "Sinais de Alerta", "Reflexos de Combate",
    ],
};

pub const ART: AbilityDefinition = AbilityDefinition {
    id: "art",
    name: "Art",
    name_pt: "Arte",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 276-277",
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
            description_pt: "Você é um profissional atuante com apelo visual e destreza fascinantes.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "You’re an acknowledged master of your field.",
            description_pt: "Você é um mestre consagrado e aclamado em sua área.",
        },
    ],
    possessed_by: "Art Students, Fine Artists, Professional Artists, Dedicated Hobbyists, Reclusive Geniuses, Mages Who Focus Magick Through Art.",
    possessed_by_pt: "Estudantes de Arte, Artistas Plásticos, Ilustradores Profissionais, Hobbistas Dedicados, Gênios Reclusos, Magos que Focam Mágika na Arte.",
    suggested_specialties: &[
        "Painting", "Video", "Singing", "Acting", "Dance", "Sculpture", "Mixed Media", "CGI", "Classical Art", "Avant-Garde", "Primitive Techniques",
    ],
    suggested_specialties_pt: &[
        "Pintura", "Vídeo", "Canto", "Atuação", "Dança", "Escultura", "Mídia Mista", "CGI", "Arte Clássica", "Vanguarda", "Técnicas Primitivas",
    ],
};

pub const ATHLETICS: AbilityDefinition = AbilityDefinition {
    id: "athletics",
    name: "Athletics",
    name_pt: "Esportes",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 277-278",
    description: "Fitness is your passion. While other folks sit on the sidelines, you’re running, climbing, swimming, biking, and developing physical prowess with intense practice. 'A sound mind in a sound body' is your motto.",
    description_pt: "O condicionamento físico é sua paixão. Enquanto outros assistem da arquibancada, você corre, escala, nada, pedala e desenvolve vigor com treino intenso. 'Mente sã em corpo são' é o seu lema.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You were pretty decent in PE class.",
            description_pt: "Você se saía bem nas aulas de educação física.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Regular exercise is essential to your life.",
            description_pt: "Exercícios regulares são fundamentais na sua rotina.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You are in superb physical shape.",
            description_pt: "Você possui uma forma física invejável e atlética.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Gym-rats and pro athletes take notes from you.",
            description_pt: "Atletas profissionais e marombeiros pedem suas dicas.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "You could easily compete at the Olympic Games.",
            description_pt: "Você possui nível para disputar e brilhar nos Jogos Olímpicos.",
        },
    ],
    possessed_by: "Jocks, Fitness Gurus, Personal Trainers, Professional Dancers, Martial Artists, Stunt Performers, Soldiers.",
    possessed_by_pt: "Atletas, Treinadores Pessoais, Dançarinos Profissionais, Praticantes de Artes Marciais, Dublês, Militares.",
    suggested_specialties: &[
        "Bodybuilding", "Cycling", "Climbing", "Equestrian", "Swimming", "Survival Training", "Extreme Endurance", "Parkour", "Skiing",
    ],
    suggested_specialties_pt: &[
        "Musculação", "Ciclismo", "Escalada", "Hipismo", "Natação", "Treinamento de Sobrevivência", "Resistência Extrema", "Parkour", "Esqui",
    ],
};

pub const AWARENESS: AbilityDefinition = AbilityDefinition {
    id: "awareness",
    name: "Awareness",
    name_pt: "Consciência",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 278-279",
    description: "You have uncanny perceptions attuned to supernatural energies, mystical resonance, invisible entities, and the subtle warping of reality that ordinary mortals miss entirely.",
    description_pt: "Você possui percepções insólitas sintonizadas com energias sobrenaturais, ressonância mágica, entidades invisíveis e as distorções sutis da realidade que os mortais comuns ignoram.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Vague shivers, goosebumps, and feelings of unseen presence.",
            description_pt: "Calafrios vagos, arrepios e sensação de presenças ocultas.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "You can identify the broad nature of supernatural disturbances.",
            description_pt: "Você consegue identificar a natureza geral de perturbações sobrenaturais.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You sense nuances of Resonance, Quintessence flows, and nearby spirits.",
            description_pt: "Você sente nuances de Ressonância, fluxos de Quintessência e espíritos próximos.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Few unseen things can hide from your sixth sense.",
            description_pt: "Raras entidades ou manipulações etéreas conseguem escapar do seu sexto sentido.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "The unseen world is as vivid to you as the physical one.",
            description_pt: "O mundo invisível é tão nítido para você quanto o mundo físico.",
        },
    ],
    possessed_by: "Psychics, Mediums, Shamans, Sensitive Mortals, Witches, Paranormal Investigators.",
    possessed_by_pt: "Médiuns, Clarividentes, Xamãs, Mortais Sensitivos, Bruxos, Investigadores Paranormais.",
    suggested_specialties: &[
        "Resonance", "Auras", "Spirits", "Quintessence", "Paradox", "Gauntlet Shifts", "Sixth Sense",
    ],
    suggested_specialties_pt: &[
        "Ressonância", "Auras", "Espíritos", "Quintessência", "Paradoxo", "Oscilações na Película", "Sexto Sentido",
    ],
};

pub const BRAWL: AbilityDefinition = AbilityDefinition {
    id: "brawl",
    name: "Brawl",
    name_pt: "Briga",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 279",
    description: "When words fail, hands and feet do the talking. Brawl represents raw, practical hand-to-hand combat: punching, kicking, wrestling, biting, gouging, and dirty infighting.",
    description_pt: "Quando o diálogo falha, mãos e pés resolvem. Briga representa o combate desarmado cru e prático: socos, chutes, agarramentos, cabeçadas, mordidas e luta suja de rua.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You can hold your own in a playground scuffle.",
            description_pt: "Você consegue se defender num bate-boca escolar.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Bar fights don't intimidate you.",
            description_pt: "Brigas de bar e confusões comuns não o intimidam.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Tough brawler; you hit hard and know where to strike.",
            description_pt: "Lutador cascudo; bate pesado e sabe onde acertar para machucar.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Prizefighter or military close-combat instructor.",
            description_pt: "Nível de pugilista profissional ou instrutor militar de combate corporal.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Your fists are lethal weapons of staggering devastation.",
            description_pt: "Seus punhos são armas letais de impacto devastador.",
        },
    ],
    possessed_by: "Bouncers, Gang Members, Soldiers, Street Fighters, Martial Artists, Tough Cops.",
    possessed_by_pt: "Seguranças de Balada, Membros de Gangue, Soldados, Lutadores de Rua, Policiais Durões.",
    suggested_specialties: &[
        "Dirty Infighting", "Grappling", "Boxing", "Kicks", "Throws", "Biting", "Disarming",
    ],
    suggested_specialties_pt: &[
        "Golpes Baixos", "Imobilizações", "Boxe", "Chutes", "Arremessos", "Mordidas", "Desarme",
    ],
};

pub const EMPATHY: AbilityDefinition = AbilityDefinition {
    id: "empathy",
    name: "Empathy",
    name_pt: "Empatia",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 279-280",
    description: "You understand how people feel, why they feel that way, and what drives their emotions. Empathy allows you to read moods, detect deception, offer solace, and build deep connections.",
    description_pt: "Você compreende o que os outros sentem, as motivações por trás de suas emoções e seus anseios mais profundos. A Empatia permite ler humores, detectar mentiras, consolar e criar laços sinceros.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "A kind, perceptive listener.",
            description_pt: "Um ouvinte atencioso e compreensivo.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "You easily spot when someone is faking their feelings.",
            description_pt: "Você percebe facilmente quando alguém está fingindo sentimentos.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You read interpersonal dynamics like an open book.",
            description_pt: "Você decifra dinâmicas interpessoais como um livro aberto.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Counselor or interrogator who uncovers deepest secrets.",
            description_pt: "Terapeuta ou interrogador capaz de alcançar as dores mais ocultas.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Near-telepathic emotional connection to the human soul.",
            description_pt: "Conexão emocional quase telepática com as dores da alma humana.",
        },
    ],
    possessed_by: "Psychologists, Caregivers, Negotiators, Actors, Spiritual Guides, Spies.",
    possessed_by_pt: "Psicólogos, Cuidadores, Mediadores, Atores, Guias Espirituais, Espiões.",
    suggested_specialties: &[
        "Lies", "Hidden Motives", "Comforting", "Reading Body Language", "Micro-expressions", "Romance",
    ],
    suggested_specialties_pt: &[
        "Detecção de Mentiras", "Motivações Ocultas", "Acolhimento", "Linguagem Corporal", "Microexpressões", "Romance",
    ],
};

pub const EXPRESSION: AbilityDefinition = AbilityDefinition {
    id: "expression",
    name: "Expression",
    name_pt: "Expressão",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 280",
    description: "The gift of articulate communication. Whether through passionate speeches, poetry, prose, or debate, Expression allows you to move hearts, sway crowds, and capture minds.",
    description_pt: "O dom da comunicação articulada e magnética. Seja por meio de discursos inflamados, poesia, prosa ou debate, a Expressão permite tocar corações, comover multidões e convencer mentes.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You speak and write clearly without awkwardness.",
            description_pt: "Fala e escreve com clareza e desenvoltura básica.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Compelling speaker and persuasive writer.",
            description_pt: "Orador envolvente e redator persuasivo.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "You can move an audience to tears or thunderous applause.",
            description_pt: "Capaz de levar uma plateia às lágrimas ou a aplausos estrondosos.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Celebrated orator, published author, or charismatic pundit.",
            description_pt: "Orador aclamado, autor publicado ou líder de opinião carismático.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Your words shape generations and alter political destinies.",
            description_pt: "Suas palavras inspiram gerações e definem rumos políticos.",
        },
    ],
    possessed_by: "Politicians, Poets, Preachers, Activists, Journalists, Actors, Teachers.",
    possessed_by_pt: "Políticos, Poetas, Pregadores, Ativistas, Jornalistas, Atores, Educadores.",
    suggested_specialties: &[
        "Speeches", "Poetry", "Debate", "Sermons", "Storytelling", "Improvisation", "Manifestos",
    ],
    suggested_specialties_pt: &[
        "Discursos", "Poesia", "Debate", "Sermões", "Contação de Histórias", "Improviso", "Manifestos",
    ],
};

pub const INTIMIDATION: AbilityDefinition = AbilityDefinition {
    id: "intimidation",
    name: "Intimidation",
    name_pt: "Intimidação",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 280-281",
    description: "You project authority, dread, or sheer menace. When you choose to, you dominate situations through cold stares, veiled threats, towering presence, or raw aggression.",
    description_pt: "Você projeta autoridade, pavor ou pura ameaça. Quando decide se impor, domina a situação através de olhares gélidos, ameaças veladas, postura imponente ou agressão iminente.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Playground bully; you know how to puff out your chest.",
            description_pt: "Valentão amador; sabe estufar o peito para parecer maior.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "You back people down when tempers flare.",
            description_pt: "Faz os outros recuarem quando os ânimos se exaltam.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Professional dominant; folks think twice before crossing you.",
            description_pt: "Dominador nato; ninguém ousa cruzar seu caminho sem hesitar.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Alpha predator; rooms fall dead silent when you glare.",
            description_pt: "Predador alfa; a sala inteira congela quando você fixa o olhar.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Terror incarnate; grown killers tremble in your presence.",
            description_pt: "O terror em pessoa; assassinos calejados tremem diante de você.",
        },
    ],
    possessed_by: "Men in Black, Cyborgs, Drill Sergeants, Mob Enforcers, Bouncers, Dark Witches.",
    possessed_by_pt: "Homens de Preto, Ciborgues, Sargentos Instrutores, Capangas da Máfia, Bruxas Sombrias.",
    suggested_specialties: &[
        "Cold Stare", "Veiled Threats", "Physical Menace", "Bad Cop", "Interrogation", "Authority",
    ],
    suggested_specialties_pt: &[
        "Olhar Gélido", "Ameaças Veladas", "Ameaça Física", "Policial Mau", "Interrogatório", "Autoridade",
    ],
};

pub const LEADERSHIP: AbilityDefinition = AbilityDefinition {
    id: "leadership",
    name: "Leadership",
    name_pt: "Liderança",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 281",
    description: "A refined art of command, Leadership inspires loyalty, organization, and courage. You rally troops, manage crisis situations, and motivate others to follow your vision.",
    description_pt: "Uma arte refinada de comando, a Liderança inspira lealdade, disciplina e coragem. Você organiza grupos, gerencia situações de crise e motiva os outros a seguirem sua visão.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Team captain or group project coordinator.",
            description_pt: "Capitão de equipe ou líder de trabalho em grupo.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Respected crew chief or middle manager.",
            description_pt: "Chefe respeitado ou gestor de equipe eficiente.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Charismatic officer whose team follows through danger.",
            description_pt: "Oficial carismático cuja equipe o segue mesmo no perigo.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Inspiring general or corporate tycoon with fanatical backing.",
            description_pt: "General brilhante ou magnata que inspira lealdade inabalável.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Legendary leader whose legacy shifts the course of history.",
            description_pt: "Líder lendário cujo comando altera o destino de nações inteiras.",
        },
    ],
    possessed_by: "Military Officers, Cabal Leaders, Executives, Political Figures, Gang Bosses, Teachers.",
    possessed_by_pt: "Oficiais Militares, Líderes de Cabala, Executivos, Figuras Políticas, Chefes de Gangue.",
    suggested_specialties: &[
        "Inspirational", "Command", "Crisis Management", "Tactical", "Mentorship", "Diplomatic",
    ],
    suggested_specialties_pt: &[
        "Inspirador", "Comando Direto", "Gestão de Crise", "Tático", "Mentoria", "Diplomático",
    ],
};

pub const STREETWISE: AbilityDefinition = AbilityDefinition {
    id: "streetwise",
    name: "Streetwise",
    name_pt: "Manha",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, p. 281",
    description: "The underworld and back alleys are your home. You understand street politics, criminal codes, fences, drug networks, urban tribes, and how to survive where the law rarely reaches.",
    description_pt: "O submundo e os becos escuros são o seu território. Você domina os códigos das ruas, redes de receptação, gírias do crime, tribos urbanas e sabe sobreviver onde a lei não entra.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "You know which alleys to avoid at night.",
            description_pt: "Sabe quais becos evitar à noite para não ser assaltado.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "You can find local hookups, party spots, and contraband.",
            description_pt: "Consegue contatos locais, festas clandestinas e contrabando.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Connected operator trusted by local street figures.",
            description_pt: "Operador articulado e respeitado pelos malandros locais.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Underworld broker with deep insight into gangs and crime cartels.",
            description_pt: "Articulador do submundo com livre trânsito em várias gangues.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Godfather of the pavement; the streets whisper their secrets to you.",
            description_pt: "O chefão do asfalto; o submundo da cidade curva-se à sua presença.",
        },
    ],
    possessed_by: "Criminals, Hackers, Undercover Cops, Bartenders, Homeless People, Runaways, Activists.",
    possessed_by_pt: "Criminosos, Hackers, Policiais Disfarçados, Bartenders, Moradores de Rua, Ativistas.",
    suggested_specialties: &[
        "Black Market", "Gangs", "Urban Survival", "Drug Trade", "Fencing Goods", "Street Rumors",
    ],
    suggested_specialties_pt: &[
        "Mercado Negro", "Gangues", "Sobrevivência Urbana", "Tráfico", "Receptação", "Boatos da Rua",
    ],
};

pub const SUBTERFUGE: AbilityDefinition = AbilityDefinition {
    id: "subterfuge",
    name: "Subterfuge",
    name_pt: "Lábia",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Core,
    page_ref: "M20, pp. 281-282",
    description: "You're adept at concealing your true motives, telling convincing lies, seeing through deception, and manipulating social dynamics to your benefit without getting caught.",
    description_pt: "Você domina a arte de disfarçar intenções, mentir com convicção absoluta, farejar farsas alheias e manipular dinâmicas sociais em seu proveito sem deixar rastros.",
    ratings: &[
        AbilityRating {
            dots: 1,
            title: "Novice",
            title_pt: "Novato",
            description: "Capable of telling harmless white lies with a straight face.",
            description_pt: "Conta pequenas mentiras sociais sem vacilar.",
        },
        AbilityRating {
            dots: 2,
            title: "Practiced",
            title_pt: "Praticante",
            description: "Skilled hustler who avoids tipping his hand.",
            description_pt: "Trapaceiro habilidoso que raramente entrega o jogo.",
        },
        AbilityRating {
            dots: 3,
            title: "Skillful",
            title_pt: "Competente",
            description: "Smooth operator; successful lawyer or lobbyist.",
            description_pt: "Negociador afiado; nível de advogado perspicaz ou lobista.",
        },
        AbilityRating {
            dots: 4,
            title: "Expert",
            title_pt: "Especialista",
            description: "Career politician or master infiltrator.",
            description_pt: "Político de carreira ou infiltrado de alto escalão.",
        },
        AbilityRating {
            dots: 5,
            title: "Master",
            title_pt: "Mestre",
            description: "Machiavelli reborn; your webs of deceit are flawless works of art.",
            description_pt: "Maquiavel renascido; suas teias de intriga são perfeitas obras de arte.",
        },
    ],
    possessed_by: "Secret Agents, Syndicate Executives, Con Artists, Politicians, Spies, Lawyers.",
    possessed_by_pt: "Agentes Secretos, Executivos do Sindicato, Golpistas, Políticos, Espiões, Advogados.",
    suggested_specialties: &[
        "Con Games", "Detecting Lies", "Seduction", "Innocence", "Fast-Talking", "Double Entendre",
    ],
    suggested_specialties_pt: &[
        "Golpes", "Detectar Mentiras", "Sedução", "Falsa Inocência", "Lero-Lero", "Duplo Sentido",
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
