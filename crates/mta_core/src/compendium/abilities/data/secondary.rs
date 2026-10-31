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
    page_ref: "M20, pp. 289-291",
    description: "You have a natural affinity for animals, connecting through body language, primal vocalizations, and instinctive empathy to calm beasts, train creatures, and read mood or intent.",
    description_pt: "Você possui uma afinidade natural com animais, conectando-se através de linguagem corporal, vocalizações primitivas e empatia instintiva para acalmar feras, adestrar criaturas e ler humores ou intenções.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Domesticated animals tend to like you.", description_pt: "Animais domesticados costumam gostar de você." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Skittish animals tend to trust you.", description_pt: "Animais ariscos costumam confiar em você." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Wild animals tend to respect you.", description_pt: "Animais selvagens costumam respeitar você." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Folks call you “the Beast Whisperer.”", description_pt: "As pessoas o chamam de “o Encantador de Feras”." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "You seem closer to animals than to human beings.", description_pt: "Você parece mais próximo dos animais do que dos seres humanos." },
    ],
    possessed_by: "Shapeshifters, Farmers, Hunters, Grooms, Witches, K-9 Cops, Animal Trainers, Pet Companions, Naturalists, Feral People Raised By Beasts.",
    possessed_by_pt: "Transmorfos, Fazendeiros, Caçadores, Cavalariços, Bruxas, Policiais do Canil (K-9), Treinadores de Animais, Companheiros de Bichos de Estimação, Naturalistas, Pessoas Ferais Criadas por Feras.",
    suggested_specialties: &["Dominance", "Befriending", "Calming", "Bonding", "Reading Signs", "“Talk To the Animals”", "Training (specific animals — Canines & Lupines, Felines, Horses, Raptors, etc.)"],
    suggested_specialties_pt: &["Dominância", "Fazer Amizade", "Acalmar", "Criar Vínculo", "Ler Sinais", "“Falar com os Animais”", "Adestramento (animais específicos — Canídeos e Lupinos, Felinos, Cavalos, Aves de Rapina, etc.)"],
};

pub const BLATANCY: AbilityDefinition = AbilityDefinition {
    id: "blatancy",
    name: "Blatancy",
    name_pt: "Descaramento",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 291-292",
    description: "Through theatrical flair, bold excuses ('We're filming a movie!'), and sheer audacity, you disguise blatant magical occurrences as special effects, stunts, or optical illusions.",
    description_pt: "Com talento teatral, desculpas audaciosas ('Estamos gravando um filme!') e pura audácia, você disfarça acontecimentos mágicos escancarados como efeitos especiais, pegadinhas ou ilusões de ótica.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "“No, seriously — we’re filming a video.”", description_pt: "“Não, sério — estamos gravando um vídeo.”" },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "“That just looked easy. It took me years to figure out how to do that trick.”", description_pt: "“Isso só pareceu fácil. Levei anos para descobrir como fazer esse truque.”" },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "“All the necessary permits and explanations are posted on our website.”", description_pt: "“Todas as autorizações necessárias e explicações estão publicadas em nosso site.”" },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "“We did this trick on Mythbusters.”", description_pt: "“Fizemos esse truque no Mythbusters (Caçadores de Mitos).”" },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "“Nothing to see here — move along.”", description_pt: "“Nada para ver aqui — continuem andando.”" },
    ],
    possessed_by: "Celebrities, Media Magicians, Pundits, Deejays, SPX Techs, Filmmakers, Stunt Artists, Martial Artists, Black Suits, Show Promoters.",
    possessed_by_pt: "Celebridades, Ilusionistas de Mídia, Comentaristas, DJs, Técnicos de Efeitos Especiais, Cineastas, Dublês, Artistas Marciais, Ternos Pretos, Promotores de Shows.",
    suggested_specialties: &["Religious Appeal", "Political Paranoia", "Stage Magic", "Film Crew", "Appeals to Vanity", "Cool Gadgets", "Geek Culture", "Sheer Balls", "Legal Authority", "Science!", "We’re On a Mission From God"],
    suggested_specialties_pt: &["Apelo Religioso", "Paranoia Política", "Mágica de Palco", "Equipe de Filmagem", "Apelo à Vaidade", "Aparatos Legais", "Cultura Geek", "Pura Cara de Pau", "Autoridade Legal", "Ciência!", "Estamos em uma Missão Enviada por Deus"],
};

pub const CAROUSING: AbilityDefinition = AbilityDefinition {
    id: "carousing",
    name: "Carousing",
    name_pt: "Farra",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 292",
    description: "The fine art of partying, drinking without collapsing, making fast friends in bars, discovering illicit nightlife hotspots, and turning revelry into social currency.",
    description_pt: "A arte de festejar intensamente, beber sem cair de bêbado, fazer amigos rápidos em bares, descobrir pontos quentes da vida noturna e transformar folia em capital social.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "“YOLO!”", description_pt: "“Só se vive uma vez! (YOLO)”" },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "“Do a little dance/ Make a little love/ Get down tonight…”", description_pt: "“Dançar um pouco / Amar um pouco / Curtir a noite…”" },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "“Sin, that you might be forgiven.”", description_pt: "“Peque, para que possa ser perdoado.”" },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "“The only real blasphemy is the refusal of joy.”", description_pt: "“A única blasfêmia real é a recusa da alegria.”" },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "“’Do what thou wilt’ shall be the whole of the law.”", description_pt: "“‘Faze o que tu queres’ há de ser tudo da Lei.”" },
    ],
    possessed_by: "Music Stars, Party Animals, Frat Boys, Bon Vivants, Cult Leaders, Ecstatics, Nephandic Seducers, Agents of Subversion, Left-Hand Path Devotees.",
    possessed_by_pt: "Astros da Música, Festeiros Incorrigíveis, Universitários Farristas, Bon Vivants, Líderes de Seitas, Extáticos, Sedutores Nefandi, Agentes da Subversão, Devotos da Senda da Mão Esquerda.",
    suggested_specialties: &["High Society", "Teenage Mayhem", "Overcoming Inhibitions", "Violating Vows", "Classy Debauchery", "Wild Parties", "Master of Ceremonies", "Mystic Justifications", "Get This Party Started"],
    suggested_specialties_pt: &["Alta Sociedade", "Baderna Adolescente", "Vencer Inibições", "Violar Votos", "Devassidão Elegante", "Festas Selvagens", "Mestre de Cerimônias", "Justificativas Místicas", "Começar a Festa"],
};

