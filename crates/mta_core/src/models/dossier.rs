use serde::{Deserialize, Serialize};
use super::traits::is_empty_str;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuizQuestionEntry {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub title: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub prompt: String,
    #[serde(default)]
    pub answer: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub category: String, // "character" ou "player"
}

pub fn default_quiz_questions() -> Vec<QuizQuestionEntry> {
    vec![
        // === SEÇÃO 1: PERGUNTAS PARA O PERSONAGEM ===
        QuizQuestionEntry {
            id: "q_char_age".to_string(),
            title: "Qual É A Sua Idade?".to_string(),
            prompt: "Quantos anos têm o seu personagem? Por quanto tempo ele estudou mágika? A sua aparência reflete a sua idade? Que acontecimentos foram importantes para o seu personagem (se a crônica se passa em 1996 e o seu personagem tem 25 anos, acontecimentos como a queda do Muro de Berlim terão efeito direto; se tem 40 anos, a Guerra do Vietnã terá influência marcante)?".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_different".to_string(),
            title: "Quando Você Percebeu Que Era... Diferente?".to_string(),
            prompt: "A maioria dos magos tem infâncias estranhas com eventos inexplicáveis e coincidências bizarras. Esses acontecimentos passaram despercebidos ou tiveram consequências tremendas? Uma infância feliz pode induzir no mago um sentimento de dever com os menos favorecidos, enquanto outro que passou sua infância marginalizado pode ter dificuldade de se relacionar com os outros.".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_skills".to_string(),
            title: "Como Você Desenvolveu Suas Habilidades?".to_string(),
            prompt: "Ninguém aparece do nada, completo com perícias e um lugar na sociedade. Quem era você? Onde você cresceu? Como você aprendeu aquilo que sabe? Estas perguntas dão mais profundidade e sugerem linhas de enredo que o Narrador possa tecer na crônica.".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_important_people".to_string(),
            title: "Quem É Importante Para Você?".to_string(),
            prompt: "Nenhum homem é uma ilha. Ele tem amigos superficiais ou um pequeno círculo bem unido? Como se relaciona com sua família? Alguma dessas pessoas sabe sobre suas habilidades de balançar os pilares do Céu? Como reagem a isso? O seu Despertar o afastou de todos que eram importantes? Quem é importante para ele agora?".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_first_magick".to_string(),
            title: "Quando Você Encontrou A Mágika Pela Primeira Vez?".to_string(),
            prompt: "Os humanos têm uma grande capacidade de ignorar ou desprezar coisas que não se encaixam nos seus modelos de mundo. Que acontecimentos superaram essa habilidade? Como descobriu que havia mistérios que não podiam ser solucionados? Ficou com medo, surpreso, louco pelo poder ou teve um colapso? Ou simplesmente nunca perdeu a habilidade infantil de acreditar em tudo?".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_mentor".to_string(),
            title: "Quem Era O Seu Mentor?".to_string(),
            prompt: "Magos da Tradição normalmente começam com algum tipo de mentor. Como conheceu essa pessoa? Você o procurou ou ele veio até você? Ele foi atencioso ou duro? Ele explicou tudo, ou simplesmente fez perguntas e observou? O mentor agiu como professor, pai, irmão mais velho ou força fundamental na personalidade?".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_cabala".to_string(),
            title: "Como Você Conheceu Os Outros na Sua Cabala?".to_string(),
            prompt: "A cabala normalmente se refere aos personagens dos outros jogadores. Como você os conheceu e interagiu com eles? Preveniram alguma catástrofe na Teia, salvaram alguém de um ataque Nefandi ou foram reunidos por uma causa maior?".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_mundane_life".to_string(),
            title: "Você Mantém Uma Vida Comum?".to_string(),
            prompt: "Você tem uma 'identidade secreta'? Você continua a interagir com os Adormecidos, escondendo os aspectos sobrenaturais da sua existência, ou você deixou sua vida antiga para trás?".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },

        // === SEÇÃO 2: DICAS & PERGUNTAS PARA OS JOGADORES SOBRE O CAMINHO ===
        QuizQuestionEntry {
            id: "q_player_what_to_do".to_string(),
            title: "O Que Você, o Jogador, Quer Fazer?".to_string(),
            prompt: "As motivações são muito importantes, tanto as suas quanto as do seu personagem. Que tipo de pessoa você quer representar (cientista louco, mago enigmático, socialite)? Que tipo de coisas você gostaria de realizar no jogo (alimentar famintos, lutar, ficar rico, vingar alguém)? Lembre-se de que Mago é sobre encontrar verdades maiores.".to_string(),
            answer: String::new(),
            category: "player".to_string(),
        },
        QuizQuestionEntry {
            id: "q_player_destiny_pursued".to_string(),
            title: "Que Destino Ele Persegue?".to_string(),
            prompt: "O que o místiko vê no seu destino? Morrer por um bem maior? Derrubar a Tecnocracia? Acabar com o mal, aperfeiçoar seu eu interior ou escrever os evangelhos do século XXI? Que visões guiam a busca do destino?".to_string(),
            answer: String::new(),
            category: "player".to_string(),
        },
        QuizQuestionEntry {
            id: "q_player_destiny_view".to_string(),
            title: "Como o Mago Vê o Seu Destino?".to_string(),
            prompt: "A maioria dos magos, especialmente os novos, não querem pensar sobre o fim do caminho. No entanto, todo mago tem alguma ideia do que o destino reservou para ele. Isso o assusta ou intriga? O que ele sente sobre isso, e o que fará para persegui-lo... ou evitá-lo?".to_string(),
            answer: String::new(),
            category: "player".to_string(),
        },
        QuizQuestionEntry {
            id: "q_player_avatar_nature".to_string(),
            title: "Como é o Avatar do Personagem?".to_string(),
            prompt: "O que está nos planos do Avatar (tornar-se uno com todos, devolver a mágika ao mundo, eliminar preconceitos, completar negócios inacabados de vidas passadas)? Como ele aparece (um amigo imaginário, um surto de inspiração, um anjo com as mãos sangrentas)?".to_string(),
            answer: String::new(),
            category: "player".to_string(),
        },
        QuizQuestionEntry {
            id: "q_player_avatar_relation".to_string(),
            title: "Como o Mago se Relaciona Com o seu Avatar?".to_string(),
            prompt: "O seu mago está em conflito com o seu eu mágiko? Como eles se relacionam? O Avatar atormenta o místiko com poder/conhecimento, senta num canto ou o arrasta através das Procuras até abrir seus olhos ou ficar louco? O mago quer ser um mago ou preferiria voltar à vida antiga? Lembre-se: harmonia perfeita é tediosa!".to_string(),
            answer: String::new(),
            category: "player".to_string(),
        },
        QuizQuestionEntry {
            id: "q_player_conflicts".to_string(),
            title: "Que Conflitos Podem Surgir ao Longo do Caminho?".to_string(),
            prompt: "O Caminho da Ascensão verdadeira nunca foi suave. Que tipos de distrações podem tirar o mago do seu caminho (amor verdadeiro, desilusões, vingança, traição, insanidade, orgulho)? Descobrindo isso, você terá uma ideia de como seu personagem reagirá quando tais coisas acontecerem.".to_string(),
            answer: String::new(),
            category: "player".to_string(),
        },
    ]
}

