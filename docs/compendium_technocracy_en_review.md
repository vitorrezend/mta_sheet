# União Tecnocrática — Textos em Inglês

**Data:** 2026-10-31
**Fonte:** M20, Capítulo Cinco, "Ascension Warriors": PDF pp. 187-196 (páginas impressas 186-195).
**Escopo:** texto integral em inglês, em Markdown, das cinco Convenções da União Tecnocrática, no mesmo formato das Tradições. O português ainda não tem os textos.

## Resumo

- **5 textos** em `data/compendium/technocracy/`: Iteration X, New World Order, Progenitors, Syndicate e Void Engineers.
- **Onde ficam:** na mesma seção das Tradições, que agora se chama **Tradições & Tecnocracia** e tem 14 itens. As Convenções aparecem num segundo grupo da lista, "A União Tecnocrática (pp. 186-195)", marcadas com ⚙.
- **Links:**
  - as Esferas de afinidade levam ao verbete de cada Esfera;
  - as Práticas citadas em "Focus" levam ao verbete de cada Prática;
  - outras referências do texto também viraram links: a Verbena e a Iteração X no texto dos Progenitores, o quadro SCIENCE!!! (p. 290) e a Esfera Ciência Dimensional no texto da Iteração X.
- **Fora do escopo:** as páginas antes do intervalo (introdução da Parte III, "The Dirty Side of the Coin" etc.) e depois dele (Parte IV, os Disparates) ficaram de fora.

## As cinco Convenções

| Convenção | Págs. | Esferas de afinidade |
|---|---|---|
| Iteration X | 186-187 | Forces, Matter ou Time |
| New World Order | 188-189 | Mind ou Correspondence/ Data |
| Progenitors | 190-191 | Life ou Prime |
| Syndicate | 192-193 | Entropy, Mind ou Primal Utility |
| Void Engineers | 194-195 | Dimensional Science (Spirit), Correspondence ou Forces |

Cada texto segue a ordem do livro:
1. A epígrafe.
2. Um bloco com a página e as Esferas de afinidade.
3. A visão geral.
4. As seções **Organization**, **Initiation**, **Affinity Spheres**, **Focus** e **Stereotypes**. Nos estereótipos, Fellow Conventions, The Traditions e The Disparates aparecem como itens separados.

## Como aparece no app

- **Lista:** a mesma tela das Tradições, com as Convenções logo abaixo das Nove Tradições.
- **Links:** `mta://technocracy/<id>`, `mta://conventions/<id>` e `mta://traditions/<id>` abrem uma Convenção.
- **Busca:** encontra uma Convenção pelo nome em inglês ou português e por apelidos, como "NWO", "High Guild", "Wanderers" e "Sindicato".
- **Em português:** os nomes aparecem traduzidos (Iteração X, Nova Ordem Mundial, Progenitores, Sindicato, Engenheiros do Vazio), com o aviso "Texto disponível por enquanto apenas em inglês". O aviso some sozinho quando houver um arquivo `.pt.md`.

## Decisões de texto

- **Títulos:** os nomes das Convenções e as epígrafes são imagens no PDF. Os nomes foram escritos à mão. O subtítulo usa outro nome que o próprio texto dá à Convenção (*The Clockwork Convention*, *The Wanderers*, *Formerly the High Guild* etc.).
- **Erros de digitação do livro corrigidos:**

  | No livro | No compêndio |
  |---|---|
  | Mathinclined | Math-inclined |
  | bleedingedge | bleeding-edge |
  | weirdscience | weird-science |
  | selfmastery | self-mastery |
  | resourceintensive | resource-intensive |
  | willbroken | will-broken |
  | counterrational | counter-rational |

  *oldlife* e *newlife* (New World Order) ficaram como no livro, porque parecem termos intencionais.

## Arquivos

- `crates/mta_core/src/compendium/traditions.rs`:
  - `ALL_CONVENTIONS`, `find_convention` e `find_faction` (busca Tradições e Convenções juntas);
  - `has_pt_text()`, que diz se já existe tradução própria.
  - `find_tradition` continua buscando só as Nove Tradições.
- `src/components/compendium/views/traditions_view.rs`: segundo grupo na lista e aviso em português.
- `navigation.rs`, `query_resolver.rs`, `category_selector.rs` e `mod.rs`: rótulos da seção, páginas (pp. 147-195) e os novos endereços `mta://`.
- `tests/suites/traditions_compendium_test.rs`: teste novo que confere:
  - os 5 textos e suas seções;
  - que todo link aponta para algo que existe;
  - os endereços `mta://technocracy/...`;
  - a busca.

## Verificação

- Script comparando o livro com os textos, frase a frase: as 288 frases estão nos textos. A única diferença é a epígrafe do Sindicato, que a extração grudou num número de página; ela está no texto.
- `cargo test --features ssr --test all_tests`: 235 testes passam.
- `cargo test -p mta_core`: 63 testes passam.
- `cargo check --target wasm32-unknown-unknown --features hydrate`: compila.
- **Não verificado:** a aparência da tela no navegador; o app não foi executado.