pub const DO: AbilityDefinition = AbilityDefinition {
    id: "do",
    name: "Do",
    name_pt: "Do",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 292-293",
    description: "The sacred, esoteric martial art of the Akashic Brotherhood. Do blends fluid physical motion, meditation, Chi manipulation, pressure point mastery, and reality-shaping focus.",
    description_pt: "A arte marcial sagrada e esotérica da Irmandade de Akasha. O Do une movimento físico fluido, meditação, manipulação de Chi, domínio de pontos de pressão e foco capaz de moldar a realidade.",
    ratings: &[
        AbilityRating { dots: 1, title: "Insect (ant, grasshopper, mantis, roach, etc.)", title_pt: "Inseto (formiga, gafanhoto, louva-a-deus, barata, etc.)", description: "Initiated into the simplest secrets of Do, you’ve begun learning the proper methods of diet, meditation, and exercise.", description_pt: "Iniciado nos segredos mais simples do Do, você começou a aprender os métodos adequados de dieta, meditação e exercício." },
        AbilityRating { dots: 2, title: "Crawler (lizard, snake, toad, etc.)", title_pt: "Rastejante (lagarto, serpente, sapo, etc.)", description: "Like the beasts that crawl upon the earth, you’re beginning to discover the keys to a higher state. Your body, mind, and spirit have achieved a kind of unity; with that understanding, you’re capable of impressive feats.", description_pt: "Assim como as feras que rastejam pela terra, você começa a descobrir as chaves para um estado superior. Seu corpo, mente e espírito alcançaram certa unidade; com essa compreensão, você é capaz de feitos impressionantes." },
        AbilityRating { dots: 3, title: "Four-Footed Animal (horse, dog, cat, etc.)", title_pt: "Animal Quadrúpede (cavalo, cão, gato, etc.)", description: "Like the greater beasts, you have unlocked the power of natural attainment. Your hands become like claws or hooves; your body surges with a strength unknown to most of those who would consider themselves human. This is the highest level many devotees achieve, the rank of an accomplished Tao-shih.", description_pt: "Como as grandes feras, você desbloqueou o poder da realização natural. Suas mãos tornam-se como garras ou cascos; seu corpo pulsa com uma força desconhecida pela maioria dos que se consideram humanos. Este é o nível mais alto que muitos devotos alcançam, o posto de um realizado Tao-shih." },
        AbilityRating { dots: 4, title: "Bipedal Animal (monkey, bear, ape, etc.)", title_pt: "Animal Bípede (macaco, urso, símio, etc.)", description: "Through great effort and understanding, you’ve opened the gates that hold lesser beings in their distracted state. Your body, mind, and spirit can accomplish miraculous things, and you have earned the right to be considered a true heir of Do, a Sihing. To go beyond this point requires supreme devotion and a practice that typically takes lifetimes to refine.", description_pt: "Por meio de grande esforço e compreensão, você abriu os portões que aprisionam seres menores em seu estado de distração. Seu corpo, mente e espírito conseguem realizar coisas milagrosas, e você conquistou o direito de ser considerado um verdadeiro herdeiro do Do, um Sihing. Ir além deste ponto exige devoção suprema e uma prática que geralmente leva vidas para ser refinada." },
        AbilityRating { dots: 5, title: "True Humanity", title_pt: "Humanidade Verdadeira", description: "Your devotion has returned you to the original state of human grace. At this point, you no longer concern yourself with what can be done, but merely with what should be done. Only now might you be regarded as a Sifu: a Master of the Way.", description_pt: "Sua devoção o fez retornar ao estado original de graça humana. Neste ponto, você já não se preocupa com o que pode ser feito, mas meramente com o que deve ser feito. Somente agora você pode ser considerado um Sifu: um Mestre do Caminho." },
    ],
    possessed_by: "Akashayana and perhaps Their Most Trusted Companions.",
    possessed_by_pt: "Akashayana e talvez Seus Companheiros Mais Confiáveis.",
    suggested_specialties: &["External (forceful) Style", "Internal (meditative) Style", "Weapon Forms", "Energetic Alchemy", "Katas", "Animal-Form Techniques"],
    suggested_specialties_pt: &["Estilo Externo (vigoroso)", "Estilo Interno (meditativo)", "Formas com Armas", "Alquimia Energética", "Katas", "Técnicas em Forma Animal"],
};

pub const SEDUCTION: AbilityDefinition = AbilityDefinition {
    id: "seduction",
    name: "Seduction",
    name_pt: "Sedução",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 295",
    description: "The subtle craft of enticing, flirting, arousing desire, intoxicating minds, and making others crave your company, approval, or romantic intimacy.",
    description_pt: "A arte sutil de atrair, flertar, despertar desejos, inebriar mentes e fazer com que os outros anseiem por sua companhia, aprovação ou intimidade romântica.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "You can score desperate drunks.", description_pt: "Você consegue fisgar bêbados desesperados." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "You can have almost anyone you want.", description_pt: "Você pode ter quase qualquer pessoa que quiser." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "You’re good enough to get people to go against their better natures.", description_pt: "Você é bom o bastante para fazer pessoas agirem contra sua própria natureza." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Turn on the charm, turn off their brains.", description_pt: "Ligue o charme, desligue o cérebro deles." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Vows, homes, hearts, and marriages are yours to break at will.", description_pt: "Votos, lares, corações e casamentos estão à sua disposição para serem despedaçados à vontade." },
    ],
    possessed_by: "Spies, Manipulators, Users, Subverters, Ecstatics, Nephandi, Pretty Strangers, Demon Lovers, Vampires in Many Senses of That Word.",
    possessed_by_pt: "Espiões, Manipuladores, Aproveitadores, Subversores, Extáticos, Nefandi, Estranhos Atraentes, Amantes Demoníacos, Vampiros em Vários Sentidos dessa Palavra.",
    suggested_specialties: &["Raw Sex", "Appealing to Kinks", "False Innocence", "Topping From Below", "Subtle Moves", "“Let’s be Bad Guys”", "Crude But Effective"],
    suggested_specialties_pt: &["Sexo Explícito", "Apelo a Fetiches", "Falsa Inocência", "Dominar por Baixo", "Movimentos Sutis", "“Vamos Ser Vilões”", "Cru Porém Eficaz"],
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
    page_ref: "M20, p. 295",
    description: "Tumbling, gymnastic flips, tightrope balance, safe falls, trapeze routines, and leaping across rooftops with gravity-defying athletic precision.",
    description_pt: "Cambalhotas, saltos mortais, equilíbrio em corda bamba, amortecimento de quedas, trapézio e saltos sobre telhados com precisão atlética que desafia a gravidade.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Practiced tumbler.", description_pt: "Acrobata praticante de solo." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "High school gymnast.", description_pt: "Ginasta de ensino médio." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Professional acrobat.", description_pt: "Acrobata profissional." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Olympic-level contender.", description_pt: "Competidor de nível olímpico." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Cirque Du Soleil artist.", description_pt: "Artista do Cirque du Soleil." },
    ],
    possessed_by: "Martial Artists, Circus Stunt Performers, Performers, Dancers, Gymnasts, Aerialists, Extreme Athletes, Shapeshifters.",
    possessed_by_pt: "Artistas Marciais, Artistas e Dublês Circenses, Performers, Dançarinos, Ginastas, Trapezistas/Acrobatas Aéreos, Atletas Radicais, Transmorfos.",
    suggested_specialties: &["Aerial Arts", "Martial Arts", "Stunts", "Falls", "Trapeze", "Competition Gymnastics", "Floor Work", "Enhanced Jumping", "Parkour"],
    suggested_specialties_pt: &["Artes Aéreas", "Artes Marciais", "Acrobacias/Dublê", "Quedas", "Trapézio", "Ginástica de Competição", "Exercícios de Solo", "Saltos Ampliados", "Parkour"],
};

pub const ARCHERY: AbilityDefinition = AbilityDefinition {
    id: "archery",
    name: "Archery",
    name_pt: "Arquearia",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 295",
    description: "Shooting humanity's oldest ranged weapon: traditional longbows, compound hunting bows, and modern mechanical crossbows with silent, deadly accuracy.",
    description_pt: "Disparar a mais antiga arma de longo alcance da humanidade: arcos longos tradicionais, arcos compostos de caça e balestras mecânicas modernas com precisão silenciosa e mortal.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Kid in archery class.", description_pt: "Criança na aula de tiro com arco." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "SCAdian archer.", description_pt: "Arqueiro da SCA (reconstituição medieval)." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Bow-hunter.", description_pt: "Caçador com arco." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Katniss Everdeen.", description_pt: "Katniss Everdeen." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Legolas.", description_pt: "Legolas." },
    ],
    possessed_by: "Snipers, Hunters, Commandos, Assassins, Survivalists, Athletes, Outlaws, Recreationists, Folks From Arboreal Cultures, Fans of The Hunger Games, Would-Be Elves.",
    possessed_by_pt: "Atiradores de Elite, Caçadores, Comandos, Assassinos, Sobrevivencialistas, Atletas, Foras da Lei, Recreacionistas, Habitantes de Culturas Arbóreas, Fãs de Jogos Vorazes, Aspirantes a Elfos.",
    suggested_specialties: &["Longbow", "Crossbow", "Hunting", "Horse-Archery", "Trick Shots", "Killing Shots", "Kyudou Techniques", "Arched Flight", "Primitive Archery"],
    suggested_specialties_pt: &["Arco Longo", "Besta / Balestra", "Caça", "Arquearia Montada", "Tiros Especiais", "Tiros Letais", "Técnicas de Kyudo", "Voo Parabólico", "Arquearia Primitiva"],
};

