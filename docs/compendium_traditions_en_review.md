# Tradições do Conselho — Textos em Inglês

**Data:** 2026-10-31
**Fonte:** M20, Capítulo Cinco, "Ascension Warriors": PDF pp. 149-167 (páginas impressas 148-166). Também foi lido o quadro "Tradition Descriptions" da p. 147 (PDF 148), que explica os campos de cada Tradição.
**Escopo:** texto integral em inglês, em Markdown, das Nove Tradições, numa seção nova do compêndio. O português ainda não tem os textos.

## Resumo

- **Seção nova "Tradições"** no compêndio. Antes não havia nada sobre as Tradições.
- **10 textos** em `data/compendium/traditions/`:
  - as 9 Tradições;
  - o quadro **Tradition Descriptions** (p. 147).
- **Links:** cada Esfera de afinidade e cada Prática citada em "Focus" leva ao verbete correspondente. O quadro introdutório lista as 9 Tradições com página e Esferas.
- **Fora do escopo:** as pp. 166-167 do PDF já começam a Parte III (A União Tecnocrática). Ficaram de fora, porque o pedido era só das Tradições.

## As Nove Tradições

| Tradição | Págs. | Esferas de afinidade |
|---|---|---|
| Akashayana | 148-149 | Mind ou Life |
| Celestial Chorus | 150-151 | Prime; Forces ou Spirit |
| Cult of Ecstasy | 152-153 | Time; Life ou Mind |
| Dreamspeakers | 154-155 | Spirit; Forces, Life ou Matter |
| Euthanatos | 156-157 | Entropy; Life ou Spirit |
| Order of Hermes | 158-159 | Forces (Life, Matter, Mind e Spirit como estudos secundários) |
| Society of Ether | 160-161 | Matter; Forces ou Prime |
| Verbena | 162-163 | Life; Forces |
| Virtual Adepts | 164-165 | Correspondence/ Data; Forces |

Cada texto segue a ordem do livro:
1. A epígrafe da Tradição.
2. Um bloco com a página e as Esferas de afinidade.
3. A visão geral.
4. As seções **Organization**, **Initiation**, **Affinity Spheres**, **Focus** e **Stereotypes**. Nos estereótipos, Fellow Traditions, The Technocracy e The Disparates aparecem como itens separados.

## Como aparece no app

- **Onde fica:** a seção "Tradições" (🏛️) é a última do seletor de categorias.
- **Lista:** à esquerda, o quadro "Tradition Descriptions" e depois as 9 Tradições, cada uma com a página.
- **Detalhe de cada Tradição:**
  - nome e página;
  - as Esferas de afinidade como botões que abrem o verbete de cada Esfera;
  - o texto integral.
- **Links:** `mta://traditions/<id>` abre uma Tradição a partir de qualquer texto do compêndio. A busca inicial também encontra Tradições por nome em inglês ou português e por apelidos, como "Chakravanti", "Hermetics" e "Sons of Ether".
- **Em português:** os nomes aparecem traduzidos (Coro Celestial, Ordem de Hermes, Sociedade do Éter etc.), com o aviso "Texto disponível por enquanto apenas em inglês".

## Decisões de texto

- **Títulos:** os nomes das Tradições e as epígrafes são imagens no PDF. Os nomes foram escritos à mão. O subtítulo de cada texto usa outro nome que o próprio texto dá à Tradição (por exemplo, *Chakravanti*, *Kha’vadi*, *Sahajiya*, *The Mercurial Elite*).
- **Epígrafes em maiúsculas** (Celestial Chorus, Euthanatos, Virtual Adepts) foram passadas para caixa normal.
- **Listas:** os cinco passos de iniciação do Cult of Ecstasy viraram uma lista.
- **Erros de digitação do livro corrigidos:**

  | No livro | No compêndio |
  |---|---|
  | possiblity | possibility |
  | malficia | maleficia |
  | Force (Esferas dos Dreamspeakers) | Forces |
  | selfdestruction | self-destruction |
  | SaintGermain | Saint-Germain |
  | opiumtrade | opium-trade |
  | longwinded | long-winded |
  | Kitab-alAlacir | Kitab-al-Alacir |

## Arquivos

- `crates/mta_core/src/compendium/traditions.rs` (novo):
  - `ALL_TRADITIONS`, `TRADITION_DESCRIPTIONS` e `find_tradition`;
  - dois testes de unidade.
- `src/components/compendium/views/traditions_view.rs` (novo): a tela da seção.
- `src/components/compendium/mod.rs`, `navigation.rs`, `query_resolver.rs`, `category_selector.rs` e `views/mod.rs`: ligação da seção nova.
- **Correção de passagem:** o cabeçalho e o rodapé da seção de Arquétipos diziam pp. 264-269; agora dizem pp. 267-273.
- `tests/suites/traditions_compendium_test.rs` (novo) confere:
  - os 9 textos e suas seções;
  - que todo link `mta://` aponta para algo que existe;
  - a navegação e a busca.
- `tests/suites/compendium_category_selector_test.rs` e `compendium_links_test.rs`: ajustados para a 10ª seção.

## Verificação

- Script comparando o livro (pp. 147-165) com os textos, frase a frase:
  - das 585 frases do livro, só não aparecem do mesmo jeito as duas epígrafes que a extração grudou em números de página e as duas frases com erros de digitação corrigidos;
  - no outro sentido, o que não vem do livro são títulos, subtítulos, rótulos e o bloco de página/Esferas.
- `cargo test --features ssr --test all_tests`: 234 testes passam.
- `cargo test -p mta_core`: 63 testes passam.
- `cargo check --target wasm32-unknown-unknown --features hydrate`: compila.
- **Não verificado:** a aparência da tela no navegador; o app não foi executado.
