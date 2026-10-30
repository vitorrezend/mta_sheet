# Revisão dos Antecedentes (Backgrounds) — Textos em Inglês

**Data:** 2026-10-30
**Fonte:** M20 (*Mage: The Ascension 20th Anniversary Edition*), Capítulo Seis, seção "Background Traits" — páginas 304 a 329 do PDF (páginas impressas 303 a 328).
**Escopo:** somente os textos integrais em inglês, `data/compendium/backgrounds/*.en.md` (modo "Texto Integral" do compêndio).

## O que não foi alterado

- **Versões em português** (`*.pt.md`): continuam com o texto antigo. Agora divergem do inglês nos antecedentes marcados como *Reescrito* abaixo.
- **Trilha de níveis** (pontuações 0–10): fica em `crates/mta_core/src/compendium/backgrounds/data.rs` e é exibida abaixo do texto. Por isso as tabelas de pontos **não** foram copiadas para os `.en.md`. A exceção são Sanctum e Totem, que já tinham tabela própria.
- **Resumo rápido** (`description` / `system` em `data.rs` e `backgrounds.json`): sem alterações.
- **Regras gerais** (`theory_background_rules.en.md`): fora do intervalo de páginas, sem alterações.

## Critérios

1. O texto em prosa de cada antecedente foi transcrito por completo, na ordem do livro, incluindo listas (equipes de Backup, tipos de Familiar, orientações de Blessing, opções de Enhancement e tamanhos de Chantry).
2. Foram mantidos os padrões do projeto: título `###`, links `mta://` para outros antecedentes, atributos e práticas, tipografia (’ “ ” — …) e as notas **Special Rule** dos antecedentes com `max_dots = 10`.
3. Frases que **não existem no livro** (conteúdo inventado ou parafraseado de forma incorreta) foram removidas.
4. Junções de hifenização da extração foram corrigidas (ex.: *single-purpose*, *fleshed-out*, *lower-ranking*, *well-informed*, *non-Technocrat*), assim como o erro de digitação do livro *biomodifed* → *biomodified*.
5. A verificação foi feita por script: cada frase de prosa do livro foi procurada no arquivo final. As únicas frases ausentes são as linhas da tabela de pontos (intencional) e pequenas paráfrases que já existiam em Sanctum.

## Legenda

- **Completado:** o texto existente era fiel, mas faltavam parágrafos.
- **Reescrito:** o texto existente divergia do livro (conteúdo inventado, regras diferentes ou resumo genérico) e foi substituído pelo texto canônico.
- **Corrigido:** o texto já estava praticamente completo; só recebeu ajustes pontuais.

## Alterações por antecedente

