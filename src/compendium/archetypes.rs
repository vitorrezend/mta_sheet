//! Canonical Personality Archetypes: Nature and Demeanor from M20 (pp. 267-273).
//!
//! Provides comprehensive, unabridged bilingual data (English and Brazilian Portuguese)
//! for all 20 canonical personality archetypes, along with the foundational rules
//! regarding Nature, Demeanor, and Willpower recovery mechanics.

use serde::Serialize;
use crate::i18n::Language;

/// Canonical Theory Article regarding Nature and Demeanor
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArchetypeTheoryArticle {
    pub id: &'static str,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub page_ref: &'static str,
    pub content: &'static str,
    pub content_pt: &'static str,
}

impl ArchetypeTheoryArticle {
    pub fn title(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.title_pt,
            Language::EnUs => self.title,
        }
    }

    pub fn content(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.content_pt,
            Language::EnUs => self.content,
        }
    }
}

/// Canonical Definition of a Personality Archetype (Nature & Demeanor)
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArchetypeDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub aliases: &'static [&'static str],
    pub page_ref: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
    pub strength_name: &'static str,
    pub strength_name_pt: &'static str,
    pub strength: &'static str,
    pub strength_pt: &'static str,
    pub weakness_name: &'static str,
    pub weakness_name_pt: &'static str,
    pub weakness: &'static str,
    pub weakness_pt: &'static str,
    pub willpower_regain: &'static str,
    pub willpower_regain_pt: &'static str,
}

impl ArchetypeDefinition {
    pub fn name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt,
            Language::EnUs => self.name,
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }

    pub fn strength_name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.strength_name_pt,
            Language::EnUs => self.strength_name,
        }
    }

    pub fn strength(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.strength_pt,
            Language::EnUs => self.strength,
        }
    }

    pub fn weakness_name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.weakness_name_pt,
            Language::EnUs => self.weakness_name,
        }
    }

    pub fn weakness(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.weakness_pt,
            Language::EnUs => self.weakness,
        }
    }

    pub fn willpower_regain(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.willpower_regain_pt,
            Language::EnUs => self.willpower_regain,
        }
    }
}

pub const ARCHETYPE_THEORY_RULES: ArchetypeTheoryArticle = ArchetypeTheoryArticle {
    id: "theory_nature_demeanor",
    title: "Personality Archetypes: Nature and Demeanor",
    title_pt: "Arquétipos de Personalidade: Natureza e Comportamento",
    page_ref: "M20, pp. 267-268",
    content: include_str!("../../data/compendium/archetypes/theory_nature_demeanor.en.md"),
    content_pt: include_str!("../../data/compendium/archetypes/theory_nature_demeanor.pt.md"),
};

pub const ARCHETYPE_ACTIVIST: ArchetypeDefinition = ArchetypeDefinition {
    id: "activist",
    name: "Activist",
    name_pt: "Ativista",
    aliases: &["activist", "ativista"],
    page_ref: "M20, pp. 267-268",
    description: r#"The world is broken. Help fix it. Speak truth to power, dig up secrets, call people out on their shit, and reveal your plans for a better world to anyone who will listen. While apathetic cowards sit back and tune out, you step up and do whatever needs doing. Sure, folks might consider you a pain in the ass, but at least you’re making a difference!"#,
    description_pt: r#"O mundo está quebrado. Ajude a consertá-lo. Fale a verdade para o poder, desenterre segredos, aponte a hipocrisia das pessoas na cara delas e revele seus planos para um mundo melhor para quem quiser ouvir. Enquanto covardes apáticos se acomodam e se desconectam, você dá um passo à frente e faz o que precisa ser feito. Claro, as pessoas podem achar você um pé no saco, mas ao menos você está fazendo a diferença!"#,
    strength_name: "Action",
    strength_name_pt: "Ação",
    strength: r#"Action is your greatest strength. You’re not one to sit things out. There’s no time to waste on mindless self-indulgence and no room to stay scared of what might happen. The wolf’s already halfway through the door, and you refuse to let that bastard win."#,
    strength_pt: r#"Ação é a sua maior força. Você não é de ficar de braços cruzados. Não há tempo a perder com autoindulgência irracional e não há espaço para ter medo do que pode acontecer. O lobo já está com metade do corpo para dentro da porta, e você se recusa a deixar aquele desgraçado vencer."#,
    weakness_name: "Outrage",
    weakness_name_pt: "Indignação Constante",
    weakness: r#"Even so, your constant Outrage wears thin. There’s never room to sit back and enjoy life. As far as you’re concerned, complacency is a sin. Your fury’s justified, of course, but it gets old all the same. Before you can truly Ascend, you’ve got to balance righteous pyrotechnics with calm acceptance. Life never has been – and never will be – perfect. Finding a place of serenity within your storm is an essential part of your transcendence."#,
    weakness_pt: r#"Mesmo assim, sua constante Indignação se desgasta. Nunca há espaço para relaxar e aproveitar a vida. Para você, a complacência é um pecado. Sua fúria é justificada, é claro, mas cansa do mesmo jeito. Antes de poder verdadeiramente Ascender, você precisa equilibrar a pirotecnia justa com a aceitação serena. A vida nunca foi — e nunca será — perfeita. Encontrar um lugar de serenidade dentro da sua tempestade é parte essencial da sua transcendência."#,
    willpower_regain: r#"Regain Willpower when you successfully confront abuse, right wrongs, or reveal an actual conspiracy and, by doing so, bring it down."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando confrontar com sucesso um abuso, corrigir injustiças ou desmascarar uma conspiração real e, com isso, derrubá-la."#,
};

pub const ARCHETYPE_BENEFACTOR: ArchetypeDefinition = ArchetypeDefinition {
    id: "benefactor",
    name: "Benefactor",
    name_pt: "Benfeitor",
    aliases: &["benefactor", "benfeitor"],
    page_ref: "M20, p. 268",
    description: r#"It’s a cruel world, but you make things easier. Generous sometimes to a fault, you supply whatever you can provide: money, advice, protection, maybe just a shoulder when someone really needs to cry. You can’t just turn blind eyes to suffering and need. It’s your moral duty to do whatever you can do to make things right. When things get tough, you call upon your inner White Knight and charge in, bringing gifts, guidance, and occasional force when nothing else will do."#,
    description_pt: r#"É um mundo cruel, mas você torna as coisas mais fáceis. Generoso às vezes até demais, você fornece tudo o que puder: dinheiro, conselhos, proteção, ou talvez apenas um ombro amigo quando alguém realmente precisa chorar. Você simplesmente não consegue fechar os olhos para o sofrimento e a necessidade. É seu dever moral fazer o que puder para consertar as coisas. Quando a situação aperta, você convoca seu Cavaleiro Branco interior e investe, trazendo presentes, orientação e, ocasionalmente, força quando nada mais resolve."#,
    strength_name: "Altruism",
    strength_name_pt: "Altruísmo",
    strength: r#"Altruism is all too rare, especially in the World of Darkness. Helping people is your pleasure. Magick, as far as you’re concerned, is a tool for helping folks less fortunate than yourself. To do less is an abuse of the powers you possess."#,
    strength_pt: r#"Altruísmo é algo muito raro, especialmente no Mundo das Trevas. Ajudar as pessoas é o seu prazer. Mágica, no que lhe diz respeito, é uma ferramenta para ajudar quem é menos afortunado do que você. Fazer menos do que isso é um abuso dos poderes que você possui."#,
    weakness_name: "Obligation",
    weakness_name_pt: "Obrigação Compulsiva",
    weakness: r#"On the flipside, though, you often feel Obligation even when you’re not actually needed. This, in turn, can become resentment – both on your part and on the parts of people who now feel, rightly or wrongly, that they owe you. Sometimes, you just need to back off, chill out, and let people do things for themselves. Martyrdom isn’t always the best Path toward Ascension."#,
    weakness_pt: r#"Por outro lado, porém, você frequentemente sente Obrigação mesmo quando sua ajuda não é realmente necessária. Isso, por sua vez, pode virar ressentimento — tanto da sua parte quanto da parte de pessoas que agora sentem, com ou sem razão, que estão em dívida com você. Às vezes, você só precisa recuar, relaxar e deixar as pessoas resolverem as coisas por si mesmas. O martírio nem sempre é o melhor Caminho para a Ascensão."#,
    willpower_regain: r#"Regain Willpower when you provide help that someone else desperately needs. The key here is that desperate part. Teaching Sunday school isn’t meeting that level of need; if you used your Arts to assist autistic kids, however, you’d clearly be making a difference in their lives."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando fornecer ajuda de que outra pessoa necessite desesperadamente. O ponto fundamental aqui é a palavra desesperadamente. Dar aulas na escola dominical não alcança esse nível de necessidade; mas se você usou suas Artes para ajudar crianças autistas, estará claramente fazendo a diferença na vida delas."#,
};

