# Revisão das Habilidades (Abilities) — Textos em Inglês

**Data:** 2026-10-30
**Fonte:** M20, Capítulo Seis, de "Core Abilities" até o fim de "Secondary Knowledges": páginas 276 a 302 do PDF (impressas 275 a 301). A última página foi incluída porque Media e Pharmacopoeia terminam nela.
**Escopo:** texto integral em inglês, níveis (ratings), "Possessed By" e especializações sugeridas das 33 Habilidades Básicas e das 25 Secundárias, além dos quadros e regras opcionais dessas páginas.

## Resumo

- **58 habilidades** agora têm texto integral em inglês: as 33 básicas e as 25 secundárias do livro.
- **12 habilidades secundárias novas**, que faltavam no compêndio: Flying, High Ritual, Lucid Dreaming, Search, Hypertech, Jetpack, Riding, Torture, Finance, Lore / RD Data, Media e Pharmacopoeia / Poisons.
- **Níveis em inglês** (título e frase de cada ponto) corrigidos para o texto do livro. Quase todos eram paráfrases. Os Conhecimentos usam os títulos do livro: Dabbler, Student, Scholar, Professor, Master.
- **Especializações sugeridas em inglês** substituídas pelas listas do livro. Muitas estavam incompletas ou eram inventadas. A coluna da tabela abaixo mostra quantas foram acrescentadas (+) e removidas (−).
- **"Possessed By" em inglês** substituído pela lista do livro.
- **Páginas** (`page_ref`) corrigidas para a numeração impressa. As antigas estavam deslocadas em até 7 páginas.
- **7 quadros** com texto integral e organizados na seção "Regras" do compêndio, dois deles novos. Veja abaixo.

## Como fica a exibição

- **Habilidade, em inglês:** o card "Description & Applications" mostra o texto integral do livro, com links internos (`mta://`) para outras habilidades, antecedentes e quadros. No fim, uma linha **Related boxes** liga a habilidade aos quadros relevantes.
- **Habilidade, em português:** continua o resumo de antes (`description_pt`). O texto integral em português fica para uma próxima etapa.
- **Quadros:** aparecem na lista "Regras e Teoria de Habilidades", na ordem do livro. O rótulo agora diferencia **Regra Opcional** de **Quadro** (sidebar); antes, todos diziam "Regra Opcional".
- **Correção de bug:** o botão "Regras" do filtro apontava para um id inexistente (`rule_theory_body_control`) e não abria nada. Agora abre o primeiro quadro.

## Quadros (boxes) e regras expandidas

| Quadro | Pág. | Situação | Formato no compêndio |
|---|---|---|---|
| Optional Rule: Minimum Abilities | 275 | Texto completado: faltava o parágrafo sobre usar a regra em Computer, Do, Hypertech, Martial Arts, Science e Technology (a lista antiga trazia Abilities erradas) | Artigo, com links para as Abilities citadas |
| Optional Rule: Professional Skills and Expert Knowledges | 277 | Texto integral, incluindo a escala de níveis e as listas de Perícias e Conhecimentos sugeridos | Artigo com as seções **Ratings** e **Reference** em lista |
| Optional Rule: The Well-Skilled Craftsman | 279 | Texto integral. A lista de Abilities a que a regra se aplica estava errada (incluía Crafts e omitia Esoterica, Lore e Politics) | Artigo com links para cada Ability coberta |
| Do and Other Martial Arts | 281 | **Novo**: não existia | Artigo do tipo Quadro, ligado a Do e Martial Arts |
| Optional Rule: Body Control | 285 | Texto integral, incluindo a nota sobre o *Akashic Brotherhood* e a referência a Environmental Hazards | A tabela "dots / successes / feat" virou lista: **Mínimo / Sucessos — Proeza: efeito** |
| Scientific Specialties | 288–289 | **Novo**: as 24 especializações científicas com definição | Artigo do tipo Quadro em lista. Os 24 nomes também viraram as especializações sugeridas de Science |
| SCIENCE!!! | 290 | Texto integral: antes era um resumo de 3 parágrafos. Agora traz as seções "Wait — I Can Explain!", "In Theory…", "There Must be Some Rational Explanation…" e "You See?" | Artigo com subtítulos `####` |
| Blatancy Effects (tabela) | 292 | **Novo**: dificuldade × sucessos × efeito × explicação | Dentro do texto de Blatancy, como lista de 5 linhas (6 a 10) |

O renderizador do compêndio (`RichTextView`) não suporta tabelas. Por isso todas as tabelas do livro viraram listas com rótulos em negrito.