pub const BIOTECH: AbilityDefinition = AbilityDefinition {
    id: "biotech",
    name: "Biotech",
    name_pt: "Biotecnologia",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 295-296",
    description: "Hands-on engineering of cybernetic implants, prosthetic interfaces, cloned tissue grafts, synthetic organs, and biological wetware.",
    description_pt: "Engenharia prática de implantes cibernéticos, interfaces de próteses, enxertos de tecido clonado, órgãos sintéticos e biomaquinário conectado ao sistema nervoso.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Given a basic understanding of biotech principles, you’ve seen the design and implementation of such technology. Despite your lack of experience, you might be able to make very simple repairs if need be.", description_pt: "Com uma compreensão básica dos princípios da biotecnologia, você já viu o projeto e a implementação de tal tecnologia. Apesar da falta de experiência, você pode ser capaz de fazer reparos muito simples se necessário." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Now you’ve got the experience to make basic repairs and can design and install simple biotech machines.", description_pt: "Agora você tem a experiência para fazer reparos básicos e pode projetar e instalar máquinas biotecnológicas simples." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Honed by study, field work, and laboratory experimentation, you’ve got serious chops with regards to cybernetic technology.", description_pt: "Aprimorado por estudo, trabalho de campo e experimentação laboratorial, você tem grande habilidade com tecnologia cibernética." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "You’re good enough to create, install, and maintain radical designs, perform major overhauls on existing tech, and employ almost any biotech Device or Procedure within your field. (Weird stuff from those demented Etherites is probably beyond your grasp… but really, does anyone truly understand those mad bastards?)", description_pt: "Você é bom o bastante para criar, instalar e manter projetos radicais, realizar revisões completas em tecnologias existentes e empregar quase qualquer Dispositivo ou Procedimento biotecnológico em sua área. (Invenções bizarras daqueles eteritas dementes provavelmente fogem ao seu alcance... mas afinal, alguém realmente entende aqueles bastardos loucos?)" },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "You could compare notes with Tony Stark.", description_pt: "Você poderia trocar anotações com Tony Stark." },
    ],
    possessed_by: "Cyborgs, Cybernetic Maintenance Techs, Q Division, Genegineers, Mad Scientists, Transhumanists.",
    possessed_by_pt: "Ciborgues, Técnicos de Manutenção Cibernética, Divisão Q, Engenheiros Genéticos, Cientistas Loucos, Transumanistas.",
    suggested_specialties: &["Cloning", "Cybernetics", "Mad Theories", "Repair", "Field Mods", "Redesign", "Nanotech", "Floronics", "Biomechanisms", "Biomods", "Web-Based Interface"],
    suggested_specialties_pt: &["Clonagem", "Cibernética", "Teorias Insanas", "Reparo", "Modificações de Campo", "Redesenho", "Nanotecnologia", "Florônica", "Biomecanismos", "Biomodificações", "Interface Baseada na Rede"],
};

pub const ENERGY_WEAPONS: AbilityDefinition = AbilityDefinition {
    id: "energy_weapons",
    name: "Energy Weapons",
    name_pt: "Armas de Energia",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 296-297",
    description: "Operating, aiming, recharging, and maintaining Directed Energy Weapons: laser carbines, plasma projectors, particle blasters, and lightning cannons.",
    description_pt: "Operar, mirar, recarregar e manter Armas de Energia Direcionada: carabinas laser, projetores de plasma, blasters de partículas e canhões elétricos.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Imperial Stormtrooper.", description_pt: "Stormtrooper Imperial." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Competent shooter.", description_pt: "Atirador competente." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Average Technocratic field operative.", description_pt: "Operativo de campo comum da Tecnocracia." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Elite Op.", description_pt: "Operativo de elite." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "“Phased plasma rifle in the 40-Watt range…”", description_pt: "“Rifle de plasma em fase na faixa de 40 watts…”" },
    ],
    possessed_by: "Cyborgs, Space Marines, Black Suits, Covert Agents, Etherite Heroes.",
    possessed_by_pt: "Ciborgues, Fuzileiros Espaciais, Ternos Pretos, Agentes Secretos, Heróis Eteritas.",
    suggested_specialties: &["Heavy Weapons", "Experimental Tech", "Repair", "Small Arms", "Field Modifications", "Cybernetic Weaponry", "Micro-Gravitational Operations", "Sniper", "Pulp-Tech"],
    suggested_specialties_pt: &["Armas Pesadas", "Tecnologia Experimental", "Reparo", "Armas Leves", "Modificações de Campo", "Armamento Cibernético", "Operações em Microgravidade", "Atirador de Elite", "Tecnologia Pulp"],
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
    page_ref: "M20, p. 298",
    description: "Intimate local familiarity with a specific city, district, or wilderness zone: shortcuts, hideouts, local legends, power brokers, and hidden escape routes.",
    description_pt: "Familiaridade íntima com uma cidade, bairro ou região selvagem específica: atalhos, esconderijos, lendas locais, pessoas influentes e rotas de fuga ocultas.",
    ratings: &[
        AbilityRating { dots: 1, title: "Dabbler", title_pt: "Curioso", description: "You’ve studied up on the area in question.", description_pt: "Você estudou sobre a área em questão." },
        AbilityRating { dots: 2, title: "Student", title_pt: "Estudante", description: "You’ve lived there a while.", description_pt: "Você morou lá por algum tempo." },
        AbilityRating { dots: 3, title: "Scholar", title_pt: "Erudito", description: "You’ve spent lots of time getting to know the local secrets.", description_pt: "Você passou muito tempo conhecendo os segredos locais." },
        AbilityRating { dots: 4, title: "Professor", title_pt: "Professor", description: "There’s not much about the place you don’t know.", description_pt: "Há pouca coisa sobre o lugar que você não saiba." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "“My city calls me, and I answer…”", description_pt: "“Minha cidade me chama, e eu respondo…”" },
    ],
    possessed_by: "Vigilantes, Cops, Crime Bosses, Journalists, Backwoods Legends, Street Folks, Black Suits, Protectors of the People.",
    possessed_by_pt: "Vigilantes, Policiais, Chefes do Crime, Jornalistas, Lendas do Interior, Pessoas das Ruas, Ternos Pretos, Protetores do Povo.",
    suggested_specialties: &["History", "Geography", "Wildlife", "Politics", "Shortcuts", "Getaways", "Crime", "Rumors", "Hideouts", "Gangs", "Information Sources", "Well-Hidden Secrets"],
    suggested_specialties_pt: &["História", "Geografia", "Vida Selvagem", "Política", "Atalhos", "Fugas", "Crime", "Boatos", "Esconderijos", "Gangues", "Fontes de Informação", "Segredos Bem Ocultos"],
};

