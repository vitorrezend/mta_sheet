use super::Language;

/// Translates Dossier / Quiz question titles
pub fn tr_quiz_title(id: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => match id {
            "q_char_age" => "Qual É A Sua Idade?",
            "q_char_different" => "Quando Você Percebeu Que Era... Diferente?",
            "q_char_skills" => "Como Você Desenvolveu Suas Habilidades?",
            "q_char_important_people" => "Quem É Importante Para Você?",
            "q_char_first_magick" => "Quando Você Encontrou A Mágika Pela Primeira Vez?",
            "q_char_mentor" => "Quem Era O Seu Mentor?",
            "q_char_cabala" => "Como Você Conheceu Os Outros na Sua Cabala?",
            "q_char_mundane_life" => "Você Mantém Uma Vida Comum?",
            "q_player_what_to_do" => "O Que Você, o Jogador, Quer Fazer?",
            "q_player_destiny_pursued" => "Que Destino Ele Persegue?",
            "q_player_destiny_view" => "Como o Mago Vê o Seu Destino?",
            "q_player_avatar_nature" => "Como é o Avatar do Personagem?",
            "q_player_avatar_relation" => "Como o Mago se Relaciona Com o seu Avatar?",
            "q_player_conflicts" => "Que Conflitos Podem Surgir ao Longo do Caminho?",
            _ => id,
        },
        Language::EnUs => match id {
            "q_char_age" => "How Old Are You?",
            "q_char_different" => "When Did You Realize You Were... Different?",
            "q_char_skills" => "How Did You Develop Your Abilities?",
            "q_char_important_people" => "Who Is Important To You?",
            "q_char_first_magick" => "When Did You First Encounter Magick?",
            "q_char_mentor" => "Who Was Your Mentor?",
            "q_char_cabala" => "How Did You Meet The Others in Your Cabal?",
            "q_char_mundane_life" => "Do You Maintain a Mundane Life?",
            "q_player_what_to_do" => "What Do You, The Player, Want To Do?",
            "q_player_destiny_pursued" => "What Destiny Do You Pursue?",
            "q_player_destiny_view" => "How Does The Mage View Their Destiny?",
            "q_player_avatar_nature" => "What Is The Nature of Your Avatar?",
            "q_player_avatar_relation" => "How Do You Relate To Your Avatar?",
            "q_player_conflicts" => "What Conflicts May Arise Along The Way?",
            _ => id,
        },
    }
}

