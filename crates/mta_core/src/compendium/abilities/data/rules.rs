//! Artigos Canônicos de Regras Opcionais e Teoria de Habilidades de M20.
//!
//! Textos completos sobre Controle Corporal, Habilidades Mínimas, O Artesão Habilidoso,
//! Perícias Profissionais/Conhecimentos Especialistas e Regras de Ciência Inspirada.

use super::super::models::AbilityTheoryArticle;

pub const RULE_BODY_CONTROL: AbilityTheoryArticle = AbilityTheoryArticle {
    id: "rule_body_control",
    title: "Optional Rule: Body Control",
    title_pt: "Regra Opcional: Controle Corporal",
    page_ref: "M20, pp. 278-279",
    content: r#"Several esoteric fields – notably yoga, t’ai chi, qigong and other advanced trance-state disciplines – grant a considerable amount of body control to a student of that field. And though casual practitioners rarely achieve such feats, devotees can accomplish apparently superhuman acts. This optional rule allows for such feats even without the use of magick.

Assume that a character with an Esoterica specialty in Yoga or Body Control, plus at least the minimum dots in Esoterica, can use the following feats, with only one effect at a time:
- **Calm Focus:** Intelligence + Esoterica (diff 7) after two turns of focus.
- **Stress Focus:** Wits + Esoterica (diff 8) after two turns of focus.

### Feats by Minimum Dots & Successes:
- **1 Dot / 1 Success - Ignore Distraction:** Screen out sensory distractions during study, casting, or meditation.
- **1 Dot / 2 Successes - Moderated Sleep:** Fall asleep peacefully anywhere on demand; enter a resting trance.
- **2 Dots / 2 Successes - Hold Breath:** Hold breath for two minutes per dot of Stamina, plus one minute per success.
- **2 Dots / 3 Successes - Tolerate Temperature:** Suffer no penalties from extreme heat/cold; reduce fire/frost damage dice by 1.
- **3 Dots / 3 Successes - Transcend Pain:** Reduce wound or illness dice-pool penalties by one for the scene.
- **3 Dots / 4 Successes - Slow Bleeding & Breathing:** Regulate vitals to avoid shock and stabilize mortal wounds.
- **4 Dots / 4 Successes - Digestive Control:** Slow toxins or poisons in bloodstream; delay effects for 1 hour per success.
- **4 Dots / 5 Successes - Death Trance:** Voluntary suspended animation. Heart rate and breathing become imperceptible. Bleeding halts, poisons freeze in place. Appears medically dead for one day per success."#,
    content_pt: r#"Vários campos esotéricos – notadamente ioga, tai chi chuan, qigong e outras disciplinas de transe avançado – concedem considerável controle somático aos seus praticantes. Embora praticantes casuais raramente atinjam tais proezas, devotos dedicados alcançam feitos quase sobre-humanos. Esta regra opcional permite tais feitos mesmo sem o uso de mágika!

Um personagem com especialização de Esotérica em Ioga ou Controle Corporal, tendo os pontos mínimos requeridos em Esotérica, pode acionar um feito por vez:
- **Sob Calma:** Inteligência + Esotérica (dif 7) após dois turnos de concentração.
- **Sob Estresse:** Raciocínio + Esotérica (dif 8) após dois turnos de concentração.

### Proezas por Pontos Mínimos e Sucessos:
- **1 Ponto / 1 Sucesso - Ignorar Distrações:** Isola barulhos e distrações sensoriais durante estudos, ritos ou meditação.
- **1 Ponto / 2 Sucessos - Sono Moderado:** Adormece em paz sob qualquer condição; entra em transe reparador instantâneo.
- **2 Pontos / 2 Sucessos - Prender a Respiração:** Prende o fôlego por 2 minutos por ponto de Vigor, mais 1 minuto por sucesso.
- **2 Pontos / 3 Sucessos - Tolerar Temperatura:** Imune a penalidades de frio/calor extremos; reduz dano de fogo/gelo em 1 dado.
- **3 Pontos / 3 Sucessos - Transcender a Dor:** Reduz em 1 ponto as penalidades de ferimentos ou doença pelo restante da cena.
- **3 Pontos / 4 Sucessos - Conter Sangramento e Respiração:** Estabiliza hemorragias e evita choque hipovolêmico sem socorro médico.
- **4 Pontos / 4 Sucessos - Controle Digestivo:** Desacelera a circulação de venenos; atrasa os efeitos tóxicos por 1 hora por sucesso.
- **4 Pontos / 5 Sucessos - Transe da Morte:** Coma voluntário em animação suspensa. Batimentos e respiração tornam-se imperceptíveis, sangramentos param. Parece clinicamente morto por 1 dia por sucesso."#,
};