| Antecedente | Pág. impressa | Situação | O que mudou |
|---|---|---|---|
| Allies | 303–304 | Completado | Adicionada a frase final do livro (acima de 5 pontos, Apêndice I). Nota *Special Rule* mantida. |
| Alternate Identity | 304 | Completado | O bloco "Game System" resumido foi trocado pelo parágrafo integral: Mental Trait + Investigation, dificuldade Identidade + 3, compra múltipla, exemplo Jane Palmer / Eva Morrissey. Adicionada a referência ao quadro *Differential Backgrounds*. |
| Arcane / Cloaking | 304 | Completado | Parágrafo de sistema integral: redução de Perception **ou Investigation** (antes só Perception), duração enquanto discreto, comportamento em combate e traços marcantes. A frase sobre a explicação tecnocrática foi restaurada à redação original. |
| Avatar / Genius | 304–305 | Completado | Restaurados a referência à entrada Quintessence (pp. 331–333) e o trecho "mais brasa que fogueira". Removida a menção a Node, que não está no livro. |
| Backup | 305–306 | Completado | Adicionados os parágrafos sobre perfil dos agentes (Traits 1–3, "temps" de elite custam o dobro), exigência de pertencer a uma organização, perda do antecedente, a lista completa de 17 equipes de exemplo e a referência ao Apêndice I. |
| Blessing | 306–307 | Completado | Adicionados os parágrafos sobre a estranheza da bênção, a ligação com forças maiores, a revisão em relação ao *Guide to the Traditions*, as 8 diretrizes obrigatórias, os exemplos (Pan, Allah, Dama do Lago, "Lucifer"), a regra de redução de dificuldade e o veto do Narrador. Exemplo corrigido para "Sister of Hippolyta" (antes "Amazon"). |
| Certification | 307 | Completado | Texto integral: exigência de Ability por ponto, subornos, vínculo com Alternate Identity e visibilidade perante as autoridades. Removido o link incorreto para a arma *katana*. |
| Chantry / Construct | 307–308 | Completado | Adicionados: posição do iniciante na hierarquia, criação de Chantry própria (vedada a tecnocratas), a tabela de pools (Secure Squat a Power Center), formas e recursos por nível, estabilidade do pool e a lista de 14 antecedentes que podem ser vinculados. Corrigido "21st century" → "20th century". |
| Contacts | 308 | **Reescrito** | O texto antigo não era do livro. Agora traz o texto canônico: exemplos de Jinx e Malcolm, +2 de dificuldade fora da área, rolagem Social + Contacts, falha e falha crítica, contatos maiores vs. casuais. |
| Cult | 309 | **Reescrito** | O texto antigo (doações, Quintessência de adoração, "nó vivo") não era do livro. Agora: bônus de dados em rituais (Chapter Ten), presença física ou em rede, seguidores Sleepers e o limite de 5 pontos como benefício máximo. **Removida** a nota *Special Rule* de até 10 pontos, que contradizia o livro e o `max_dots = 5` do próprio projeto. |
| Demesne | 309–310 | **Reescrito** | O texto antigo descrevia um local físico onde a magia vulgar vira coincidente, o que está **incorreto**. No M20, Demesne é um palácio da memória / Reino Onírico mental. Agora traz: visita (Perception + Demesne, dif. 5–9), viagem astral, aparência (Wits + Demesne), memórias ocultas, atrair outras mentes (Mind 3/4) e uso durante uma Quiet. |
| Destiny | 310 | **Reescrito** | O texto antigo ("rede de segurança", a arma que trava) não era do livro. Agora traz a regra canônica: uma vez por sessão, sem Willpower, rola-se Destiny contra dif. 8 e cada sucesso recupera 1 Willpower. Inclui o "Momento do Destino". |
| Dream / Hypercram | 310–311 | Completado | Adicionados os parágrafos de narrativa e de sistema: Perception + Dream dif. 6, substituição da Ability e "um Trait não soma ao outro". O exemplo de Spider foi restaurado à versão do livro (o antigo inventava detalhes e um link para *Dô*). Adicionada a decisão final do Narrador. |
| Enhancement | 311–312 | Completado | Adicionados: descrição das três opções, regras completas de Cibernética (Paradox permanente, falhas de segurança, dano extra de Life, Primium Countermeasures), Biomods e Genegineering, a lista de Atributos permitidos (até 8), o custo de **2 pontos por ponto**, a impossibilidade de compartilhar/requisitar, o uso por não-tecnocratas e as referências. |
| Fame | 312–313 | **Reescrito** | O texto antigo não era do livro. Agora: perfil público, deferência, rolagem Charisma/Manipulation/Appearance + Fame, visibilidade na era das câmeras e fama efêmera. |
| Familiar / Companion | 313–315 | **Reescrito** | O texto antigo tinha regras erradas ("armazena 3 Quintessência por ponto", "obliterado na Umbra"). Agora: visão tecnocrática dos Companions, os 12 tipos de familiar, benefícios (Advice, Empathy, Feast of Nettles), capacidade de 5 Paradox por ponto, Quintessence Feeding, perda ao morrer (1 Willpower permanente + 2 Quintessência por nível) e o ritual de vínculo. |
| Influence | 315–316 | **Reescrito** | O texto antigo não era do livro. Agora: deferência social, combinações com Atributos Sociais/Mentais/Físicos, até 10 pontos e as práticas Art of Desire e Dominion. *Special Rule* mantida. |
| Legend | 316–317 | **Reescrito** | O texto antigo (reputação entre espíritos e fadas) estava **incorreto**. No M20, Legend torna o personagem um "Nó ambulante": uma rolagem por história recupera Quintessência (dif. 6–9), cria Tass e recarrega aliados; também cobre objetos e animais lendários e a Ressonância lendária. |
| Library | 317–318 | **Reescrito** | O texto antigo (reduz dificuldade de Enigmas/Occult, link para *alchemy*) não era do livro. Agora: arquivo multimídia, pesquisa com Mental Trait + Library, a regra de **pool** (maior nível + 1 por contribuinte) e a referência a Raising Traits. |
| Mentor | 318 | **Reescrito** | O texto antigo não era do livro. Agora: natureza do mentor, recrutamento tecnocrático, agendas próprias, benefícios e obrigações, e o mentor como personagem do Narrador. |
| Node | 318–319 | **Reescrito** | Título corrigido de "Node / Requisition Pool" para **Node**. Removido o conteúdo inventado (Primium líquido). Agora: conversão de Nós na Guerra da Ascensão, Ressonância e forma do Tass, Gauntlet, produção semanal sugerida (2 por ponto por semana, metade Tass), caerns dos lobisomens e a nota de escala 1–10. |
| Past Lives | 319 | **Reescrito** | O texto antigo não era do livro. Agora a regra canônica: uma vez por sessão, Past Lives dif. 8, cada sucesso dá um dado extra para outra Ability, e a falha crítica causa trauma de vidas passadas. |
| Patron | 319–320 | **Reescrito** | O texto antigo (financiador que exige prestação de contas) estava **incorreto**. No M20, Patron é um benfeitor **secreto**, cujas intenções só o Narrador conhece. |
| Rank | 320 | **Reescrito** | O texto antigo (patente dentro da Tecnocracia) estava **incorreto**: Rank é posição em instituições **Sleeper**. Agora: Influence = Rank dentro do seu domínio (metade fora dele), Resources e Fame = metade do Rank em capacidade oficial. |
| Requisitions | 320–321 | **Reescrito** | O texto antigo trazia uma tabela de dificuldades **errada** (4–8 com rótulos inventados). Agora: exclusivo de tecnocratas, cada sucesso vale 5 pontos de Devices, a opção sem rolagem, devolução e perda, pool do amálgama e a tabela correta (Doubtful 9, Questionable 8, Assumed 7, Assured 6, Total 5), que coincide com `data.rs`. |
| Resources | 321–322 | Completado | Adicionados: o High Guild, o potencial econômico das Esferas, a preferência por encantar pessoas em vez de transmutar ouro, o controle de grandes fortunas e a nota sobre vetar 8–10 pontos (Primal Utility, Money and Wealth). Link de prática corrigido para `art_of_desire` (o antigo `hypereconomics` não é um id existente). |
| Retainers | 322 | Completado | Adicionados os parágrafos sobre o Retainer como personagem do Narrador, as habilidades mundanas, a transformação em Ally quando treinado, e servos construídos ou espirituais. |
| Sanctum / Laboratory | 322–323 | Corrigido | Já estava completo. Corrigido o Gauntlet do laboratório tecnocrático para **9**, como no livro (estava "standard or elevated"), e "a Black Suit" → "the Black Suit". |
| Secret Weapons | 323–324 | **Reescrito** | O texto antigo (armas experimentais com códigos de dano) estava **incorreto**. No M20, são protótipos da Q Division emprestados a agentes tecnocratas, com rolagem secreta do Narrador (Social + Secret Weapons). |
| Spies | 324 | **Reescrito** | O texto antigo não era do livro. Agora: natureza dos informantes, risco de traição e as quatro rolagens (Intelligence, Manipulation, Charisma, Perception + Spies). *Special Rule* mantida. |
| Status | 324–325 | **Reescrito** | O texto antigo não era do livro. Agora: reputação entre pares Despertos, bônus em rolagens Sociais (metade entre aliados, nenhum entre inimigos) e o fato de ser conquistado e poder ser perdido. |
| Totem | 325–327 | Corrigido | Já estava completo. Restaurada a frase "As mentioned above, only shamanic…". |
| Wonder | 328 | **Reescrito** | O texto antigo ("não gera Paradoxo", "recarrega em Nós") não era do livro. Agora: Talisman / Fetish / Device, exemplos, a referência à Toybox (pp. 651–653), o temperamento dos Wonders e a criação de uma história para o item. Título ajustado para "Wonder (Device / Fetish / Talisman, etc.)". Observação: este texto está na pág. 329 do PDF, uma após o intervalo pedido. |

## Pendências sugeridas

- Atualizar as versões `*.pt.md` dos antecedentes **reescritos**, que hoje contradizem o inglês (sobretudo Demesne, Legend, Patron, Rank, Secret Weapons, Requisitions e Wonder).
- O resumo rápido (`description` em `data.rs` / `backgrounds.json`) foi verificado por amostragem (Demesne e Patron) e está fiel ao livro. Não precisa de correção imediata.