pub const ARCHETYPE_CONTRARY: ArchetypeDefinition = ArchetypeDefinition {
    id: "contrary",
    name: "Contrary",
    name_pt: "Contrário",
    aliases: &["contrary", "contrario", "do contra"],
    page_ref: "M20, p. 268",
    description: r#"Inversion is an essential part of real life. For every rule, there must be exceptions. You live to turn things inside out, undercutting assumptions by showing their weak foundations. You’re the Devil’s Advocate, pointing out flaws by embodying the opposite of what folks expect."#,
    description_pt: r#"A inversão é uma parte essencial da vida real. Para cada regra, deve haver exceções. Você vive para virar as coisas do avesso, minando suposições ao expor seus alicerces frágeis. Você é o Advogado do Diabo, apontando falhas ao personificar o oposto exato do que as pessoas esperam."#,
    strength_name: "Insight",
    strength_name_pt: "Percepção / Insight",
    strength: r#"A successful Contrary displays Insight; your inversions succeed because they point to a deeper truth. If you’re serious about this path (that is, if you’re an actual Contrary, not simply an asshole), then you’re a jester with a clue. That ability to see beyond appearances is extremely useful in the Awakened realm."#,
    strength_pt: r#"Um Contrário bem-sucedido demonstra Percepção; suas inversões dão certo porque apontam para uma verdade mais profunda. Se você leva esse caminho a sério (isto é, se você é um verdadeiro Contrário e não simplesmente um babaca), então você é um bobo da corte perspicaz. Essa capacidade de enxergar além das aparências é extremamente útil no reino dos Despertos."#,
    weakness_name: "Subversion",
    weakness_name_pt: "Subversão Perpétua",
    weakness: r#"And yet, your perpetual Subversion can be annoying, intrusive, and outright destructive. Certain assumptions really are true, and inverting them doesn’t provide any particular wisdom. If you seriously want to reach beyond appearances, you have to realize that your own contrariness can be its own limitation. True wisdom comes through a balance between rejection and acceptance."#,
    weakness_pt: r#"E no entanto, sua perpétua Subversão pode ser irritante, invasiva e abertamente destrutiva. Certas suposições são genuinamente verdadeiras, e invertê-las não traz sabedoria alguma. Se você quer seriamente ir além das aparências, precisa entender que sua própria mania de contrariar pode ser sua própria limitação. A verdadeira sabedoria vem do equilíbrio entre rejeição e aceitação."#,
    willpower_regain: r#"Regain Willpower whenever your inversion of expectations leads folks to realize how false those expectations had been."#,
    willpower_regain_pt: r#"Recupere Força de Vontade sempre que sua inversão de expectativas levar as pessoas a perceberem quão falsas eram essas expectativas."#,
};

pub const ARCHETYPE_CRUSADER: ArchetypeDefinition = ArchetypeDefinition {
    id: "crusader",
    name: "Crusader",
    name_pt: "Cruzado",
    aliases: &["crusader", "cruzado"],
    page_ref: "M20, pp. 268-269",
    description: r#"People need a hero, and you’re there to fill that role. Driven by a higher purpose – religious conviction, moral ethics, a philosophical ideal – you strive for a better world. The sword you wield might be more symbolic than literal: a scientist or teacher can be a Crusader too. The struggle, though, is what defines you. A better future must be built upon the foundations of our flawed world, and your duty compels you to demolish the obstacles so that reconstruction can begin."#,
    description_pt: r#"As pessoas precisam de um herói, e você está lá para cumprir esse papel. Movido por um propósito superior — convicção religiosa, ética moral, um ideal filosófico — você luta por um mundo melhor. A espada que você empunha pode ser mais simbólica do que literal: um cientista ou professor também pode ser um Cruzado. A luta, porém, é o que define você. Um futuro melhor deve ser construído sobre as fundações do nosso mundo imperfeito, e seu dever o compele a demolir os obstáculos para que a reconstrução possa começar."#,
    strength_name: "Zeal",
    strength_name_pt: "Zelo",
    strength: r#"Your admirable Zeal propels you through every challenge you face. Lesser souls might falter, but you will not."#,
    strength_pt: r#"Seu admirável Zelo o impulsiona através de cada desafio que você enfrenta. Almas menores podem vacilar, mas você não."#,
    weakness_name: "Fanaticism",
    weakness_name_pt: "Fanatismo",
    weakness: r#"Fanaticism, though, is the mark of a Crusader. Your convictions leave little room for compromise. When everything looks like a nail, it’s hard to stop hammering. Until and unless you learn to temper your enthusiasm and question your own assumptions, you’re just chasing shadows of your own extremism."#,
    weakness_pt: r#"O Fanatismo, contudo, é a marca do Cruzado. Suas convicções deixam pouco espaço para concessões. Quando tudo parece um prego, é difícil parar de martelar. A menos que você aprenda a dosar seu entusiasmo e a questionar suas próprias certezas, estará apenas perseguindo sombras do seu próprio extremismo."#,
    willpower_regain: r#"Regain Willpower when you accomplish some great deed in the name of your higher goal. As a player, you need to define what that higher goal is, and then follow it even when it hurts you. A true Crusader’s beliefs are not governed by convenience."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando realizar algum grande feito em nome do seu objetivo superior. Como jogador, você precisa definir qual é esse objetivo superior e segui-lo mesmo quando isso lhe custar caro. As crenças de um verdadeiro Cruzado não são governadas pela conveniência."#,
};