pub const BELIEF_SYSTEMS: AbilityDefinition = AbilityDefinition {
    id: "belief_systems",
    name: "Belief Systems",
    name_pt: "Sistemas de Crença",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 298-299",
    description: "Study of world religions, cultural worldviews, ideologies, and local dogmas. Essential for crafting coincidental magick aligned with local Consensus reality zones.",
    description_pt: "Estudo das religiões mundiais, visões culturais, ideologias e dogmas locais. Crucial para formular mágika coincidente alinhada às Zonas de Realidade do Consenso.",
    ratings: &[
        AbilityRating { dots: 1, title: "Dabbler", title_pt: "Curioso", description: "You’ve gleaned a working knowledge off Wikipedia.", description_pt: "Você colheu um conhecimento prático pela Wikipédia." },
        AbilityRating { dots: 2, title: "Student", title_pt: "Estudante", description: "You hold a decent understanding of current religious and cultural beliefs.", description_pt: "Você possui uma compreensão razoável das crenças religiosas e culturais atuais." },
        AbilityRating { dots: 3, title: "Scholar", title_pt: "Erudito", description: "You’ve got a deep comprehension of dominant belief systems worldwide.", description_pt: "Você possui uma compreensão profunda dos sistemas de crença dominantes no mundo." },
        AbilityRating { dots: 4, title: "Professor", title_pt: "Professor", description: "You’re a noted authority on comparative religions and political philosophies.", description_pt: "Você é uma autoridade notória em religiões comparadas e filosofias políticas." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Karen Armstrong fact-checks her books with you.", description_pt: "Karen Armstrong confere os fatos dos livros dela com você." },
    ],
    possessed_by: "Journalists, Missionaries, Theologians, Historians, Professors, Pundits, Philosophers, Poli-Sci Scholars, Media Manipulators, Political/ Religious Activists, Paradigm-Shifting Mages.",
    possessed_by_pt: "Jornalistas, Missionários, Teólogos, Historiadores, Professores, Comentaristas, Filósofos, Eruditos de Ciência Política, Manipuladores de Mídia, Ativistas Políticos/Religiosos, Magos que Mudam Paradigmas.",
    suggested_specialties: &["Regional Beliefs", "Esoterica", "Cults", "Current Events", "Religious/ Political History", "Geopolitics", "Comparative Religion", "Media Manipulation", "Belief Trends", "Paradigm-Hacking"],
    suggested_specialties_pt: &["Crenças Regionais", "Esotérica", "Seitas", "Acontecimentos Atuais", "História Religiosa/Política", "Geopolítica", "Religião Comparada", "Manipulação de Mídia", "Tendências de Crença", "Hacking de Paradigma"],
};

pub const CRYPTOGRAPHY: AbilityDefinition = AbilityDefinition {
    id: "cryptography",
    name: "Cryptography",
    name_pt: "Criptografia",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 299",
    description: "The mathematical science of data encryption, key generation, steganography, digital hashing, and cracking top-secret ciphers.",
    description_pt: "A ciência matemática de criptografia de dados, geração de chaves, esteganografia, hashes digitais e decifração de cifras ultrassecretas.",
    ratings: &[
        AbilityRating { dots: 1, title: "Dabbler", title_pt: "Curioso", description: "Spy novel fan.", description_pt: "Fã de romances de espionagem." },
        AbilityRating { dots: 2, title: "Student", title_pt: "Estudante", description: "Military signals engineer.", description_pt: "Engenheiro de sinais militares." },
        AbilityRating { dots: 3, title: "Scholar", title_pt: "Erudito", description: "Intelligence operative.", description_pt: "Operativo de inteligência." },
        AbilityRating { dots: 4, title: "Professor", title_pt: "Professor", description: "Cipher specialist.", description_pt: "Especialista em cifras." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Codebuster supreme.", description_pt: "Decifrador supremo de códigos." },
    ],
    possessed_by: "Puzzle Fans, Intelligence Specialists, Spies, Alchemists, Communication Analysts, Insurgents, Counterterrorism Agents.",
    possessed_by_pt: "Fãs de Quebra-Cabeças, Especialistas em Inteligência, Espiões, Alquimistas, Analistas de Comunicação, Insurgentes, Agentes Antiterrorismo.",
    suggested_specialties: &["Mathematical Encryption", "Secret Languages", "Mystical Codes", "Occulted Lore", "Encryption History", "Weird Logic", "Computers", "Art-History Symbolism"],
    suggested_specialties_pt: &["Criptografia Matemática", "Línguas Secretas", "Códigos Místicos", "Conhecimento Ocultado", "História da Criptografia", "Lógica Bizarra", "Computadores", "Simbolismo da História da Arte"],
};

pub const DEMOLITIONS: AbilityDefinition = AbilityDefinition {
    id: "demolitions",
    name: "Demolitions",
    name_pt: "Demolições",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 299",
    description: "Handling, assembling, defusing, and detonating explosives: C4, dynamite, blasting caps, shape charges, and controlled structural demolition.",
    description_pt: "Manusear, montar, desarmar e detonar explosivos: C4, dinamite, espoletas, cargas moldadas e demolição estrutural controlada.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Model kits, fireworks, and you.", description_pt: "Kits de plastimodelismo, fogos de artifício e você." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Anarchy!", description_pt: "Anarquia!" },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "You’ve trained with explosive compounds, devices, and precautions.", description_pt: "Você treinou com compostos explosivos, dispositivos e precauções." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "“Stand back — I’m a professional.”", description_pt: "“Afastem-se — sou um profissional.”" },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "A master of disasters, you can defuse, as well as create, a wide range of explosive devices.", description_pt: "Um mestre dos desastres, você pode desarmar, assim como criar, uma ampla variedade de dispositivos explosivos." },
    ],
    possessed_by: "Mythbusters, Terrorists, Counterterrorists, Radicals, Survivalists, Spies, Pyrotechnicians, Military Specialists, YouTube Stars, Kids With More Enthusiasm Than Sense.",
    possessed_by_pt: "Caçadores de Mitos (Mythbusters), Terroristas, Agentes Antiterrorismo, Radicais, Sobrevivencialistas, Espiões, Pirotécnicos, Especialistas Militares, Astros do YouTube, Jovens com Mais Entusiasmo do que Juízo.",
    suggested_specialties: &["IEDs", "Bomb Disposal", "Stage Effects", "Fireworks", "Car Bombs", "Improvised Materials", "Magickal Enhancement", "Building Demolition", "“Watch This!”"],
    suggested_specialties_pt: &["Dispositivos Explosivos Improvisados (IEDs)", "Desarme de Bombas", "Efeitos de Palco", "Fogos de Artifício", "Carros-Bomba", "Materiais Improvisados", "Aprimoramento Mágico", "Demolição de Edifícios", "“Olha Isso!”"],
};

// ============================================================================
// HABILIDADES SECUNDÁRIAS ADICIONADAS (M20 pp. 293-300)
// ============================================================================

pub const FLYING: AbilityDefinition = AbilityDefinition {
    id: "flying",
    name: "Flying",
    name_pt: "Voo",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 293",
    description: "People don’t usually fly. You do. Whether it’s because you’ve got (or can grow) wings and you know how to use them or else you’re simply practiced in the fine art of aerial maneuvers, you’re at home in the air as well as on the ground.",
    description_pt: "Pessoas normalmente não voam. Você voa. Seja porque tem (ou pode criar) asas e sabe usá-las, seja porque domina a arte das manobras aéreas, você se sente em casa tanto no ar quanto no chão.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Suicide case.", description_pt: "Suicida." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Awkward angel.", description_pt: "Anjo desajeitado." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Quidditch player.", description_pt: "Jogador de Quadribol." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Winged avenger.", description_pt: "Vingador alado." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Wind-dancer.", description_pt: "Dançarino do vento." },
    ],
    possessed_by: "Witches, Shapeshifters, Bird-Folk, Angelic or Demonic Entities, Magic Carpet Riders, Arcane Athletes, Crouching Tigers and Hidden Dragons.",
    possessed_by_pt: "Bruxas, Transmorfos, Povo-Pássaro, Entidades Angelicais ou Demoníacas, Pilotos de Tapete Mágico, Atletas Arcanos, Tigres Agachados e Dragões Escondidos.",
    suggested_specialties: &["Broomsticks", "Sports", "Aerial Combat", "Hunting", "Strafing", "Carrying Passengers", "Chi-Based Levitation"],
    suggested_specialties_pt: &["Vassouras", "Esportes", "Combate Aéreo", "Caça", "Rasantes", "Carregar Passageiros", "Levitação pelo Chi"],
};

