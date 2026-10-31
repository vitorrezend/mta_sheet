# Revisão das Esferas — Textos em Inglês

**Data:** 2026-10-30
**Fonte:** M20, Capítulo 10, "Part II: The Spheres", PDF pp. 512-529 (páginas impressas 511-528).
**Escopo:** texto integral em inglês, em Markdown, das 9 Esferas, das 3 Esferas Tecnocráticas opcionais, da regra opcional Wild Talent e das regras gerais do início da seção. O português não foi alterado.

## Resumo

- **14 textos reescritos** a partir do livro em `data/compendium/spheres/<id>.en.md`. Os textos anteriores eram paráfrases: das 545 frases do livro nessas páginas, a maior parte não aparecia como no original.
- **Catálogo em inglês corrigido** em `crates/mta_core/src/compendium/spheres/data.rs`: subtítulo, especialidades, descrição e nome e resumo de cada posto, todos tirados do livro.
- **Páginas** nos cabeçalhos dos textos corrigidas para a numeração impressa. Os `page_ref` do catálogo já estavam certos.

## Textos

| Texto | Págs. | O que mudou |
|---|---|---|
| General Sphere Rules (`theory_sphere_rules`) | 511-512 | Texto do livro na ordem original; inclui a seção **Spheres By Rote**, que faltava |
| Correspondence | 512-513 | Texto integral; corrigida a frase da arma ("snatched away by Correspondence alone", não "Correspondence/ Matter") |
| Entropy | 514-515 | Texto integral; postos 4 e 5 com os nomes do livro (Affect Living Things; Affect Thought / Shape Memes / Binding Oath) |
| Forces | 515-516 | Texto integral |
| Life | 516-517 | Texto integral |
| Matter | 517-519 | Texto integral |
| Mind | 519-520 | Texto integral |
| Prime | 520-521 | Texto integral, com link para o Antecedente Avatar |
| Spirit | 521-522 | Texto integral |
| Time | 522-523 | Texto integral |
| Data | 524-525 | Texto integral e o quadro **Data Connections** |
| Dimensional Science | 525-526 | Texto integral |
| Primal Utility | 526-527 | Texto integral e o quadro **Primal Ventures**, com links para Node e Resources |
| Wild Talent | 527-528 | Texto integral, incluindo frases que faltavam em Marauder Wild Talent e The Death-Strike |

Cada texto mantém o formato anterior: título, subtítulo, bloco com especialidades, página e Esfera equivalente (mística ou tecnocrática), introdução e um `####` por posto. Os nomes de outras Esferas viraram links `mta://spheres/<id>` (a primeira menção de cada uma por parágrafo).

## Catálogo (`data.rs`, só inglês)

- **Subtítulos** do livro: por exemplo, Correspondence "Connections and Dimensions" (antes "Mastery of Space and Distance"), Forces "Raising Storms", Life "The Living Form", Matter "Shaping the Materials".
- **Especialidades** do livro. As anteriores de Correspondence, Entropy, Forces, Life e Matter tinham sido inventadas (por exemplo, "Apportation, Bi-location" ou "Electricity, Fire, Gravity").
- **Nomes dos postos** iguais aos títulos do livro, e **resumo de cada posto** com a primeira frase do livro.
- **Descrição** de cada Esfera com a abertura do livro.
- Wild Talent continua com as especialidades antigas; o livro não lista especialidades para ele.

## Quadros

O livro traz dois quadros nessas páginas. Ambos viraram listas, porque o renderizador do compêndio não suporta tabelas:

- **Data Connections** (p. 524): no PDF, as colunas "Data Mastery / Successes" e "Time Per Roll / Operator-Subject Connection" não ficam alinhadas por linha. Por isso o compêndio mostra as duas como listas separadas, sem supor qual linha corresponde a qual.
- **Primal Ventures** (p. 526): uma linha por escala de Venture, com o nível de Node equivalente, a conexão exigida e a nota de rodapé (marcada com †).

## Correções de digitação do livro

*Patten Spheres* → *Pattern Spheres*; *immdiate* → *immediate*; *runs of out* → *runs out of*; *EDE's experience* → *EDEs experience*; *Dimension Science 2* → *Dimensional Science 2*; *unifiedfield* → *unified-field*; *humansized* → *human-sized*; *longterm* → *long-term*; *valuegenerating* → *value-generating*; *meta-physical* → *metaphysical*.

## Verificação

- Script: cada frase do livro (pp. 511-528, até "Part III") foi procurada nos `.en.md`. Ficaram de fora só títulos colados à primeira frase pela extração, as correções de digitação acima e os dois quadros (reescritos como listas).
- `cargo test --features ssr --test all_tests`: 232 testes passam. Um teste esperava as especialidades inventadas de Forces ("Electricity", "Fire") e agora confere as do livro ("Motion", "Weather").
- `cargo check --target wasm32-unknown-unknown --features hydrate`: compila.
- **Não verificado:** a aparência da tela no navegador; o app não foi executado.