pub const ARCHETYPE_HACKER: ArchetypeDefinition = ArchetypeDefinition {
    id: "hacker",
    name: "Hacker",
    name_pt: "Hacker",
    aliases: &["hacker"],
    page_ref: "M20, p. 269",
    description: r#"Every system is a locked vault, and you’ve got the keys. If you don’t have ones that work, you’ll make new ones – that’s half the fun of life, after all! Puzzles excite you; limits just piss you off. Especially given the sheer amount of abuse that’s built into any system, the world needs folks like you to tear down impediments and set reality free."#,
    description_pt: r#"Todo sistema é um cofre trancado, e você tem as chaves. Se não tiver as que funcionam, você fabricará novas — afinal, essa é metade da graça da vida! Enigmas o fascinam; limites apenas o irritam. Especialmente diante da imensa quantidade de abusos embutidos em qualquer sistema, o mundo precisa de pessoas como você para derrubar impedimentos e libertar a realidade."#,
    strength_name: "Imagination",
    strength_name_pt: "Imaginação",
    strength: r#"Imagination is your greatest strength. You see things not as they are, but as they could be once you get done with them! Driven to dismantle existing systems and then put them together in interesting new shapes, you tend to pepper your compulsions with sincerely-held philosophy. You’re not a vandal, for crying out loud – you’re a visionary who refuses to accept shit sandwiches handed out as lunch."#,
    strength_pt: r#"Imaginação é a sua maior força. Você enxerga as coisas não como elas são, mas como poderiam ser depois que você terminar de mexer nelas! Impelido a desmontar sistemas existentes e depois remontá-los em formatos novos e intrigantes, você tende a temperar suas compulsões com filosofia genuína. Pelo amor de Deus, você não é um vândalo — você é um visionário que se recusa a aceitar engolir sapos servidos no almoço."#,
    weakness_name: "Perversity",
    weakness_name_pt: "Perversidade Desconstrutiva",
    weakness: r#"Sometimes, though, you go too far. Perversity leads you to tear apart things that weren’t broken… things that may, in fact, have been better left alone. Although it might seem philosophically valid to tear down the wards on a wizard’s lab, those wards might have been laid that way for reasons you didn’t understand until it was a little too late."#,
    weakness_pt: r#"Às vezes, no entanto, você vai longe demais. A Perversidade o leva a despedaçar coisas que não estavam quebradas… coisas que, de fato, teriam ficado muito melhor intocadas. Embora possa parecer filosoficamente válido derrubar as proteções místicas do laboratório de um mago, essas proteções podem ter sido colocadas lá por razões que você não compreendeu até ser tarde demais."#,
    willpower_regain: r#"Regain Willpower when you detect a flaw in some important structure, system, or device, or else when you puzzle out a way to improve something that was supposedly designed well to begin with."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando detectar uma falha em alguma estrutura, sistema ou dispositivo importante, ou quando decifrar uma maneira de aprimorar algo que supostamente já havia sido bem projetado desde o princípio."#,
};

pub const ARCHETYPE_IDEALIST: ArchetypeDefinition = ArchetypeDefinition {
    id: "idealist",
    name: "Idealist",
    name_pt: "Idealista",
    aliases: &["idealist", "idealista"],
    page_ref: "M20, pp. 269-270",
    description: r#"Cowards accept what is. You know how much better things will be once the flaws in the system have been purged. Guided by a great ideal – spiritual devotion, political philosophy, scientific theory, compassionate humanity – you refuse to remain shackled by defeatism. Your Ideal is correct. You know this to be true. Now is the time to bring it about. Once you do, everyone else will see just how wrong they’ve been… and how right you are."#,
    description_pt: r#"Covardes aceitam o que está posto. Você sabe o quanto as coisas serão melhores assim que as falhas no sistema forem purgadas. Guiado por um grande ideal — devoção espiritual, filosofia política, teoria científica, humanidade compassiva — você se recusa a permanecer acorrentado pelo derrotismo. Seu Ideal está correto. Você sabe que isso é verdade. Agora é a hora de concretizá-lo. Uma vez que você o faça, todos os outros verão o quanto estiveram errados… e o quanto você está certo."#,
    strength_name: "Conviction",
    strength_name_pt: "Convicção",
    strength: r#"Conviction is your life’s blood. Whether you’re a scientist chasing inspiration, a theorist assured that this theory cannot fail, a religious devotee, or some other sort of Idealist, you possess near-unshakable faith in your ideal – the kind of faith that shapes reality."#,
    strength_pt: r#"Convicção é a seiva da sua vida. Seja você um cientista perseguindo inspiração, um teórico seguro de que sua tese não pode falhar, um devoto religioso ou outro tipo de Idealista, você possui uma fé quase inabalável no seu ideal — o tipo de fé que molda a realidade."#,
    weakness_name: "Dogmatic",
    weakness_name_pt: "Dogmatismo",
    weakness: r#"Like most fanatics, though, you’re Dogmatic to a fault. Blind to any potential flaws, you’ll fight – sometimes literally – to assure the truth of your beliefs. All mages are idealists to a point; in your case, though, this devotion seems stifling. When (not if, when) reality falls short of your ideals, you might wind up depressed, violent, or – as with many Marauders – insane."#,
    weakness_pt: r#"Como a maioria dos fanáticos, contudo, você é Dogmático ao extremo. Cego a quaisquer falhas em potencial, você lutará — às vezes literalmente — para assegurar a verdade de suas crenças. Todos os magos são idealistas até certo ponto; no seu caso, porém, essa devoção parece sufocante. Quando (não se, mas quando) a realidade não alcançar seus ideais, você pode acabar deprimido, violento ou — como muitos Espreitadores/Marauders — completamente insano."#,
    willpower_regain: r#"Regain Willpower when your beliefs are tested to the breaking point yet do not fail."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando suas crenças forem testadas até o ponto de ruptura e ainda assim não falharem."#,
};

pub const ARCHETYPE_INNOVATOR: ArchetypeDefinition = ArchetypeDefinition {
    id: "innovator",
    name: "Innovator",
    name_pt: "Inovador",
    aliases: &["innovator", "inovador"],
    page_ref: "M20, p. 270",
    description: r#"There’s always a better way. You spend your life looking for methods and inventions that improve on what has gone before. Sure, past achievements are wonderful enough… but if you just add this, shift that, approach the issue from this other angle instead, then you’ll make a good thing that much better or fix an obvious flaw in a promising design."#,
    description_pt: r#"Sempre existe um jeito melhor. Você passa a vida procurando métodos e invenções que aprimorem o que veio antes. Claro, as conquistas do passado são maravilhosas… mas se você apenas adicionar isso, mudar aquilo, abordar a questão por este outro ângulo, então tornará algo bom muito melhor ou consertará uma falha óbvia em um projeto promissor."#,
    strength_name: "Creativity",
    strength_name_pt: "Criatividade",
    strength: r#"Creativity is your strong point. No practice, tool, or traditional method is too good for improvement, especially not at this crucial point in human evolution. Let other people follow the established paths – you’re busy drawing up the next road toward a goal most folks don’t even know exists."#,
    strength_pt: r#"Criatividade é o seu ponto forte. Nenhuma prática, ferramenta ou método tradicional é bom demais que não possa ser melhorado, especialmente neste momento crucial da evolução humana. Deixe que os outros sigam os caminhos estabelecidos — você está ocupado traçando a próxima estrada rumo a um objetivo que a maioria das pessoas nem sabe que existe."#,
    weakness_name: "Restless Unorthodoxy",
    weakness_name_pt: "Heterodoxia Inquieta",
    weakness: r#"Your Restless Unorthodoxy, though, can get you in trouble. Especially if you belong to a sect or faction based on established results and protocols (the Technocracy, the Hermetic Order, the Akashayana, and so on), your innovations might be someone else’s heresy. Many mages have been burnt at stakes both literal and symbolic for doing the things you do, and a missed step could make (an) ash of you."#,
    weakness_pt: r#"Sua Heterodoxia Inquieta, contudo, pode colocá-lo em apuros. Especialmente se você pertencer a uma seita ou facção baseada em resultados e protocolos estabelecidos (a Tecnocracia, a Ordem Hermética, os Akashayana e assim por diante), suas inovações podem ser a heresia de outra pessoa. Muitos magos foram queimados em fogueiras tanto literais quanto simbólicas por fazer o que você faz, e um passo em falso pode reduzi-lo a cinzas."#,
    willpower_regain: r#"Regain Willpower when your inspiration leads to a helpful new breakthrough."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando sua inspiração levar a uma nova e valiosa descoberta ou avanço prático."#,
};