pub fn deserialize_quiz_entries<'de, D>(deserializer: D) -> Result<Vec<QuizQuestionEntry>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct PartialQuizEntry {
        id: String,
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        prompt: Option<String>,
        #[serde(default)]
        answer: String,
        #[serde(default)]
        category: Option<String>,
    }

    let parsed_entries = Vec::<PartialQuizEntry>::deserialize(deserializer)?;
    let mut default_entries = default_quiz_questions();

    for parsed in parsed_entries {
        if let Some(existing) = default_entries.iter_mut().find(|e| e.id == parsed.id) {
            existing.answer = parsed.answer;
            if let Some(t) = parsed.title { if !t.is_empty() { existing.title = t; } }
            if let Some(p) = parsed.prompt { if !p.is_empty() { existing.prompt = p; } }
            if let Some(c) = parsed.category { if !c.is_empty() { existing.category = c; } }
        } else {
            default_entries.push(QuizQuestionEntry {
                id: parsed.id,
                title: parsed.title.unwrap_or_default(),
                prompt: parsed.prompt.unwrap_or_default(),
                answer: parsed.answer,
                category: parsed.category.unwrap_or_else(|| "custom".to_string()),
            });
        }
    }

    Ok(default_entries)
}

pub fn serialize_compact_quiz_entries<S>(entries: &Vec<QuizQuestionEntry>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let answered: Vec<&QuizQuestionEntry> = entries.iter().filter(|e| !e.answer.trim().is_empty()).collect();
    let mut seq = serializer.serialize_seq(Some(answered.len()))?;
    for entry in answered {
        seq.serialize_element(entry)?;
    }
    seq.end()
}

pub fn is_default_quiz(q: &CharacterQuizData) -> bool {
    q.entries.iter().all(|e| e.answer.trim().is_empty())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CharacterQuizData {
    #[serde(default = "default_quiz_questions", deserialize_with = "deserialize_quiz_entries", serialize_with = "serialize_compact_quiz_entries")]
    pub entries: Vec<QuizQuestionEntry>,
}

impl Default for CharacterQuizData {
    fn default() -> Self {
        Self {
            entries: default_quiz_questions(),
        }
    }
}