pub const HIGH_RITUAL: AbilityDefinition = AbilityDefinition {
    id: "high_ritual",
    name: "High Ritual",
    name_pt: "Alto Ritual",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 293-294",
    description: "There’s both an art and a science to a good rite. You’ve studied not only the techniques and trappings of ritual practices but also the symbolic, psychological, and metaphysical principles behind the big show. In general, this Talent reflects a familiarity with rituals appropriate to your spiritual or magickal pursuits; a Catholic priest understands the various Church ceremonies, whereas an Eleusinian priestess knows her way around Greek mystery rites. Beyond that knowledge, though, you understand not only the rituals of your chosen practice but also possess a gift for staging rituals in general.",
    description_pt: "Há tanto arte quanto ciência em um bom rito. Você estudou não só as técnicas e os adereços das práticas rituais, mas também os princípios simbólicos, psicológicos e metafísicos por trás do grande espetáculo, e tem o dom de encenar rituais em geral.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "You could plan a wedding, call the corners, or recite the Stations of the Cross.", description_pt: "Você conseguiria planejar um casamento, invocar os quadrantes ou recitar a Via-Sacra." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Your parties and rites earn respect from your community.", description_pt: "Suas festas e ritos ganham o respeito da sua comunidade." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Beyond your familiarity with specific rituals, you’ve begun learning about other practices as well.", description_pt: "Além dos rituais que conhece, você começou a aprender outras práticas." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "An acclaimed priest or priestess, you’ve got a knack for powerful ceremonies and a broad understanding of specific ritual techniques.", description_pt: "Sacerdote ou sacerdotisa aclamado, com talento para cerimônias poderosas e amplo domínio de técnicas rituais." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "You’ve mastered not only the rites of your culture but many ritual practices of other cultures too. So potent is your knowledge that you could make atheists into believers.", description_pt: "Você domina os ritos da sua cultura e de muitas outras; seu conhecimento poderia transformar ateus em fiéis." },
    ],
    possessed_by: "Clergy, Occultists, Entertainers, High Magicians, Showboating Frauds, Comparative Religion Specialists, Ritual Mages, Citizen Evangelists, Party Planners, Church or Funeral Directors.",
    possessed_by_pt: "Clérigos, Ocultistas, Artistas, Altos Magos, Charlatães Exibidos, Especialistas em Religião Comparada, Magos Ritualistas, Evangelistas Leigos, Organizadores de Festas, Diretores de Igreja ou Funerária.",
    suggested_specialties: &["High Mass", "Obscure Rituals", "Spontaneous Rituals", "Secular Ceremonies", "Music Concerts", "Impossible Miracles", "specific cultural rites (Jewish, Catholic, Satanic, Voudoun, etc.)"],
    suggested_specialties_pt: &["Missa Solene", "Rituais Obscuros", "Rituais Espontâneos", "Cerimônias Seculares", "Shows de Música", "Milagres Impossíveis", "ritos culturais específicos (Judaico, Católico, Satânico, Vodu etc.)"],
};

pub const LUCID_DREAMING: AbilityDefinition = AbilityDefinition {
    id: "lucid_dreaming",
    name: "Lucid Dreaming",
    name_pt: "Sonho Lúcido",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 294",
    description: "Most folks float through dreams with no sense of control. Not you. When you choose to assert yourself in the dreamlands, you may shift the tone of a dream, change locations, confront nightmares head-on, or even construct dreams of your choosing. Cosmologically, you know how to tap into the essence of Maya and turn it in your favor. With practice, you might create a Demesne Background (see below) that suits your temperament. In game terms, this Talent lets you manipulate your surroundings in a dream (Perception + Lucid Dreaming, difficulty 6); shape new structures (difficulty 8); or take complete control over that dream and your place in it (difficulty 9). You can use this Talent to direct conscious meditations, too (difficulties 5, 7, and 8, respectively), which can help you gain insights, unravel riddles, and achieve a state of calm or resolution. If you’re meditating or dreaming about a particular enigma, then your successes — after you wake up — count as if they were successes on an Enigmas roll. Jennifer Rollins, as an example, meditates about a puzzle she wants to solve, and so her successes with a Lucid Dreaming roll count as successes used to solve that puzzle; once she leaves the trance, she understands how those pieces fit together. For the Demesne Background, see that entry in this chapter, (pp. 310-311). For game systems involved in riddles, puzzles, and enigmas, see Art and Science and Social Occasions and Intrigue in Chapter Nine, (p. 404).",
    description_pt: "A maioria das pessoas atravessa os sonhos sem nenhum controle. Você não. Quando decide se impor no mundo dos sonhos, pode mudar o tom de um sonho, trocar de lugar, enfrentar pesadelos ou até construir os sonhos que quiser.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "You’re aware of your dreams and can remember them if you choose.", description_pt: "Você tem consciência dos seus sonhos e consegue lembrá-los se quiser." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "You can try to dream a certain dream.", description_pt: "Você consegue tentar sonhar um sonho específico." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Some control over your dreams.", description_pt: "Algum controle sobre os seus sonhos." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "You know how to influence and read dreams as aspects of your inner self.", description_pt: "Você sabe influenciar e interpretar sonhos como aspectos do seu eu interior." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "You understand the metaphysical elements of dreams and can manipulate them, recall them, inspire them, or share them to your will.", description_pt: "Você entende os elementos metafísicos dos sonhos e pode manipulá-los, lembrá-los, inspirá-los ou compartilhá-los à vontade." },
    ],
    possessed_by: "Shamans, Sages, Witches, Artists, Savants, Philosophers, Dream Researchers, Marauders, Martial Artists, Fey-Blooded People, The Dream Police.",
    possessed_by_pt: "Xamãs, Sábios, Bruxas, Artistas, Eruditos, Filósofos, Pesquisadores do Sono, Marauders, Artistas Marciais, Pessoas com Sangue Feérico, A Polícia dos Sonhos.",
    suggested_specialties: &["Dream Control", "Waking From Nightmares", "Puzzling Enigmas", "Visionquesting", "Omens", "Dream Combat", "Demesne"],
    suggested_specialties_pt: &["Controle Onírico", "Despertar de Pesadelos", "Decifrar Enigmas", "Busca de Visão", "Presságios", "Combate Onírico", "Domínio (Demesne)"],
};

pub const SEARCH: AbilityDefinition = AbilityDefinition {
    id: "search",
    name: "Search",
    name_pt: "Busca",
    category: AbilityCategory::Talents,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 294-295",
    description: "Given time and motivation, you know how to toss a room or comb the woods. You’ve honed an eye for clues, mastered the classic hiding spaces (and puzzled out some new ones), and learned how to follow your instincts. If there’s something to be found, you’re likely to find it… and if you combine this Talent with Sphere 1 Effects, there’s not much that can hide from you for long.",
    description_pt: "Com tempo e motivação, você sabe revirar um cômodo ou vasculhar uma mata. Tem olho para pistas, conhece os esconderijos clássicos (e descobriu alguns novos) e sabe seguir os próprios instintos.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "You’re good at finding lost stuff.", description_pt: "Você é bom em achar coisas perdidas." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Clues seem to jump out at you.", description_pt: "As pistas parecem saltar aos seus olhos." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "The last place to look is the first place you try.", description_pt: "O último lugar onde procurar é o primeiro que você tenta." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "The pros consult you when something or someone needs to be found.", description_pt: "Os profissionais consultam você quando algo ou alguém precisa ser encontrado." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Your gift for detections seems downright supernatural… and probably is.", description_pt: "Seu dom para encontrar coisas parece sobrenatural... e provavelmente é." },
    ],
    possessed_by: "Burglars, Shakedown Experts, Detectives, Inspectors, Profilers, Bounty Hunters, Black Suits, CSI Teams.",
    possessed_by_pt: "Arrombadores, Especialistas em Revista, Detetives, Inspetores, Analistas de Perfil, Caçadores de Recompensa, Homens de Preto, Equipes de Perícia.",
    suggested_specialties: &["Tossing", "Hidden Spaces", "Likely Places", "Outdoor Cues", "Police Procedures", "Concealed Doors", "Telltale Marks"],
    suggested_specialties_pt: &["Revistas", "Espaços Ocultos", "Lugares Prováveis", "Pistas ao Ar Livre", "Procedimentos Policiais", "Portas Ocultas", "Marcas Reveladoras"],
};