pub const ARCHETYPE_KID: ArchetypeDefinition = ArchetypeDefinition {
    id: "kid",
    name: "Kid",
    name_pt: "Criança",
    aliases: &["kid", "crianca"],
    page_ref: "M20, p. 270",
    description: r#"Either young in years or young at heart, you give the impression of needing to be taken care of by older, wiser folks. That impression may be deceiving, but you’re glad to use it to your best advantage. You could be an actual child – somewhere between post-toddlerhood and late adolescence – or chronologically an adult with a childlike personality. Whatever your actual age might be, your concerns are simple, capricious, and typically self-involved."#,
    description_pt: r#"Seja jovem em idade ou jovem de coração, você passa a impressão de precisar do cuidado de pessoas mais velhas e sábias. Essa impressão pode ser enganosa, mas você não hesita em usá-la em seu proveito. Você pode ser uma criança de fato — entre a primeira infância e o final da adolescência — ou cronologicamente um adulto com personalidade infantil. Seja qual for a sua idade real, suas preocupações são simples, caprichosas e tipicamente autocentradas."#,
    strength_name: "Innocence",
    strength_name_pt: "Inocência",
    strength: r#"There’s a kind of Innocence to you, and it provides the source of your strength. People want to protect that sense of wonder and hope and tend to offer you a compassion they might not feel toward other folks."#,
    strength_pt: r#"Há uma espécie de Inocência em você, e ela é a fonte da sua força. As pessoas querem proteger essa sensação de encantamento e esperança, e tendem a lhe oferecer uma compaixão que talvez não sentissem em relação a outros."#,
    weakness_name: "Immaturity",
    weakness_name_pt: "Imaturidade",
    weakness: r#"The extreme of innocence, though, is Immaturity. Trusting too easily, acting too rashly, jumping into things without considering their potential consequences… you’re guilty of all these things and more. You can slide from charming child to spoiled brat in the space between two heartbeats, and the traits that inspire people to take care of you can become infuriating as well. If and when you want to reach a higher state, you’ll have to put certain childish things aside."#,
    weakness_pt: r#"O extremo da inocência, porém, é a Imaturidade. Confiar com facilidade excessiva, agir de forma precipitada, mergulhar nas coisas sem considerar as consequências potenciais… você é culpado de tudo isso e mais. Você pode transitar de criança encantadora a pivete mimado no espaço de dois batimentos cardíacos, e os traços que inspiram as pessoas a cuidar de você também podem se tornar exasperantes. Se e quando desejar alcançar um estado superior, terá que deixar para trás certas infantilidades."#,
    willpower_regain: r#"Regain Willpower when you bring out the nurturing side of someone who doesn’t normally care much for other people."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando despertar o lado protetor e acolhedor de alguém que normalmente não se importa com outras pessoas."#,
};

pub const ARCHETYPE_LONER: ArchetypeDefinition = ArchetypeDefinition {
    id: "loner",
    name: "Loner",
    name_pt: "Solitário",
    aliases: &["loner", "solitario"],
    page_ref: "M20, pp. 270-271",
    description: r#"Fuck the world. Everything you need in life you’ve got inside yourself. Even in the middle of a crowd, you’re alone… or at least, you feel that way. People don’t understand you, and you don’t care enough to try to understand them. It’s not that you’re a sociopath or anything (although you just might be one) so much as it’s that you just don’t connect with that whole social animal thing at all."#,
    description_pt: r#"Foda-se o mundo. Tudo o que você precisa na vida você tem dentro de si mesmo. Mesmo no meio de uma multidão, você está sozinho… ou, ao menos, é assim que se sente. As pessoas não entendem você, e você não se importa o suficiente para tentar entendê-las. Não é que você seja um sociopata nem nada do tipo (embora até possa ser), mas sim que simplesmente não se conecta com toda essa história de 'animal social'."#,
    strength_name: "Self-Reliance",
    strength_name_pt: "Autossuficiência",
    strength: r#"Self-Reliance is your blessing. You’re so used to doing everything yourself that you rarely depend on another person’s aid. Ascension, so far as you understand it, is a solitary task, so why bother asking for assistance with that goal?"#,
    strength_pt: r#"Autossuficiência é a sua bênção. Você está tão acostumado a fazer tudo sozinho que raramente depende da ajuda de outra pessoa. A Ascensão, até onde você compreende, é uma tarefa solitária, então por que se dar ao trabalho de pedir assistência para alcançar esse objetivo?"#,
    weakness_name: "Disconnection",
    weakness_name_pt: "Desconexão",
    weakness: r#"Trouble is, your Disconnection cuts you off from empathy and the wealth of human experience. Until and unless you let down the barriers and connect with other living things, you’re doomed to a lonesome and limited existence."#,
    weakness_pt: r#"O problema é que sua Desconexão o afasta da empatia e da riqueza da experiência humana. A menos que você baixe as barreiras e se conecte com outros seres vivos, estará condenado a uma existência solitária e limitada."#,
    willpower_regain: r#"Regain Willpower when you achieve some meaningful goal without help from anyone else, especially if your accomplishment benefits other people too."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando atingir algum objetivo significativo sem a ajuda de ninguém, especialmente se sua realização beneficiar outras pessoas também."#,
};

pub const ARCHETYPE_MACHINE: ArchetypeDefinition = ArchetypeDefinition {
    id: "machine",
    name: "Machine",
    name_pt: "Máquina",
    aliases: &["machine", "maquina"],
    page_ref: "M20, p. 270",
    description: r#"Flesh is weak. You are not. Whether or not you’re an actual cyborg, you have the relentless drive of a Terminator on PCP. Flaws do not concern you. Limits are for the mewling meatbags known as human beings. Compassion is for weaklings. Fatigue is for lesser entities. If and when you fall short of perfection, you understand that such failure is just a challenge to do better next time."#,
    description_pt: r#"A carne é fraca. Você não. Quer seja um ciborgue de verdade ou não, você possui o ímpeto implacável de um Exterminador sob efeito de drogas pesadas. Falhas não lhe dizem respeito. Limites são para os sacos de carne chorosos conhecidos como seres humanos. Compaixão é para os fracos. Fadiga é para entidades inferiores. Se e quando você ficar aquém da perfeição, entenderá que tal falha é apenas um desafio para fazer melhor da próxima vez."#,
    strength_name: "Diligent, Stoic",
    strength_name_pt: "Diligente e Estóico",
    strength: r#"A Diligent, Stoic creature, you push yourself to the limits of endurance. Everything you do is done to exacting standards, without complaint or hesitation. The Technocratic Union loves operatives like you."#,
    strength_pt: r#"Uma criatura Diligente e Estóica, você se esforça até os limites extremos da resistência. Tudo o que você faz é executado segundo padrões rigorosos, sem reclamações ou hesitação. A União Tecnocrática adora agentes como você."#,
    weakness_name: "Creepy and Prone to Burn-Out",
    weakness_name_pt: "Sinistro e Propenso a Esgotamento",
    weakness: r#"As far as other people are concerned, however, you’re Creepy and Prone to Burn-Out. Whatever illusions you harbor about your perfection, you’re as flawed as any other mortal thing. Even machines have limits, and unless you learn to acknowledge your own, you’re headed toward the scrap heap, not Ascension."#,
    weakness_pt: r#"No que diz respeito aos outros, no entanto, você é Sinistro e Propenso a Esgotamento. Quaisquer ilusões que alimente sobre a sua perfeição, você é tão falho quanto qualquer outra coisa mortal. Até máquinas têm limites, e a menos que aprenda a reconhecer os seus, estará caminhando para o ferro-velho, e não para a Ascensão."#,
    willpower_regain: r#"Regain Willpower when you transcend the limits of mere flesh, bone, and steel, accomplishing something that no one else has managed to do."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando transcender os limites de mera carne, osso e aço, realizando algo que ninguém mais conseguiu fazer."#,
};