## Alterações por habilidade

Colunas: **Níveis** = quantos dos 5 níveis em inglês mudaram. **Especializações** = total atual (+ acrescentadas / − removidas).

| Habilidade | Categoria | Tipo | Pág. | Situação | Níveis | Especializações |
|---|---|---|---|---|---|---|
| Alertness | Talento | Básica | 275 | Revisada | 0 | 7 (+0 / −1) |
| Art | Talento | Básica | 275–276 | Revisada | 0 | 15 (+5 / −1) |
| Athletics | Talento | Básica | 276 | Revisada | 2 | 11 (+2) |
| Awareness | Talento | Básica | 276–277 | Revisada | 5 | 7 (+5 / −5) |
| Brawl | Talento | Básica | 277 | Revisada | 5 | 6 (+4 / −5) |
| Empathy | Talento | Básica | 277 | Revisada | 5 | 9 (+9 / −6) |
| Expression | Talento | Básica | 279 | Revisada | 5 | 8 (+8 / −7) |
| Intimidation | Talento | Básica | 279 | Revisada | 5 | 7 (+5 / −4) |
| Leadership | Talento | Básica | 279 | Revisada | 5 | 6 (+6 / −6) |
| Streetwise | Talento | Básica | 279 | Revisada | 5 | 13 (+11 / −4) |
| Subterfuge | Talento | Básica | 279 | Revisada | 5 | 8 (+7 / −5) |
| Crafts | Perícia | Básica | 279–280 | Revisada | 5 | 10 (+8 / −5) |
| Drive | Perícia | Básica | 280 | Revisada | 5 | 12 (+9 / −4) |
| Etiquette | Perícia | Básica | 280–281 | Revisada | 5 | 10 (+9 / −5) |
| Firearms | Perícia | Básica | 281 | Revisada | 5 | 10 (+8 / −5) |
| Martial Arts | Perícia | Básica | 281 | Revisada | 5 | 8 (+8 / −6) |
| Meditation | Perícia | Básica | 281 | Revisada | 5 | 13 (+13 / −5) |
| Melee | Perícia | Básica | 281–282 | Revisada | 5 | 9 (+8 / −6) |
| Research | Perícia | Básica | 282 | Revisada | 5 | 7 (+7 / −5) |
| Stealth | Perícia | Básica | 282–283 | Revisada | 5 | 7 (+7 / −6) |
| Survival | Perícia | Básica | 283 | Revisada | 5 | 6 (+6 / −7) |
| Technology | Perícia | Básica | 283 | Revisada | 5 | 6 (+6 / −6) |
| Academics | Conhecimento | Básica | 283 | Revisada | 5 | 3 (+3 / −7) |
| Computer | Conhecimento | Básica | 283–284 | Revisada | 5 | 16 (+15 / −5) |
| Cosmology / Subdimensions | Conhecimento | Básica | 284 | Revisada | 5 | 10 (+10 / −6) |
| Enigmas | Conhecimento | Básica | 284 | Revisada | 5 | 10 (+9 / −5) |
| Esoterica | Conhecimento | Básica | 284–286 | Revisada | 5 | 28 (+21 / −1) |
| Investigation | Conhecimento | Básica | 286 | Revisada | 5 | 9 (+5 / −2) |
| Law | Conhecimento | Básica | 286 | Revisada | 5 | 12 (+12 / −6) |
| Medicine | Conhecimento | Básica | 286 | Revisada | 5 | 12 (+11 / −5) |
| Occult | Conhecimento | Básica | 286–287 | Revisada | 5 | 14 (+13 / −6) |
| Politics | Conhecimento | Básica | 287–288 | Revisada | 5 | 18 (+17 / −5) |
| Science | Conhecimento | Básica | 288 | Revisada | 5 | 24 (+24 / −7) |
| Animal Kinship | Talento | Secundária | 289–291 | Revisada | 5 | 7 (+5 / −5) |
| Blatancy | Talento | Secundária | 291–292 | Revisada | 5 | 11 (+11 / −5) |
| Carousing | Talento | Secundária | 292 | Revisada | 5 | 9 (+9 / −5) |
| Do | Talento | Secundária | 292–293 | Revisada | 5 | 6 (+6 / −5) |
| Flying | Talento | Secundária | 293 | **Nova** | 5 (novos) | 7 (+7) |
| High Ritual | Talento | Secundária | 293–294 | **Nova** | 5 (novos) | 7 (+7) |
| Lucid Dreaming | Talento | Secundária | 294 | **Nova** | 5 (novos) | 7 (+7) |
| Search | Talento | Secundária | 294–295 | **Nova** | 5 (novos) | 7 (+7) |
| Seduction | Talento | Secundária | 295 | Revisada | 5 | 7 (+7 / −5) |
| Acrobatics | Perícia | Secundária | 295 | Revisada | 5 | 9 (+6 / −3) |
| Archery | Perícia | Secundária | 295 | Revisada | 5 | 9 (+6 / −2) |
| Biotech | Perícia | Secundária | 295–296 | Revisada | 5 | 11 (+10 / −4) |
| Energy Weapons | Perícia | Secundária | 296–297 | Revisada | 5 | 9 (+9 / −5) |
| Hypertech | Perícia | Secundária | 297 | **Nova** | 5 (novos) | 13 (+13) |
| Jetpack | Perícia | Secundária | 297 | **Nova** | 5 (novos) | 6 (+6) |
| Riding | Perícia | Secundária | 297 | **Nova** | 5 (novos) | 11 (+11) |
| Torture | Perícia | Secundária | 298 | **Nova** | 5 (novos) | 9 (+9) |
| Area Knowledge | Conhecimento | Secundária | 298 | Revisada | 5 | 12 (+9 / −2) |
| Belief Systems | Conhecimento | Secundária | 298–299 | Revisada | 5 | 10 (+10 / −6) |
| Cryptography | Conhecimento | Secundária | 299 | Revisada | 5 | 8 (+8 / −5) |
| Demolitions | Conhecimento | Secundária | 299 | Revisada | 5 | 9 (+7 / −3) |
| Finance | Conhecimento | Secundária | 299 | **Nova** | 5 (novos) | 11 (+11) |
| Lore / RD Data | Conhecimento | Secundária | 299–300 | **Nova** | 5 (novos) | 12 (+12) |
| Media | Conhecimento | Secundária | 300–301 | **Nova** | 5 (novos) | 20 (+20) |
| Pharmacopoeia / Poisons | Conhecimento | Secundária | 301 | **Nova** | 5 (novos) | 9 (+9) |