pub const HYPERTECH: AbilityDefinition = AbilityDefinition {
    id: "hypertech",
    name: "Hypertech",
    name_pt: "Hipertecnologia",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 297",
    description: "There’s mundane technology, and then there’s the stuff that technomages command. This latter tech is not, of course, “magick” — perish that thought! Still, its principles transcend the limited understanding of the Masses and their convenient little world. The advanced principles employed by the Technocracy, Virtual Adepts, various mad scientists, and even the technomancers of other, more traditional pursuits all range outside everyday sciences. These arcane technologies — nanotech, Trinary computing, extradimensional travel, and so on — seem mysterious even to many of the rank-and-file who employ them. You, however, understand how such things work… and through such understanding, you can modify existing Devices, create your own, or employ machines created by other technomancer mages.",
    description_pt: "Existe a tecnologia mundana e existe aquela que os tecnomagos comandam. Você entende como funcionam as tecnologias arcanas (nanotecnologia, computação trinária, viagem extradimensional) e pode modificar Dispositivos, criar os seus ou usar máquinas criadas por outros tecnomantes.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "You understand basic theories of alternative sciences.", description_pt: "Você entende as teorias básicas das ciências alternativas." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "“Esoterica” is just another word for the obvious.", description_pt: "“Esotérica” é só outra palavra para o óbvio." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "You can wrap your head around the technological perversions of other groups.", description_pt: "Você consegue compreender as perversões tecnológicas de outros grupos." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Technologies of your foes and rivals dance to the tune you program for them.", description_pt: "As tecnologias de inimigos e rivais dançam conforme a música que você programa." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "So rich is your comprehension of the universe that few technologies elude your command.", description_pt: "Sua compreensão do universo é tão vasta que poucas tecnologias escapam ao seu comando." },
    ],
    possessed_by: "Mad Scientists, Renegade Technocrats, Visionary Geniuses, Tech-Hackers, Q Division Ops, Design Specialists, Elite Field Ops, Black Suits, Gadgeteers.",
    possessed_by_pt: "Cientistas Loucos, Tecnocratas Renegados, Gênios Visionários, Hackers de Tecnologia, Agentes da Divisão Q, Especialistas em Design, Agentes de Elite, Homens de Preto, Inventores de Engenhocas.",
    suggested_specialties: &["Hacking-n-Cracking", "Improvisation", "Repair", "Jury-Rigging", "Radical Conversions", "Weaponsmith", "Computers", "Microtech", "Nanotech", "Experimental Theories", "Ethertech Theories", "Street Mods", "Anarchtech"],
    suggested_specialties_pt: &["Invasão e Quebra de Sistemas", "Improvisação", "Reparos", "Gambiarras", "Conversões Radicais", "Armeiro", "Computadores", "Microtecnologia", "Nanotecnologia", "Teorias Experimentais", "Teorias Etéricas", "Modificações de Rua", "Anarcotecnologia"],
};

pub const JETPACK: AbilityDefinition = AbilityDefinition {
    id: "jetpack",
    name: "Jetpack",
    name_pt: "Mochila a Jato",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 297",
    description: "The tricky art of personal flight devices is yours to enjoy. With this Skill, you know how to control body-mounted flying gadgets. Like most tech-based Abilities, your success depends a lot on your familiarity, or lack of it, with the specific kind of tech involved. A Man in Black would have as hard a time with a Son of Ether’s jetpack as an Etherite would have with standard-issue NWO gear. Once you’ve logged some flight-time with a device, however, you’re on your way.",
    description_pt: "A arte delicada dos dispositivos de voo pessoal é sua. Com esta Perícia, você sabe controlar engenhocas voadoras presas ao corpo; o sucesso depende muito da sua familiaridade com o tipo específico de tecnologia.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "“Oooooooooh, shiiiiiiiiiiiiiiittttttt!”", description_pt: "“Aaaaaai, meeeeeeeeerdaaaaaa!”" },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "“That’s not flying — that’s falling with style.”", description_pt: "“Isso não é voar, é cair com estilo.”" },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Experienced and controlled.", description_pt: "Experiente e controlado." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "You can fly, fight, and return to earth intact.", description_pt: "Você consegue voar, lutar e voltar ao chão inteiro." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "“To Infinity — and BEYOND!”", description_pt: "“Ao infinito e além!”" },
    ],
    possessed_by: "Space Marines, Etherites, Daredevils, Commandos, Black Suits, Research Assistants, Test Pilots.",
    possessed_by_pt: "Fuzileiros Espaciais, Eteritas, Destemidos, Comandos, Homens de Preto, Assistentes de Pesquisa, Pilotos de Teste.",
    suggested_specialties: &["Combat", "Tricks", "Pursuit", "High Speeds", "Boot-Jets", "Micro-G"],
    suggested_specialties_pt: &["Combate", "Acrobacias", "Perseguição", "Altas Velocidades", "Botas a Jato", "Microgravidade"],
};

pub const RIDING: AbilityDefinition = AbilityDefinition {
    id: "riding",
    name: "Riding",
    name_pt: "Montaria",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 297",
    description: "You know how to saddle, care for, and ride some sort of animal. Through a combination of techniques and bonding, you stand a decent chance of getting where you want to go without being thrown off, trampled, or possibly eaten by your ride. Certain beasts, of course, are harder to ride than others. For truly dangerous or exotic mounts (tigers, griffins, giant owls, etc.), you’ll probably need Riding 3 or better in order to stay in the saddle at all. Under challenging circumstances (combat, hard gallops, stunts, flying, and so on), you may need to make a Dexterity + Riding roll. Charisma + Riding helps you bond with your mount, and Intelligence + Riding lets you know how to take care of it, maintain riding gear, and puzzle out someone else’s riding techniques. To reflect an unusually keen bond with animals, see the Animal Kinship Talent, (p. 289).",
    description_pt: "Você sabe selar, cuidar e montar algum tipo de animal. Com técnica e vínculo, tem boas chances de chegar aonde quer sem ser derrubado, pisoteado ou devorado pela montaria.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Dude-ranch tourist.", description_pt: "Turista de hotel-fazenda." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Practiced equestrian.", description_pt: "Cavaleiro praticante." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Professional rider.", description_pt: "Montador profissional." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Bronco-buster.", description_pt: "Domador de potros." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Ride the tiger, bay-beee!", description_pt: "Monte o tigre, beibe!" },
    ],
    possessed_by: "Cowboys, Medievalists, Athletes, Stable Grooms, Wealthy Folks, Farmers, Pre-Industrial Travelers.",
    possessed_by_pt: "Caubóis, Medievalistas, Atletas, Cavalariços, Pessoas Abastadas, Fazendeiros, Viajantes Pré-Industriais.",
    suggested_specialties: &["Bareback", "Stunts", "Horse Whispering", "Traditional Techniques", "Showmanship", "Jumping", "Combat", "Jousting", "Racing", "Weird Mounts", "Rapport"],
    suggested_specialties_pt: &["Montaria em Pelo", "Acrobacias", "Encantador de Cavalos", "Técnicas Tradicionais", "Exibição", "Saltos", "Combate", "Justa", "Corrida", "Montarias Estranhas", "Afinidade"],
};