pub const ARCHETYPE_MAD_SCIENTIST: ArchetypeDefinition = ArchetypeDefinition {
    id: "mad_scientist",
    name: "Mad Scientist",
    name_pt: "Cientista Louco",
    aliases: &["mad scientist", "cientista louco", "mad_scientist"],
    page_ref: "M20, pp. 270-271",
    description: r#"Science is not a field for cowards and fools. You understand that technology expands only when brave souls such as yourself dare the unthinkable and push past the established norms of hidebound preconceptions. Yes, those cowards and fools do falter when they behold the scope of your temerity. Let them! Just as Vesalius defied taboos when he performed dissections or Galileo challenged the Church with his discoveries, so too must other visionaries stand firm and expand the reach of science… visionaries including, of course, yourself."#,
    description_pt: r#"A ciência não é um campo para covardes e tolos. Você compreende que a tecnologia só avança quando almas corajosas como você ousam o impensável e ultrapassam as normas estabelecidas dos preconceitos retrógrados. Sim, esses covardes e tolos vacilam quando contemplam a extensão da sua audácia. Deixe-os! Assim como Vesalius desafiou tabus ao realizar dissecações ou Galileu desafiou a Igreja com suas descobertas, outros visionários também devem se manter firmes e expandir o alcance da ciência… visionários incluindo, é claro, você mesmo."#,
    strength_name: "Vision",
    strength_name_pt: "Visão",
    strength: r#"You do, in fact, have Vision. It’s probably your greatest strength. Pursing that vision despite all obstacles, you move the boundaries of reality through sheer determination."#,
    strength_pt: r#"Você tem, de fato, Visão. É provavelmente a sua maior força. Perseguindo essa visão apesar de todos os obstáculos, você desloca os limites da realidade por pura determinação."#,
    weakness_name: "Batshit Insane",
    weakness_name_pt: "Loucura Total",
    weakness: r#"Problem is, you are Batshit Insane. There are reasons that other people have not dared the things you do or, if they have, that they’re reviled names in the Halls of Infamy. The Technocracy, Virtual Adepts, and especially the Etherites might depend upon minds like yours, but they also keep a wary eye out for the times when you’ll go too far – because you will."#,
    weakness_pt: r#"O problema é que você é Completamente Maluco. Existem razões para outras pessoas não terem ousado as coisas que você faz ou, se o fizeram, serem nomes execrados no Hall da Infâmia. A Tecnocracia, os Adeptos da Virtualidade e especialmente os Eteritas podem depender de mentes como a sua, mas também mantêm um olho atento e receoso para os momentos em que você irá longe demais — porque você irá."#,
    willpower_regain: r#"Regain Willpower when you successfully bend the rules of your technomancer sect while still applying something that seems oddly like science (see the sidebar SCIENCE!!!, p. 290) and manage to avoid invoking Paradox. This feat should feature a bizarre stroke of ingenuity that shouldn’t work and yet somehow does. (Simply using magick does not count – after all, you’re not some kind of sorcerer!)"#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando você dobrar com sucesso as regras da sua facção tecnomante enquanto ainda aplica algo que parece estranhamente com ciência (veja o quadro CIÊNCIA!!!, p. 290) e conseguir evitar invocar o Paradoxo. Esse feito deve apresentar uma sacada bizarra de engenhosidade que não deveria funcionar, mas de algum modo funciona. (Simplesmente usar mágica não conta — afinal, você não é algum tipo de feiticeiro!)."#,
};

pub const ARCHETYPE_MARTYR: ArchetypeDefinition = ArchetypeDefinition {
    id: "martyr",
    name: "Martyr",
    name_pt: "Mártir",
    aliases: &["martyr", "martir"],
    page_ref: "M20, p. 271",
    description: r#"It’s your glory to sacrifice yourself for the greater good. Precisely what this greater good looks like is up to you. It’s probably based on a religious creed – possibly, but not necessarily, Islam or Christianity. Then again, you may want to give your all for a secular cause you believe in, maybe a political ideology, a philosophical ideal, or simply the opportunity to be heroic in ways only you can perform. This sacrifice doesn’t have to be fatal, though it’ll probably wind up being terminal eventually. Until then, you tend to put yourself in a position to suffer so that someone else might thrive."#,
    description_pt: r#"É a sua glória sacrificar-se pelo bem maior. O formato exato desse bem maior depende de você. Provavelmente baseia-se em um credo religioso — possivelmente, mas não necessariamente, o Islã ou o Cristianismo. Por outro lado, você pode querer dar tudo de si por uma causa secular em que acredita, talvez uma ideologia política, um ideal filosófico ou simplesmente a oportunidade de ser heróico de maneiras que só você pode realizar. Esse sacrifício não precisa ser fatal, embora provavelmente acabe sendo terminal em algum momento. Até lá, você tende a se colocar em posição de sofrer para que outro possa prosperar."#,
    strength_name: "Sacrifice",
    strength_name_pt: "Sacrifício",
    strength: r#"Sacrifice is a mighty thing in magick. The willingness to surrender one’s self to a greater end is one of the most obvious (and famous) Paths toward Ascension."#,
    strength_pt: r#"O Sacrifício é algo grandioso na mágica. A disposição de entregar a si mesmo por um fim maior é um dos Caminhos mais evidentes (e célebres) rumo à Ascensão."#,
    weakness_name: "Self-Deprecation",
    weakness_name_pt: "Autodepreciação",
    weakness: r#"That said, such Self-Deprecation can have unfortunate consequences far beyond simple hazards to your health. Martyrs tend to get on people’s nerves, wind up being taken advantage of, and deal with abuse that has nothing to do with the greater good. Worse, perhaps, people like you often resent the beneficiaries of their sacrifice, many of whom might not want you to sacrifice yourself at all."#,
    weakness_pt: r#"Dito isso, tal Autodepreciação pode ter consequências infelizes muito além dos simples riscos à sua saúde. Mártires tendem a dar nos nervos dos outros, acabam sendo explorados e suportam abusos que nada têm a ver com o bem maior. Pior, talvez, pessoas como você frequentemente se ressentem dos beneficiários do seu sacrifício, muitos dos quais talvez nem quisessem que você se sacrificasse."#,
    willpower_regain: r#"Regain Willpower whenever you manage to make a noticeably positive difference in someone else’s situation by giving deeply of yourself."#,
    willpower_regain_pt: r#"Recupere Força de Vontade sempre que conseguir fazer uma diferença visivelmente positiva na situação de outra pessoa ao doar profundamente de si mesmo."#,
};

pub const ARCHETYPE_MONSTER: ArchetypeDefinition = ArchetypeDefinition {
    id: "monster",
    name: "Monster",
    name_pt: "Monstro",
    aliases: &["monster", "monstro"],
    page_ref: "M20, p. 271",
    description: r#"Heaven lacks glory without the threat of Hell. It’s your noble chore, then, to be the Agent of Infamy. An obvious choice for Nephandic mages (though not, by a long shot, exclusive to their kind), this Archetype embodies unapologetic villainy. As Voltaire (the singer, not the philosopher) put it, “It’s so easy when you’re evil”… easy, fun, and satisfying!"#,
    description_pt: r#"O Céu não tem glória sem a ameaça do Inferno. É a sua nobre tarefa, portanto, ser o Agente da Infâmia. Uma escolha óbvia para magos Nefandi (embora, nem de longe, exclusiva da laia deles), este Arquétipo encarna a vilania descarada. Como Voltaire (o cantor, não o filósofo) disse: “It’s so easy when you’re evil” (É tão fácil quando você é mau)… fácil, divertido e gratificante!"#,
    strength_name: "Reveal Dark Truths",
    strength_name_pt: "Revelar Verdades Sombrias",
    strength: r#"You Reveal Dark Truths that many people are afraid to face. When you show up, Pollyanna runs screaming for the hills. Your existence undercuts the comfortable lies of polite society. Anyone can play a hero, but a memorable villain is worth his weight in blood."#,
    strength_pt: r#"Você Revela Verdades Sombrias que muitas pessoas têm medo de encarar. Quando você aparece, os ingênuos fogem aos berros. A sua existência desmorona as mentiras confortáveis da sociedade educada. Qualquer um pode bancar o herói, mas um vilão memorável vale o seu peso em sangue."#,
    weakness_name: "Depraved",
    weakness_name_pt: "Depravação",
    weakness: r#"The problem, of course, is that you’re Depraved. There might be a heroic heel-turn in your future, but that’s unlikely. Chances are, you’re headed for the Cauls; if you haven’t Fallen with the Nephandi already, you probably will soon enough."#,
    weakness_pt: r#"O problema, claro, é que você é Depravado. Pode ser que haja uma virada heróica no seu futuro, mas isso é improvável. As chances são de que você esteja a caminho dos Casulos (Cauls); se você ainda não Caiu com os Nefandi, provavelmente o fará em breve."#,
    willpower_regain: r#"Regain Willpower by performing spectacular acts of heartlessness and ruin."#,
    willpower_regain_pt: r#"Recupere Força de Vontade realizando atos espetaculares de crueldade e ruína."#,
};