/// Translates Dossier / Quiz question prompts
pub fn tr_quiz_prompt(id: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => match id {
            "q_char_age" => "Quantos anos têm o seu personagem? Por quanto tempo ele estudou mágika? A sua aparência reflete a sua idade? Que acontecimentos foram importantes para o seu personagem (se a crônica se passa em 1996 e o seu personagem tem 25 anos, acontecimentos como a queda do Muro de Berlim terão efeito direto; se tem 40 anos, a Guerra do Vietnã terá influência marcante)?",
            "q_char_different" => "A maioria dos magos tem infâncias estranhas com eventos inexplicáveis e coincidências bizarras. Esses acontecimentos passaram despercebidos ou tiveram consequências tremendas? Uma infância feliz pode induzir no mago um sentimento de dever com os menos favorecidos, enquanto outro que passou sua infância marginalizado pode ter dificuldade de se relacionar com os outros.",
            "q_char_skills" => "Ninguém aparece do nada, completo com perícias e um lugar na sociedade. Quem era você? Onde você cresceu? Como você aprendeu aquilo que sabe? Estas perguntas dão mais profundidade e sugerem linhas de enredo que o Narrador possa tecer na crônica.",
            "q_char_important_people" => "Nenhum homem é uma ilha. Ele tem amigos superficiais ou um pequeno círculo bem unido? Como se relaciona com sua família? Alguma dessas pessoas sabe sobre suas habilidades de balançar os pilares do Céu? Como reagem a isso? O seu Despertar o afastou de todos que eram importantes? Quem é importante para ele agora?",
            "q_char_first_magick" => "Os humanos têm uma grande capacidade de ignorar ou desprezar coisas que não se encaixam nos seus modelos de mundo. Que acontecimentos superaram essa habilidade? Como descobriu que havia mistérios que não podiam ser solucionados? Ficou com medo, surpreso, louco pelo poder ou teve um colapso? Ou simplesmente nunca perdeu a habilidade infantil de acreditar em tudo?",
            "q_char_mentor" => "Magos da Tradição normalmente começam com algum tipo de mentor. Como conheceu essa pessoa? Você o procurou ou ele veio até você? Ele foi atencioso ou duro? Ele explicou tudo, ou simplesmente fez perguntas e observou? O mentor agiu como professor, pai, irmão mais velho ou força fundamental na personalidade?",
            "q_char_cabala" => "A cabala normalmente se refere aos personagens dos outros jogadores. Como você os conheceu e interagiu com eles? Preveniram alguma catástrofe na Teia, salvaram alguém de um ataque Nefandi ou foram reunidos por uma causa maior?",
            "q_char_mundane_life" => "Você tem uma 'identidade secreta'? Você continua a interagir com os Adormecidos, escondendo os aspectos sobrenaturais da sua existência, ou você deixou sua vida antiga para trás?",
            "q_player_what_to_do" => "As motivações são muito importantes, tanto as suas quanto as do seu personagem. Que tipo de pessoa você quer representar (cientista louco, mago enigmático, socialite)? Que tipo de coisas você gostaria de realizar no jogo (alimentar famintos, lutar, ficar rico, vingar alguém)? Lembre-se de que Mago é sobre encontrar verdades maiores.",
            "q_player_destiny_pursued" => "O que o místiko vê no seu destino? Morrer por um bem maior? Derrubar a Tecnocracia? Acabar com o mal, aperfeiçoar seu eu interior ou escrever os evangelhos do século XXI? Que visões guiam a busca do destino?",
            "q_player_destiny_view" => "A maioria dos magos, especialmente os novos, não querem pensar sobre o fim do caminho. No entanto, todo mago tem alguma ideia do que o destino reservou para ele. Isso o assusta ou intriga? O que ele sente sobre isso, e o que fará para persegui-lo... ou evitá-lo?",
            "q_player_avatar_nature" => "O que está nos planos do Avatar (tornar-se uno com todos, devolver a mágika ao mundo, eliminar preconceitos, completar negócios inacabados de vidas passadas)? Como ele aparece (um amigo imaginário, um surto de inspiração, um anjo com as mãos sangrentas)?",
            "q_player_avatar_relation" => "O seu mago está em conflito com o seu eu mágiko? Como eles se relacionam? O Avatar atormenta o místiko com poder/conhecimento, senta num canto ou o arrasta através das Procuras até abrir seus olhos ou ficar louco? O mago quer ser um mago ou preferiria voltar à vida antiga? Lembre-se: harmonia perfeita é tediosa!",
            "q_player_conflicts" => "O Caminho da Ascensão verdadeira nunca foi suave. Que tipos de distrações podem tirar o mago do seu caminho (amor verdadeiro, desilusões, vingança, traição, insanidade, orgulho)? Descobrindo isso, você terá uma ideia de como seu personagem reagirá quando tais coisas acontecerem.",
            _ => id,
        },
        Language::EnUs => match id {
            "q_char_age" => "How old is your character? How long have they studied magick? Does their appearance reflect their age? What historical events shaped their life (e.g. the fall of the Berlin Wall, the Vietnam War, the rise of the Internet)?",
            "q_char_different" => "Most mages have unusual childhoods filled with strange coincidences. Did these go unnoticed or cause profound consequences? A happy upbringing may foster responsibility, while an alienated youth might make bonding difficult.",
            "q_char_skills" => "Nobody appears out of nowhere with skills and a place in society. Who were you? Where did you grow up? How did you learn what you know? These roots provide rich story hooks for the Storyteller.",
            "q_char_important_people" => "No mage is an island. Do you have casual friends or a tight-knit circle? How is your relationship with your family? Does anyone know about your ability to shake the pillars of Heaven?",
            "q_char_first_magick" => "Humans excel at rationalizing anomalies. What event broke through that filter? How did you realize there were mysteries science couldn't explain? Were you terrified, amazed, or hungry for power?",
            "q_char_mentor" => "Tradition mages usually begin with a mentor. How did you meet? Did you seek them out or were you found? Were they nurturing or strict? Did they act as teacher, parent, or a formative influence?",
            "q_char_cabala" => "Your cabal connects you with the other player characters. How did you meet? Did you prevent a disaster, survive a Nephandi ambush, or answer a higher calling together?",
            "q_char_mundane_life" => "Do you keep a 'secret identity'? Do you still walk among the Sleepers hiding your supernatural nature, or have you left your old mortal life completely behind?",
            "q_player_what_to_do" => "Player motivations matter just as much as character goals. What archetype do you wish to portray (mad scientist, hermit mystic, high-society investigator)? What deeds do you want to accomplish?",
            "q_player_destiny_pursued" => "What destiny calls to your mage? Dying for the greater good? Bringing down the Technocracy? Perfecting your inner self? What vision guides your path to Ascension?",
            "q_player_destiny_view" => "Few mages want to dwell on the end of the road. Yet every mage senses what fate holds. Does your destiny excite or frighten you? Will you embrace it or fight it?",
            "q_player_avatar_nature" => "What does your Avatar desire? How does it manifest (an imaginary friend, a sudden flash of genius, a crowned angelic vision)? What cosmic role does it expect you to fulfill?",
            "q_player_avatar_relation" => "Are you in harmony or conflict with your Avatar? Does it guide you patiently or drag you kicking and screaming through Seekings? Perfect harmony is rare and boring!",
            "q_player_conflicts" => "The Path of Ascension is fraught with peril. What flaws or temptations could derail your mage (pride, tragic love, vengeance, obsession, insanity)? How will you face them?",
            _ => id,
        },
    }
}