pub const RULE_MINIMUM_ABILITIES: AbilityTheoryArticle = AbilityTheoryArticle {
    id: "rule_minimum_abilities",
    title: "Optional Rule: Minimum Abilities",
    title_pt: "Regra Opcional: Habilidades Mínimas",
    page_ref: "M20, p. 278",
    content: r#"If you focus powerful magick through a specific Ability (as detailed in Chapter Ten: Focus and the Arts), it's a good idea to be extremely skilled with that Ability. As an optional rule, the Storyteller may insist that a character needs to have at least one dot in the focus-based Ability for every dot he has in the Sphere Rank of an Effect focused through that Ability.

If, for example, Autumn uses artwork (and thus, the Art Talent) as a focus instrument, then she needs to have at least one dot in Art for every dot in the Spheres she uses artwork to focus. In short, she can't use artwork to focus a Forces 3 / Prime 2 Effect unless she has at least three dots in Art.

This option demands a certain 'minimum competency level' on focus-related Abilities. We especially suggest this rule for:
- **Computer** (channeling Data or Correspondence)
- **Do & Martial Arts** (channeling Forces or Life in hand-to-hand combat)
- **Science & Technology** (channeling Matter, Forces, or Prime through hypertech Devices)."#,
    content_pt: r#"Se um mago canaliza mágika poderosa através de uma Habilidade específica (conforme detalhado no Capítulo 10: Foco e Artes), faz todo o sentido que ele seja extremamente competente nessa Habilidade. Como regra opcional, o Narrador pode exigir que o mago possua ao menos um ponto na Habilidade usada como foco para cada nível de Esfera envolvido no Feitiço.

Por exemplo, se Autumn utiliza pinturas (o Talento Arte) como seu instrumento de foco, ela precisa ter no mínimo 3 pontos em Arte para conseguir canalizar um Efeito de Forças 3 / Primórdio 2 através de seus pincéis.

Essa regra garante coerência dramática e competência técnica. Recomendamos seu uso prioritário para:
- **Computador** (ao canalizar Dados, Correspondência ou Rede Digital)
- **Do & Artes Marciais** (ao canalizar Forças, Vida ou Mente em golpes corporais)
- **Ciência & Tecnologia** (ao canalizar Matéria, Forças ou Primórdio em Dispositivos hipertecnológicos)."#,
};

pub const RULE_WELL_SKILLED_CRAFTSMAN: AbilityTheoryArticle = AbilityTheoryArticle {
    id: "rule_well_skilled_craftsman",
    title: "Optional Rule: The Well-Skilled Craftsman",
    title_pt: "Regra Opcional: O Artesão Habilidoso",
    page_ref: "M20, p. 279",
    content: r#"Mages excel at many different fields of endeavor and focus their Arts through quite a few of them. Rather than buying multiple separate Abilities that share identical core mechanics, this optional rule allows your character to broaden their repertoire economically.

When purchasing a broad Ability (such as Crafts, Art, Athletics, Firearms, Melee, Academics, or Science), you define a single primary specialty. Once you achieve four dots (••••) in that Ability, you can purchase additional distinct specialties within that same Trait for only **4 Experience Points (or 4 Freebie Points)** each.

### Example:
Spider Chase has Crafts 4 with a primary specialty in Leatherwork. She wants her character to also be competent at Tailoring and Vehicle Repair. Under this rule, she can buy both Tailoring and Vehicle Repair as secondary craft specialties for 4 XP each (8 XP total), using her 4-dot Crafts pool whenever working in those disciplines."#,
    content_pt: r#"Magos dominam múltiplos campos de expressão e focam suas Artes em vários ofícios interligados. Em vez de obrigar o jogador a recomprar do zero Habilidades afins que compartilham a mesma mecânica base, a regra do Artesão Habilidoso permite expandir o repertório de forma econômica e elegante.

Ao adquirir uma Habilidade ampla (como Ofícios, Arte, Esportes, Armas de Fogo, Armas Brancas, Acadêmicos ou Ciência), o jogador define uma especialização principal. Ao alcançar quatro pontos (••••) nessa Habilidade, ele pode comprar especializações adicionais completas na mesma Habilidade por apenas **4 Pontos de Experiência (ou 4 Pontos de Bônus)** cada!

### Exemplo Prático:
Spider Chase tem Ofícios 4 com especialização em Trabalho em Couro. Ela deseja que sua maga também seja competente em Alfaiataria e Conserto de Veículos. Sob esta regra, ela compra Alfaiataria e Mecânica por 4 XP cada (8 XP no total), rolando sua parada completa de Ofícios 4 sempre que operar nessas áreas."#,
};