pub const ARCHETYPE_PROPHET: ArchetypeDefinition = ArchetypeDefinition {
    id: "prophet",
    name: "Prophet",
    name_pt: "Profeta",
    aliases: &["prophet", "profeta"],
    page_ref: "M20, pp. 271-272",
    description: r#"Speaking capital-T Truth, you bring a sacred message to a wounded world. That message might not necessarily be religious, though it’s at least got religious undertones. Regardless of its nature, it’s capital-I Important in ways that simple opinions could never be. Traditionally, a prophet becomes the chosen representative of Truth, throwing light into shadowed corners and forcing folks to look at their own hypocrisies. It’s not an easy cross to bear, but Truth will not be denied for long."#,
    description_pt: r#"Proferindo a Verdade com V maiúsculo, você traz uma mensagem sagrada a um mundo ferido. Essa mensagem não precisa necessariamente ser religiosa, embora possua ao menos nuances místicas. Independentemente de sua natureza, ela é Importante com I maiúsculo de maneiras que simples opiniões jamais poderiam ser. Tradicionalmente, um profeta se torna o representante escolhido da Verdade, lançando luz sobre cantos sombrios e forçando as pessoas a encararem suas próprias hipocrisias. Não é uma cruz fácil de carregar, mas a Verdade não será negada por muito tempo."#,
    strength_name: "Insight",
    strength_name_pt: "Visão e Percepção",
    strength: r#"Insight is your greatest strength. Prophets tend to see things that are hidden to most people: secrets, omens, visions of the future, and so forth. Even if you lack the blessing/ curse of prophecy, few mysteries escape your sight."#,
    strength_pt: r#"Percepção é a sua maior força. Profetas tendem a enxergar coisas ocultas para a maioria: segredos, presságios, visões do futuro e afins. Mesmo que lhe falte a bênção/maldição da profecia mística, poucos mistérios escapam ao seu olhar."#,
    weakness_name: "Ruthlessness",
    weakness_name_pt: "Implacabilidade",
    weakness: r#"Ruthlessness is the traditional flaw of prophets. Driven by their vision of Truth, such people tend to be impatient, fanatical, and defiant of mortal power. You’ll probably score lots of points for guts, but you might find those guts roasting on a spit in some ruler’s torture garden. True prophets often meet unhappy ends… and they tend to take their followers down with them when they go."#,
    weakness_pt: r#"Implacabilidade é a falha tradicional dos profetas. Movidas por sua visão da Verdade, tais pessoas tendem a ser impacientes, fanáticas e desafiadoras do poder mortal. Você provavelmente ganhará muitos pontos por coragem, mas poderá ver suas vísceras assando em um espeto no calabouço de algum tirano. Verdadeiros profetas frequentemente encontram finais infelizes… e tendem a levar seus seguidores para a cova junto com eles."#,
    willpower_regain: r#"Regain Willpower when you speak Truth to power and inspire a successful change."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando você falar a Verdade aos poderosos e inspirar uma mudança bem-sucedida."#,
};

pub const ARCHETYPE_ROGUE: ArchetypeDefinition = ArchetypeDefinition {
    id: "rogue",
    name: "Rogue",
    name_pt: "Malandro",
    aliases: &["rogue", "malandro", "rebelde"],
    page_ref: "M20, p. 272",
    description: r#"You make rebellion look good. More than just some scruffy malcontent, you challenge convention with stylish dispatch. Attitude is your first, last and middle name, and it’s the only label you accept. You give not one precious fuck for authority, and when you finally head towards that Big Dirt Nap, you plan to go out laughing."#,
    description_pt: r#"Você faz a rebeldia ter estilo. Muito mais do que apenas um desajustado qualquer, você desafia as convenções com elegância e atitude. Postura é seu primeiro, último e nome do meio, e é o único rótulo que você aceita. Você não dá a mínima para autoridade e, quando finalmente for bater as botas, planeja partir dando risada."#,
    strength_name: "Defiance",
    strength_name_pt: "Desafio",
    strength: r#"Defiance is your life’s blood, the pillar of your soul."#,
    strength_pt: r#"O Desafio é o sangue que corre nas suas veias, o pilar da sua alma."#,
    weakness_name: "Selfishness",
    weakness_name_pt: "Egoísmo",
    weakness: r#"Your problem, though, is Selfishness. So dedicated are you to your own individuality that you’ll ignore the concerns of other folks. Before you can become a valuable part of your society (much less a candidate for transcendence), you’ll have to learn to let go of your own whims and take the bigger picture into account."#,
    weakness_pt: r#"Seu problema, no entanto, é o Egoísmo. Tão dedicado você é à sua própria individualidade que ignora as preocupações alheias. Antes de se tornar uma parte valorosa da sua sociedade (quanto mais um candidato à transcendência), terá de aprender a abrir mão dos seus caprichos e levar o quadro geral em consideração."#,
    willpower_regain: r#"Regain Willpower whenever you manage to score a victory against the forces of oppressive authority."#,
    willpower_regain_pt: r#"Recupere Força de Vontade sempre que conseguir uma vitória contra as forças da autoridade opressora."#,
};

pub const ARCHETYPE_SENSUALIST: ArchetypeDefinition = ArchetypeDefinition {
    id: "sensualist",
    name: "Sensualist",
    name_pt: "Sensualista",
    aliases: &["sensualist", "sensualista", "bon vivant", "hedonista"],
    page_ref: "M20, p. 272",
    description: r#"Sensation is a drug to you. The jittery rush of adrenaline, the caress of this week’s lover on your skin, the raspy pull of a brush through messy hair, even the dull burn of a broken heart – you chase those dragons with bright abandon. The obvious choice for an Ecstatic mage, this compulsion finds new expressions in the Art of magick. The first thing you learn in each Sphere, after all, is perception. For someone like you, that’s all the reason in the world you need in order to learn each Sphere and savor its sensations."#,
    description_pt: r#"Sensação é uma droga para você. A descarga frenética de adrenalina, a carícia do amante da semana na sua pele, a puxada áspera de uma escova pelo cabelo desgrenhado, até a dor surda de um coração partido — você persegue essas sensações com entrega desenfreada. Escolha óbvia para um mago Estático (Culto do Êxtase), essa compulsão encontra novas expressões na Arte da mágica. Afinal, a primeira coisa que você aprende em cada Esfera é percepção. Para alguém como você, esse é todo o motivo de que precisa para aprender cada Esfera e saborear suas sensações."#,
    strength_name: "Receptiveness",
    strength_name_pt: "Receptividade",
    strength: r#"Receptiveness is your gift. Your devotion to sensuality inspires you to challenge yourself. You’re not shy about sharing, either, and your delight in even the most painful stimulations can get other people to explore them with you too."#,
    strength_pt: r#"Receptividade é o seu dom. Sua devoção à sensualidade o inspira a desafiar a si mesmo. Você não tem vergonha de compartilhar suas experiências, e seu prazer até mesmo nos estímulos mais intensos pode levar outras pessoas a explorá-los com você."#,
    weakness_name: "Hedonism",
    weakness_name_pt: "Hedonismo",
    weakness: r#"Hedonism is your curse. Those indulgences can drive you to do stupid, reckless, selfish things. You tend to haul other folks along with you for the ride, and although you might cherish the resulting agonies, your companions are less likely to appreciate them… or you… after a few disasters."#,
    weakness_pt: r#"O Hedonismo é a sua maldição. Essas indulgências podem levá-lo a fazer coisas tolas, imprudentes e egoístas. Você tende a arrastar outras pessoas junto na sua jornada e, embora você possa apreciar as agonias resultantes, seus companheiros dificilmente as apreciarão… ou a você… após alguns desastres."#,
    willpower_regain: r#"Regain Willpower when you get a chance to revel in some potent sensual experience… especially if you get to share with someone else. (Novelty, though, is key; when you do the same thing over and over just to get a rush, that rush fades to boredom in no time.)"#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando tiver a chance de se deliciar em alguma experiência sensorial potente… especialmente se puder compartilhá-la com outra pessoa. (Novidade, contudo, é a chave; quando você repete a mesma coisa várias vezes só pela euforia, o entusiasmo se esvai no tédio num instante)."#,
};