Todas as 58 receberam texto integral novo em `data/compendium/abilities/<id>.en.md`.

## Pontos de atenção

- **Style e Pilot** existem no compêndio como secundárias, mas **não estão no M20** (pp. 275–301). Foram mantidas sem alteração, para não quebrar fichas que já as usam. Vale decidir se ficam, saem ou ganham uma fonte (outro suplemento).
- **Demolitions** é Conhecimento no livro, e o compêndio já estava certo.
- **Science:** no livro, as especializações sugeridas são uma frase ("Almost any hard scientific field…"). No compêndio, a lista sugerida passou a ser a das 24 especializações do quadro Scientific Specialties.
- **Correções de digitação do livro** aplicadas no texto: *reminder* → *remainder* (Body Control) e *Ver y basic sur vival* → *Very basic survival* (Survival), além das palavras grudadas pela extração (*Well-Skilled*, *Mind-based*, *Rabble-Rousers* etc.).
- **Português:** nas 12 habilidades novas, os campos em PT (nome, resumo, níveis, "possuído por", especializações) foram traduzidos de forma resumida, porque o código exige esses campos. Nas habilidades existentes, os campos em PT **não** foram alterados. Por isso, os níveis e especializações em PT agora podem divergir do inglês.

## Verificação

- Script: cada frase de prosa do livro foi procurada nos `.md` gerados. As únicas ausências são intencionais (links no lugar de "see p. 277" e a correção *reminder* → *remainder*).
- `cargo test --features ssr --test all_tests`: 228 testes passando, incluindo um teste novo (`test_abilities_unabridged_texts_and_links`). Ele garante que toda habilidade do livro tem texto em inglês e que todos os links `mta://` dos textos e quadros apontam para algo que existe.
- `cargo check --target wasm32-unknown-unknown --features hydrate`: compila.
- **Não verificado:** a aparência da tela no navegador; o app não foi executado.

## Pendências sugeridas

- Texto integral em português das 58 habilidades e dos 7 quadros.
- Revisar os níveis, "possuído por" e especializações em PT das habilidades existentes, para acompanhar o inglês.
- Decidir o destino de Style e Pilot.

## Atualização: tradução para português

Depois desta revisão, todos os textos ganharam a versão `.pt.md`, e a tela em português passou a mostrar o texto integral traduzido no lugar dos resumos.