pub const RULE_PROFESSIONAL_EXPERT: AbilityTheoryArticle = AbilityTheoryArticle {
    id: "rule_professional_expert",
    title: "Optional Rule: Professional Skills & Expert Knowledges",
    title_pt: "Regra Opcional: Perícias Profissionais & Conhecimentos Especialistas",
    page_ref: "M20, p. 277",
    content: r#"Human beings master thousands of diverse disciplines that cannot all fit into a standard character sheet. This catch-all optional rule creates custom professional and vocational fields:

- **Professional Skills:** Vocational crafts refined through hands-on practice (e.g. Cooking, Brewing, Gambling, Glassblowing, Tanning, Scuba Diving).
- **Expert Knowledges:** Intellectual disciplines honed by scholarly immersion (e.g. Journalism, Criminology, Theology, Heraldry, Forensics, Urban Planning).

Each Professional or Expert trait is rated from 1 to 5 dots and purchased independently. If the group uses 'The Well-Skilled Craftsman', related sub-branches can be acquired once 4 dots are achieved."#,
    content_pt: r#"A humanidade desenvolve milhares de vocações e saberes práticos que não caberiam em uma única folha de papel. Esta regra guarda-chuva permite criar campos profissionais e técnicos sob medida:

- **Perícias Profissionais:** Atividades manuais e práticas aperfeiçoadas com o ofício diário (ex.: Culinária, Destilaria, Jogos de Azar, Soprador de Vidro, Mergulho Autônomo).
- **Conhecimentos Especialistas:** Saberes intelectuais teóricos aprofundados (ex.: Jornalismo, Criminologia, Teologia, Heráldica, Urbanismo, Perícia Contábil).

Cada Perícia Profissional ou Conhecimento Especialista opera na escala de 1 a 5 bolinhas e é adquirido como uma Habilidade customizada na ficha."#,
};

pub const RULE_SCIENCE_THEORY: AbilityTheoryArticle = AbilityTheoryArticle {
    id: "rule_science_theory",
    title: "SCIENCE!!! Theory & Coincidental Magick",
    title_pt: "CIÊNCIA!!! Teoria e Mágika Coincidente",
    page_ref: "M20, pp. 288-289",
    content: r#"Science is not the enemy of magick – it is the systematic pursuit of provable, repeatable knowledge. Because modern Consensus is steeped in scientific technology, magick that can be explained in scientific jargon is often Coincidental, not Vulgar, in developed areas!

### "Wait – I Can Explain!" (Scientific Rationale Mechanic):
When attempting a borderline magical feat, a technomancer can roll Manipulation or Charisma + Science (difficulty set by the Storyteller based on how plausible the pseudo-science sounds). If successful, and if accompanied by convincing technobabble, the Effect is treated as Coincidental rather than Vulgar!

### The Catch: "There Must Be Some Rational Explanation..."
Inspired Science cannot do the arbitrary impossible. A Progenitor can clone a creature or inject silver nitrate, but cannot simply turn a werewolf into silver with a magic wand wave. If the explanation violates the technomancer's own paradigm or scientific consistency, the Storyteller calls shenanigans: POOF! Vulgar magick and Paradox backfire!"#,
    content_pt: r#"A ciência não é inimiga da magia – ela é a busca metódica por conhecimento comprovável e reprodutível. Como o Consenso moderno é alicerçado na tecnologia científica, a mágika que pode ser explicada em termos científicos plausíveis torna-se Coincidente, e não Vulgar, na maior parte do mundo civilizado!

### "Espere – Eu Posso Explicar!" (Mecânica de Justificativa Científica):
Ao realizar um Feitiço que flerte com o impossível, um tecnomante pode rolar Manipulação ou Carisma + Ciência (dificuldade estipulada pelo Narrador conforme a plausibilidade do jargão técnico). Em caso de sucesso acompanhado de uma explicação científica convincente, o Efeito é considerado Coincidente em vez de Vulgar!

### A Contrapartida: "Tem que Haver uma Explicação Racional..."
A Ciência Inspirada não faz mágika de desenho animado. Um Progenitor pode clonar tecidos ou injetar nitrato de prata, mas não pode simplesmente estalar os dedos e transformar um lobisomem em prata maciça. Se a explicação quebrar o próprio paradigma do cientista ou violar a coerência do experimento: PUF! Mágika Vulgar imediata com choque de Paradoxo!"#,
};

pub const ABILITY_THEORY_RULES: &[AbilityTheoryArticle] = &[
    RULE_BODY_CONTROL,
    RULE_MINIMUM_ABILITIES,
    RULE_WELL_SKILLED_CRAFTSMAN,
    RULE_PROFESSIONAL_EXPERT,
    RULE_SCIENCE_THEORY,
];