pub const ARCHETYPE_SURVIVOR: ArchetypeDefinition = ArchetypeDefinition {
    id: "survivor",
    name: "Survivor",
    name_pt: "Sobrevivente",
    aliases: &["survivor", "sobrevivente"],
    page_ref: "M20, p. 272",
    description: r#"Nothing stops you. Hardened by previous ordeals, you’ve developed a sense of self-preservation that keeps you going when lesser souls surrender to the odds. You’ve got little patience or sympathy for people who won’t do whatever it takes to endure hardship. “Shut up and step up or get the fuck out of my way” is your motto. Other folks might not make it through, but their weakness won’t lead to your death."#,
    description_pt: r#"Nada detém você. Endurecido por provações anteriores, você desenvolveu um instinto de autopreservação que o mantém em movimento quando almas mais fracas se rendem às probabilidades. Você tem pouca paciência ou simpatia por pessoas que não fazem o que for necessário para suportar a adversidade. 'Cale a boca e dê um jeito, ou saia da porra do meu caminho' é o seu lema. Outros podem não sobreviver, mas a fraqueza deles não custará a sua vida."#,
    strength_name: "Perseverance",
    strength_name_pt: "Perseverança",
    strength: r#"Perseverance is your defining trait. You do what you need to do with what you have to work with. Obstacles are just logistics, and excuses never healed a broken leg."#,
    strength_pt: r#"Perseverança é o seu traço definidor. Você faz o que precisa ser feito com os recursos que tem em mãos. Obstáculos são mera logística, e desculpas nunca curaram uma perna quebrada."#,
    weakness_name: "Callousness",
    weakness_name_pt: "Insensibilidade",
    weakness: r#"Your weakness, though, is a profound Callousness. You’ve shut down so much in order to keep going that folks often wonder if you feel anything at all."#,
    weakness_pt: r#"Sua fraqueza, contudo, é uma profunda Insensibilidade. Você bloqueou tanto seus sentimentos para continuar em frente que as pessoas frequentemente se perguntam se você sente alguma coisa."#,
    willpower_regain: r#"Regain Willpower whenever you survive a difficult situation through stubborn cunning and a refusal to give in."#,
    willpower_regain_pt: r#"Recupere Força de Vontade sempre que sobreviver a uma situação difícil por meio de astúcia obstinada e uma recusa ferrenha em se render."#,
};

pub const ARCHETYPE_TRADITIONALIST: ArchetypeDefinition = ArchetypeDefinition {
    id: "traditionalist",
    name: "Traditionalist",
    name_pt: "Tradicionalista",
    aliases: &["traditionalist", "tradicionalista"],
    page_ref: "M20, p. 272",
    description: r#"Devoted to tradition, you preserve the legacy of bygone wisdom. New ideas are shaky, untested by time, and thus flawed. The Old Ways often are best. They were good enough for our ancestors, and they’ll be good enough for future generations too."#,
    description_pt: r#"Devoto à tradição, você preserva o legado da sabedoria ancestral. Ideias novas são frágeis, não foram testadas pelo tempo e, portanto, contêm falhas. Os Costumes Antigos frequentemente são os melhores. Foram bons o suficiente para os nossos ancestrais e serão bons o suficiente para as gerações futuras também."#,
    strength_name: "Consistency",
    strength_name_pt: "Consistência",
    strength: r#"Your Consistency is a virtue. Especially in an age like ours, in which novelty defines mainstream culture, it’s good to have someone as authentic and dependable as you."#,
    strength_pt: r#"Sua Consistência é uma virtude. Especialmente em uma era como a nossa, na qual a novidade define a cultura comum, é bom ter alguém tão autêntico e confiável quanto você."#,
    weakness_name: "Rigidity",
    weakness_name_pt: "Rigidez",
    weakness: r#"Rigidity, though, can destroy you. Stasis tends to lead to Entropy, especially where mages are concerned. Though your dedication to tradition is admirable enough, you’ll need to learn some flexibility if you want to adapt in this age and eventually transcend it."#,
    weakness_pt: r#"A Rigidez, no entanto, pode destruí-lo. A Estagnação tende a levar à Entropia, especialmente no que concerne aos magos. Embora sua dedicação à tradição seja admirável, você precisará aprender um pouco de flexibilidade se quiser se adaptar a esta era e eventualmente transcendê-la."#,
    willpower_regain: r#"Regain Willpower when your efforts prove the value of old-fashioned ways."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando seus esforços provarem o valor dos costumes antigos."#,
};

pub const ARCHETYPE_TRICKSTER: ArchetypeDefinition = ArchetypeDefinition {
    id: "trickster",
    name: "Trickster",
    name_pt: "Trapaceiro",
    aliases: &["trickster", "trapaceiro"],
    page_ref: "M20, pp. 272-273",
    description: r#"The world is your plaything. People are your toys. A Lord or Lady of Misrule, you work an eternal scam against the dull background of everyday existence. Unlike the Contrary, you’re less interested in political subversion than in personal gratification. It’s not that you don’t care about your companions – you do. Still, when life is one big joke, it’s better to be the setup than the punchline."#,
    description_pt: r#"O mundo é o seu playground. As pessoas são seus brinquedos. Um Senhor ou Senhora do Caos Festivo, você opera um golpe eterno contra o pano de fundo monótono da existência cotidiana. Diferente do Contrário, você está menos interessado em subversão política do que em satisfação pessoal. Não é que você não se importe com seus companheiros — você se importa. Ainda assim, quando a vida é uma piada sem fim, é muito melhor contar a piada do que ser o alvo do riso."#,
    strength_name: "Cleverness",
    strength_name_pt: "Esperteza",
    strength: r#"Cleverness is your claim to fame. Your tricks display considerable flair. Unbound by typical concerns, you upend the order of gods and kings. Nothing holds you back except the few scruples you might actually possess."#,
    strength_pt: r#"Esperteza é a sua marca registrada. Seus truques exibem um talento considerável. Livre das preocupações comuns, você subverte a ordem de deuses e reis. Nada o detém, exceto os poucos escrúpulos que você porventura possua."#,
    weakness_name: "Untrustworthiness",
    weakness_name_pt: "Não Confiável",
    weakness: r#"Untrustworthiness is your crown of thorns. Charming as you might be, there’s an edge to you that smart people perceive. Your own tricks often backfire on you too, leaving you with less than you’d had at the beginning of that scheme. If and when you want something better for yourself, you’ll need to know when to stop laughing and start taking ownership of your actions and their consequences."#,
    weakness_pt: r#"A falta de confiabilidade é a sua coroa de espinhos. Por mais encantador que seja, há um tom ardiloso em você que as pessoas inteligentes percebem. Seus próprios golpes frequentemente saem pela culatra, deixando-o com menos do que tinha no início do plano. Se e quando desejar algo melhor para si mesmo, precisará saber a hora de parar de rir e começar a assumir a responsabilidade por suas ações e suas consequências."#,
    willpower_regain: r#"Regain Willpower when your clever plans result in a big gain for you and your companions or a big loss for your enemies."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando seus planos engenhosos resultarem em um grande ganho para você e seus companheiros ou em uma grande perda para seus inimigos."#,
};