pub const TORTURE: AbilityDefinition = AbilityDefinition {
    id: "torture",
    name: "Torture",
    name_pt: "Tortura",
    category: AbilityCategory::Skills,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 298",
    description: "Call it “enhanced interrogation” if you like. We all know what you really do: You hurt people. Maybe you’re “just following orders,” or you realize that ticking bombs sometimes require blunt instruments. Perhaps you just plain enjoy it, even if you can’t even admit that to yourself. Whatever means you use to justify this skill, you excel at causing mental, physical, and perhaps spiritual distress. This might be part of a religious duty, military service, mystic practice, or god-given prerogative. Then again, you could simply be a sick fuck. Whether said distress involves brutal trauma, exquisite agony, or “simple” emotional manipulation, your skills bring people — perhaps even yourself — to, and perhaps over, the edge of what was once considered endurance.",
    description_pt: "Chame de “interrogatório aprimorado” se quiser. Todos sabemos o que você realmente faz: machuca pessoas. Seja qual for a justificativa, você é excelente em causar sofrimento mental, físico e talvez espiritual. Cenas de tortura são conteúdo adulto e devem ser tratadas com cuidado e respeito pelos limites dos jogadores.",
    ratings: &[
        AbilityRating { dots: 1, title: "Novice", title_pt: "Novato", description: "Playground bully.", description_pt: "Valentão do recreio." },
        AbilityRating { dots: 2, title: "Practiced", title_pt: "Praticante", description: "Bad cop.", description_pt: "Policial mau." },
        AbilityRating { dots: 3, title: "Skillful", title_pt: "Competente", description: "Covert ops interrogator.", description_pt: "Interrogador de operações secretas." },
        AbilityRating { dots: 4, title: "Expert", title_pt: "Especialista", description: "Elite pro dom.", description_pt: "Dominador profissional de elite." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Nazi doctor, Spanish Inquisitor, or serial killer.", description_pt: "Médico nazista, Inquisidor espanhol ou assassino em série." },
    ],
    possessed_by: "Enforcers, Gangsters, Spies, Counter-Terrorism Agents, Left-Hand Path Ecstatic Mages and Euthanatos, NWO Operatives, Scary Doctors, Sadists (Safe, Sane, and Otherwise).",
    possessed_by_pt: "Executores, Gângsteres, Espiões, Agentes Antiterrorismo, Extáticos e Euthanatos da Senda da Mão Esquerda, Agentes da NWO, Médicos Assustadores, Sádicos (Seguros, Sãos e Outros).",
    suggested_specialties: &["Mind Games", "Exquisite Techniques", "Prolonged Agony", "Classical Devices", "Sexplay", "Self-Torment", "Information Retrieval", "“Works of Art”", "Pain — Not Harm"],
    suggested_specialties_pt: &["Jogos Mentais", "Técnicas Requintadas", "Agonia Prolongada", "Instrumentos Clássicos", "Jogos Sexuais", "Autotormento", "Extração de Informações", "“Obras de Arte”", "Dor, Não Dano"],
};

pub const FINANCE: AbilityDefinition = AbilityDefinition {
    id: "finance",
    name: "Finance",
    name_pt: "Finanças",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 299",
    description: "Money is the ultimate human magic trick, and you know how it works. Trade, trends, investments, markets, and risks provide you with mental stimulation, Awakened/ Enlightened power, economic fortitude, and — most likely — plenty of money (in game terms, the Resources Background).",
    description_pt: "O dinheiro é o truque de mágica humano definitivo, e você sabe como ele funciona. Comércio, tendências, investimentos, mercados e riscos lhe dão estímulo mental, poder Desperto ou Iluminado, solidez econômica e, provavelmente, muito dinheiro (em termos de jogo, o Antecedente Recursos).",
    ratings: &[
        AbilityRating { dots: 1, title: "Dabbler", title_pt: "Curioso", description: "Smart investor.", description_pt: "Investidor esperto." },
        AbilityRating { dots: 2, title: "Student", title_pt: "Estudante", description: "Bank employee.", description_pt: "Funcionário de banco." },
        AbilityRating { dots: 3, title: "Scholar", title_pt: "Erudito", description: "Professional stockbroker.", description_pt: "Corretor de ações profissional." },
        AbilityRating { dots: 4, title: "Professor", title_pt: "Professor", description: "Wall Street wizard.", description_pt: "Mago de Wall Street." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Power behind the throne.", description_pt: "O poder por trás do trono." },
    ],
    possessed_by: "Bankers, Brokers, Tycoons, Executives, Reporters, Accountants, Investors, Wealthy Bastards, Nephandic Corrupters, Captains of Industry, Masters of the Modern World.",
    possessed_by_pt: "Banqueiros, Corretores, Magnatas, Executivos, Repórteres, Contadores, Investidores, Ricaços Desgraçados, Corruptores Nefandos, Capitães da Indústria, Senhores do Mundo Moderno.",
    suggested_specialties: &["Stock Markets", "Investment Trends", "International Trading", "Warning Signs", "Insider Trading", "Laws", "Law-Breaking", "Black Markets", "Currency", "Power of Greed", "Ars Cupiditae"],
    suggested_specialties_pt: &["Bolsas de Valores", "Tendências de Investimento", "Comércio Internacional", "Sinais de Alerta", "Informação Privilegiada", "Legislação", "Burlar a Lei", "Mercados Negros", "Câmbio", "Poder da Ganância", "Ars Cupiditae"],
};

pub const LORE: AbilityDefinition = AbilityDefinition {
    id: "lore",
    name: "Lore / RD Data",
    name_pt: "Saber / Dados RD",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 299-300",
    description: "You know more about the Night-Folk than they’d appreciate you knowing. Sure, your understanding gets filtered through the lens of your own group’s perspective, but it’s more than the average person… or mage… comprehends. For mystic mages, such knowledge is considered Lore: the hidden secrets of vampires, werecreatures, and the like. Among Technocrats, it’s known as RD Data: information about the clandestine affairs of Reality Deviants whose very existence presents a threat to the Consensus.",
    description_pt: "Você sabe mais sobre o Povo da Noite do que eles gostariam. Para magos místicos, esse conhecimento é o Saber (Lore): os segredos de vampiros, lobisomens e afins. Entre Tecnocratas, são os Dados RD: informações sobre Desviantes da Realidade cuja existência ameaça o Consenso.",
    ratings: &[
        AbilityRating { dots: 1, title: "Dabbler", title_pt: "Curioso", description: "They’re out there…", description_pt: "Eles estão por aí..." },
        AbilityRating { dots: 2, title: "Student", title_pt: "Estudante", description: "You’ve heard a few names and legends about them.", description_pt: "Você ouviu alguns nomes e lendas sobre eles." },
        AbilityRating { dots: 3, title: "Scholar", title_pt: "Erudito", description: "These entities have distinct social structures, and you’ve begun to understand what they might be.", description_pt: "Essas entidades têm estruturas sociais próprias, e você começa a entender o que elas podem ser." },
        AbilityRating { dots: 4, title: "Professor", title_pt: "Professor", description: "Although your knowledge is biased by your own perspective, you can tell a Ventrue from a Setite and you’ve got some idea what that distinction means.", description_pt: "Mesmo com a visão enviesada, você distingue um Ventrue de um Setita e tem ideia do que isso significa." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "You’re not supposed to know as much about them as you do… but you do.", description_pt: "Você não deveria saber tanto sobre eles... mas sabe." },
    ],
    possessed_by: "Sages, Wizards, Xenobiologists, Black Suits, Witches, Shamans, Field Ops, Adventurers, Emissaries, Strategists, Interrogators, Intelligence Analysts, Monster-Hunters.",
    possessed_by_pt: "Sábios, Magos, Xenobiólogos, Homens de Preto, Bruxas, Xamãs, Agentes de Campo, Aventureiros, Emissários, Estrategistas, Interrogadores, Analistas de Inteligência, Caçadores de Monstros.",
    suggested_specialties: &["Infernalists", "Marauders", "Traditions", "Technocracy", "Disparates", "Faeries", "Vampires", "Werebeasts", "Ghosts", "Hunters", "Demons", "etc"],
    suggested_specialties_pt: &["Infernalistas", "Marauders", "Tradições", "Tecnocracia", "Ofícios Dissidentes", "Fadas", "Vampiros", "Metamorfos", "Fantasmas", "Caçadores", "Demônios", "etc."],
};

pub const MEDIA: AbilityDefinition = AbilityDefinition {
    id: "media",
    name: "Media",
    name_pt: "Mídia",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Secondary,
    page_ref: "M20, pp. 300-301",
    description: "Mass media is mass reality… and you know how to shape both. The fine art and science of crafting public perception is yours to employ. You know who to call, what to tell them, which strings to pull, and probably how to make a tasty profit off the end result. Although this Trait does not necessarily grant access to high-placed media figures (see the Influence Background for those kinds of connections), you understand the Mass Media Machine and can guide it to your best advantage. The Dominion, Art of Desire, and Reality-Hacking practices often depend upon an understanding of media (see those entries in Chapter Ten), and media can even function as an instrument of certain types of magick — see the Mass Media entry on (p. 594) for details.",
    description_pt: "Mídia de massa é realidade de massa, e você sabe moldar as duas. Você sabe a quem ligar, o que dizer e quais cordas puxar para conduzir a Máquina da Mídia a seu favor, e provavelmente como lucrar com o resultado.",
    ratings: &[
        AbilityRating { dots: 1, title: "Dabbler", title_pt: "Curioso", description: "Mass-Com major.", description_pt: "Estudante de Comunicação." },
        AbilityRating { dots: 2, title: "Student", title_pt: "Estudante", description: "Reporter.", description_pt: "Repórter." },
        AbilityRating { dots: 3, title: "Scholar", title_pt: "Erudito", description: "Career journalist.", description_pt: "Jornalista de carreira." },
        AbilityRating { dots: 4, title: "Professor", title_pt: "Professor", description: "Network honcho.", description_pt: "Chefão de emissora." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Rupert Murdoch.", description_pt: "Rupert Murdoch." },
    ],
    possessed_by: "Writers, Editors, Pundits, Media Flacks, Spin Doctors, Watchdog Advocates, Bloggers, Advertisers, Celebrities, Corrupters, Media Agents, Politicians, Activists, Adbusters, NWO Watchers, Media-Indoctrination Specialists.",
    possessed_by_pt: "Escritores, Editores, Comentaristas, Assessores de Imprensa, Marqueteiros, Defensores da Transparência, Blogueiros, Publicitários, Celebridades, Corruptores, Agentes de Mídia, Políticos, Ativistas, Adbusters, Vigilantes da NWO, Especialistas em Doutrinação Midiática.",
    suggested_specialties: &["Spin Control", "Politics", "Leverage", "Slander", "Media Law", "International Media", "Propaganda", "Panic", "Subversion", "Local News", "Internet Media", "Alternative Sources", "Movies", "TV", "Cable News", "Media Consolidation", "Top-Down Influence", "Entertainment", "Advertising", "Walking The Line Between Truth And Fiction"],
    suggested_specialties_pt: &["Controle de Narrativa", "Política", "Influência", "Difamação", "Direito da Comunicação", "Mídia Internacional", "Propaganda", "Pânico", "Subversão", "Jornalismo Local", "Mídia na Internet", "Fontes Alternativas", "Cinema", "TV", "Canais de Notícias", "Concentração da Mídia", "Influência de Cima para Baixo", "Entretenimento", "Publicidade", "Andar na Linha entre Verdade e Ficção"],
};

pub const PHARMACOPOEIA: AbilityDefinition = AbilityDefinition {
    id: "pharmacopoeia",
    name: "Pharmacopoeia / Poisons",
    name_pt: "Farmacopeia / Venenos",
    category: AbilityCategory::Knowledges,
    scope: AbilityScope::Secondary,
    page_ref: "M20, p. 301",
    description: "Drugs are your specialty: street drugs, herbal compounds, experimental medications, psychoactives, hypertech concoctions, and the like. You understand the effects of various chemicals on the human animal and perhaps on other animals as well. A specialty of shamans, witches, Progenitors, Ecstatics, alchemists, and other mages who employ pharmaceutical compounds, this Knowledge allows you — with the proper tools, of course — to measure, recognize, analyze, counter, or dose someone with a range of chemicals. As usual, higher ratings reflect greater expertise. You’ll never know everything, of course, but you’re rarely flying blind.",
    description_pt: "Drogas são a sua especialidade: drogas de rua, compostos herbais, medicamentos experimentais, psicoativos, preparados hipertecnológicos e afins. Com as ferramentas certas, você sabe medir, reconhecer, analisar, neutralizar ou ministrar uma série de substâncias; a variação Conhecimento de Venenos cobre o uso dessas substâncias para fazer mal.",
    ratings: &[
        AbilityRating { dots: 1, title: "Dabbler", title_pt: "Curioso", description: "Street dealer.", description_pt: "Traficante de rua." },
        AbilityRating { dots: 2, title: "Student", title_pt: "Estudante", description: "Pharmacy student or savvy herbalist.", description_pt: "Estudante de farmácia ou herborista esperto." },
        AbilityRating { dots: 3, title: "Scholar", title_pt: "Erudito", description: "Dr. Timothy Leary.", description_pt: "Dr. Timothy Leary." },
        AbilityRating { dots: 4, title: "Professor", title_pt: "Professor", description: "Experienced with mystic, esoteric, and hypertech compounds.", description_pt: "Experiente com compostos místicos, esotéricos e hipertecnológicos." },
        AbilityRating { dots: 5, title: "Master", title_pt: "Mestre", description: "Renowned Pharmacopoeist.", description_pt: "Farmacopeísta renomado." },
    ],
    possessed_by: "Progenitors, Ecstatics, Alchemists, Entheogenic Mystics, Dope Dealers, Vice Cops, Verbena, Interrogators, Corrupters, Party Animals, Mad Scientists, Medical Professionals.",
    possessed_by_pt: "Progenitores, Extáticos, Alquimistas, Místicos Enteógenos, Traficantes, Policiais da Narcóticos, Verbena, Interrogadores, Corruptores, Festeiros, Cientistas Loucos, Profissionais da Saúde.",
    suggested_specialties: &["Mundane Meds", "Hypermeds", "Street Drugs", "Entheogens", "Magical Concoctions", "Designer Compounds", "Herbal Remedies", "Nutritional Supplements", "Mood-Altering Drugs"],
    suggested_specialties_pt: &["Medicamentos Comuns", "Hipermedicamentos", "Drogas de Rua", "Enteógenos", "Preparados Mágicos", "Compostos Sintéticos", "Remédios Herbais", "Suplementos Nutricionais", "Drogas que Alteram o Humor"],
};

pub const SECONDARY_ABILITIES: &[AbilityDefinition] = &[
    ANIMAL_KINSHIP,
    BLATANCY,
    CAROUSING,
    DO,
    FLYING,
    HIGH_RITUAL,
    LUCID_DREAMING,
    SEARCH,
    SEDUCTION,
    STYLE,
    ACROBATICS,
    ARCHERY,
    BIOTECH,
    ENERGY_WEAPONS,
    HYPERTECH,
    JETPACK,
    PILOT,
    RIDING,
    TORTURE,
    AREA_KNOWLEDGE,
    BELIEF_SYSTEMS,
    CRYPTOGRAPHY,
    DEMOLITIONS,
    FINANCE,
    LORE,
    MEDIA,
    PHARMACOPOEIA,
];
