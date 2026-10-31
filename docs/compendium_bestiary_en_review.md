# Aliados & Antagonistas (Bestiário) — Textos em Inglês

**Data:** 2026-10-31
**Fonte:** M20, Apêndice I, "Allies and Antagonists": PDF pp. 619-642 (páginas impressas 618-641).
**Escopo:** parte A do plano de fichas rápidas. Todas as fichas prontas do apêndice entram no compêndio com o texto do livro em inglês, e os números de cada ficha ficam legíveis pelo app, para a parte C (cartões de NPC na sala).

## Resumo

- **Seção nova "Aliados & Antagonistas"** (🐺), a 11ª do compêndio.
- **79 textos** em `data/compendium/bestiary/`: 61 fichas e 18 introduções e quadros (em `articles/`).
- **Leitor de números:** `quick_stats()` extrai de cada ficha:
  - Força de Vontade;
  - níveis de vitalidade;
  - atributos;
  - Arete;
  - Rage, Gnosis e Essence, para espíritos;
  - a base de iniciativa (Destreza + Raciocínio).

  Na tela, esses números aparecem numa faixa de resumo acima do texto.

## O que entrou, por grupo

| Grupo | Págs. | Fichas |
|---|---|---|
| Supporting Cast | 618 | Introdução do apêndice, com a epígrafe de Goya |
| Bestiary | 618-620 | Introdução + 14 animais: jacaré, três pássaros, gato, chimpanzé, três cães, cavalo, leopardo/onça, lince, tigre/leão, lobo |
| Among the Masses | 620-623 | Introdução + Typical Citizen, Thug, Beat Cop, Professional Badass, Government Agent |
| Extraordinary Operatives & Technocratic Creations | 623-626 | Introdução + Extraordinary Scientist, Extraordinary Field Agent, Steelskin Cyborg, Victor, HIT Marks (introdução), HIT Mark V, HIT Mark X e o quadro **Hacking a HIT Mark** |
| Our Awakened Brethren | 626-628 | Introdução + Urban Shamanic Musician, Awakened Hacktivist, Black Suit |
| Awakened Enemies | 628-631 | Introdução + Marauders (introdução), Pillory, Hive, Nephandi (introdução), Widderslainte Cultist, NWO Infiltrator |
| Spirits | 631-633 | Introdução + Naturae (introdução), Ancient Forest-Heart, Animal Spirits (introdução) e os espíritos Corvo, Serpente, Cavalo, Coruja e Macaco |
| Totem Spirits | 633-636 | Introdução + quadro **Totem Traits** + Raven & Crow, Coyote, Spider, Glitch, City-Heart, The Man, Mary Jane, The Machine |
| Elementals | 636-637 | Introdução + Ar, Terra, Fogo, Água, Eletricidade, Vidro, Gelo, Metal, Tempestade, Madeira |
| Minions | 638-639 | Introdução + Memophores (tabela de categorias e os 6 exemplos), Pattern Spider |
| Paradox Manifestations | 639-641 | Introdução + Judgment, Wrinkle, Sendings |

## Formato

Cada ficha tem:
1. O título.
2. A página.
3. A descrição do livro.
4. As linhas da ficha, no formato `- **Rótulo:** valor` (Attributes, Abilities, Willpower, Health Levels, Armor Rating, Attack, Equipment, Image, Roleplaying Notes, Focus, Charms etc.).

Nos espíritos, a linha "Willpower 8, Rage 4, Gnosis 7, Essence 200" do livro ganhou o rótulo **Traits**, para o app conseguir ler.

As introduções de grupo e os quadros (Hacking a HIT Mark, Totem Traits) aparecem na lista com 📜, separados das fichas.

## Como aparece no app

- **Lista:** organizada pelos 11 grupos do livro. Cada ficha mostra o nome e a página.
- **Detalhe:** nome, página e grupo, a faixa de resumo e o texto integral.
  - Na faixa de resumo do Thug, por exemplo: "Força de Vontade 3 · 7 níveis de vitalidade · Iniciativa 4 + 1d10".
  - Nos espíritos, a faixa mostra Rage, Gnosis e Essence.
- **Navegação:** `mta://bestiary/<id>` abre uma ficha.
- **Busca:** encontra pelo nome em inglês ou em português (por exemplo, "Thug" ou "Terno Preto").
- **Em português:** os nomes da lista estão traduzidos, com o aviso "Texto disponível por enquanto apenas em inglês".

## Como os textos foram feitos

São 61 fichas, então os textos foram gerados por script a partir da extração do PDF, e não escritos à mão:
1. Limpeza: rodapés removidos, linhas juntadas, aspas e travessões tipográficos.
2. Separação de cada verbete por marcadores.
3. Rótulos da ficha transformados em lista.

Depois houve ajustes à mão:
- **Remontagens:** dois textos que o livro interrompe com um quadro foram remontados. Raven & Crow é cortado pelo quadro Totem Traits, e o HIT Mark V pelo Hacking a HIT Mark.
- **Correção de digitação:** a linha de vitalidade "OK, -1, -1 -2, - 2, -5" se repete em várias fichas e foi corrigida para "OK, -1, -1, -2, -2, -5".
- **Formatação:** os exemplos de memóforos e os itens do quadro Totem Traits foram formatados como lista.

A página de cada verbete foi detectada automaticamente, procurando o começo e o fim do texto em cada página do PDF.

`data/compendium/bestiary/manifest.json` guarda a lista de verbetes, grupos e páginas. `crates/mta_core/src/compendium/bestiary/data.rs` é gerado a partir dele.

## Arquivos

- `crates/mta_core/src/compendium/bestiary/`:
  - `models.rs`: tipos, grupos e o leitor `quick_stats()`;
  - `data.rs`: o catálogo;
  - `mod.rs`: busca e testes de unidade.
- `src/components/compendium/views/bestiary_view.rs`: a tela.
- `mod.rs`, `navigation.rs`, `query_resolver.rs` e `category_selector.rs`: ligação da seção nova.
- `tests/suites/bestiary_compendium_test.rs`: teste de integração.
- `compendium_category_selector_test.rs` e `compendium_links_test.rs`: ajustados para a 11ª seção.

## Verificação

- **Comparação com o livro, em trechos de 8 palavras:** os 2.274 trechos foram procurados nos textos e 111 não bateram. Todos ficam em pontos de corte: título de ficha colado ao texto, o rótulo "Traits" acrescentado e a linha de vitalidade corrigida. Não falta conteúdo do livro.
- **Leitor de números:** testado com pessoas (Thug, Black Suit), animais (Lobo, com faixa "3-4"; Cavalo, com o "Strength: 5" do livro), espíritos (elementais, com Rage, Gnosis e Essence) e Judgment, com "Materialized Health Levels: 20".
- `cargo test --features ssr --test all_tests`: 238 testes passam.
- `cargo test -p mta_core`: 67 testes passam.
- `cargo check --target wasm32-unknown-unknown --features hydrate`: compila.
- **Não verificado:** a aparência da tela no navegador; o app não foi executado.

## Próximo passo (parte C)

Os cartões de NPC na sala podem usar `quick_stats()` direto:
- a vitalidade vira a trilha clicável do cartão;
- a Força de Vontade vira os pontos para gastar;
- `initiative_base()` entra no rastreador de iniciativa, que já tem o campo "é NPC".