pub const ARCHETYPE_VISIONARY: ArchetypeDefinition = ArchetypeDefinition {
    id: "visionary",
    name: "Visionary",
    name_pt: "Visionário",
    aliases: &["visionary", "visionario"],
    page_ref: "M20, p. 273",
    description: r#"Beyond typical perception, there’s something more. You see it, though few other people do. Even by mage standards, you’re a radical seer. Though you spend lots of time and energy chasing your vision, that deeper goal is worth whatever its search might cost."#,
    description_pt: r#"Além da percepção comum, há algo a mais. Você enxerga isso, embora poucas pessoas consigam. Mesmo para os padrões dos magos, você é um vidente radical. Embora gaste muito tempo e energia perseguindo sua visão, esse objetivo mais profundo vale o que quer que a busca venha a custar."#,
    strength_name: "Inspiration",
    strength_name_pt: "Inspiração",
    strength: r#"Inspiration guides your steps, inspiring you to see what others can’t or won’t perceive. You visualize some great ideal – the return of your people’s glory, a world-changing technology, a path toward Earth’s redemption, that sort of thing – with the passion of a true believer."#,
    strength_pt: r#"Inspiração guia os seus passos, inspirando-o a ver o que outros não conseguem ou se recusam a perceber. Você vislumbra algum grande ideal — o retorno da glória do seu povo, uma tecnologia transformadora do mundo, um caminho para a redenção da Terra, esse tipo de coisa — com a paixão de um verdadeiro devoto."#,
    weakness_name: "Pride",
    weakness_name_pt: "Orgulho",
    weakness: r#"Your weakness, though, is Pride in your stubborn search for truth. To Ascend, you must learn humility, seeking visions that other people can enjoy as well."#,
    weakness_pt: r#"Sua fraqueza, no entanto, é o Orgulho em sua teimosa busca pela verdade. Para Ascender, você precisa aprender a humildade, buscando visões que outras pessoas possam desfrutar também."#,
    willpower_regain: r#"Regain Willpower when you convince folks that your vision is worth their devotion too, or when your actions turn your vision into practical reality."#,
    willpower_regain_pt: r#"Recupere Força de Vontade quando convencer as pessoas de que sua visão é digna da devoção delas também, ou quando suas ações transformarem sua visão em realidade prática."#,
};

/// All 20 canonical Personality Archetypes from M20 (pp. 267-273)
pub const ALL_ARCHETYPES: &[ArchetypeDefinition; 20] = &[
    ARCHETYPE_ACTIVIST,
    ARCHETYPE_BENEFACTOR,
    ARCHETYPE_CONTRARY,
    ARCHETYPE_CRUSADER,
    ARCHETYPE_HACKER,
    ARCHETYPE_IDEALIST,
    ARCHETYPE_INNOVATOR,
    ARCHETYPE_KID,
    ARCHETYPE_LONER,
    ARCHETYPE_MACHINE,
    ARCHETYPE_MAD_SCIENTIST,
    ARCHETYPE_MARTYR,
    ARCHETYPE_MONSTER,
    ARCHETYPE_PROPHET,
    ARCHETYPE_ROGUE,
    ARCHETYPE_SENSUALIST,
    ARCHETYPE_SURVIVOR,
    ARCHETYPE_TRADITIONALIST,
    ARCHETYPE_TRICKSTER,
    ARCHETYPE_VISIONARY,
];

/// Helper function to normalize query strings for case-insensitive and accent-insensitive matching
fn normalize_query(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'ã' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'õ' | 'ô' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            _ => c,
        })
        .collect()
}

/// Look up an archetype by ID, English name, Portuguese name, or alias
pub fn find_archetype(query: &str) -> Option<&'static ArchetypeDefinition> {
    let q = normalize_query(query);
    if q.is_empty() {
        return None;
    }

    // Direct ID or name match first
    for arch in ALL_ARCHETYPES.iter() {
        if normalize_query(arch.id) == q
            || normalize_query(arch.name) == q
            || normalize_query(arch.name_pt) == q
        {
            return Some(arch);
        }
        for &alias in arch.aliases {
            if normalize_query(alias) == q {
                return Some(arch);
            }
        }
    }

    // Substring / partial match fallback
    for arch in ALL_ARCHETYPES.iter() {
        let norm_name = normalize_query(arch.name);
        let norm_pt = normalize_query(arch.name_pt);
        if norm_name.contains(&q) || norm_pt.contains(&q) || q.contains(&norm_name) || q.contains(&norm_pt) {
            return Some(arch);
        }
    }

    None
}

/// Retrieve all archetype names localized to the requested language
pub fn get_archetype_names(lang: Language) -> Vec<&'static str> {
    ALL_ARCHETYPES.iter().map(|a| a.name(lang)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_archetypes_count_and_uniqueness() {
        assert_eq!(ALL_ARCHETYPES.len(), 20);
        let mut ids = std::collections::HashSet::new();
        for arch in ALL_ARCHETYPES.iter() {
            assert!(!arch.id.is_empty());
            assert!(!arch.name.is_empty());
            assert!(!arch.name_pt.is_empty());
            assert!(!arch.description.is_empty());
            assert!(!arch.description_pt.is_empty());
            assert!(!arch.strength_name.is_empty());
            assert!(!arch.strength.is_empty());
            assert!(!arch.weakness_name.is_empty());
            assert!(!arch.weakness.is_empty());
            assert!(!arch.willpower_regain.is_empty());
            assert!(!arch.willpower_regain_pt.is_empty());
            assert!(ids.insert(arch.id), "Duplicate ID detected: {}", arch.id);
        }
    }

    #[test]
    fn test_find_archetype_exact_and_case_insensitive() {
        assert_eq!(find_archetype("Activist").unwrap().id, "activist");
        assert_eq!(find_archetype("ativista").unwrap().id, "activist");
        assert_eq!(find_archetype("TRICKSTER").unwrap().id, "trickster");
        assert_eq!(find_archetype("trapaceiro").unwrap().id, "trickster");
        assert_eq!(find_archetype("cientista louco").unwrap().id, "mad_scientist");
        assert_eq!(find_archetype("Mad Scientist").unwrap().id, "mad_scientist");
        assert_eq!(find_archetype("rogue").unwrap().id, "rogue");
        assert_eq!(find_archetype("rebelde").unwrap().id, "rogue");
        assert_eq!(find_archetype("malandro").unwrap().id, "rogue");
    }

    #[test]
    fn test_archetype_theory_rules_integrity() {
        assert_eq!(ARCHETYPE_THEORY_RULES.id, "theory_nature_demeanor");
        assert!(ARCHETYPE_THEORY_RULES.content.contains("Jungian terms"));
        assert!(ARCHETYPE_THEORY_RULES.content_pt.contains("termos junguianos"));
        assert!(ARCHETYPE_THEORY_RULES.content.contains("one to three Willpower points"));
        assert!(ARCHETYPE_THEORY_RULES.content_pt.contains("um a três pontos de Força de Vontade"));
    }

    #[test]
    fn test_archetype_bilingual_accessors() {
        let arch = &ARCHETYPE_SURVIVOR;
        assert_eq!(arch.name(Language::EnUs), "Survivor");
        assert_eq!(arch.name(Language::PtBr), "Sobrevivente");
        assert_eq!(arch.strength_name(Language::EnUs), "Perseverance");
        assert_eq!(arch.strength_name(Language::PtBr), "Perseverança");
        assert_eq!(arch.weakness_name(Language::EnUs), "Callousness");
        assert_eq!(arch.weakness_name(Language::PtBr), "Insensibilidade");
        assert!(arch.willpower_regain(Language::PtBr).contains("Recupere Força de Vontade"));
    }
}
