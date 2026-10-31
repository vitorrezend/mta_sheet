# Revisão de Naturezas e Comportamentos — Textos em Inglês

**Data:** 2026-10-30
**Fonte:** M20, Capítulo Seis, "Personality Archetypes: Nature and Demeanor": do fim da página 268 ao começo da 274 do PDF (páginas impressas 267 a 273).
**Escopo:** conferir se o compêndio tem todos os arquétipos com o texto do livro em inglês, a regra de recuperação de Força de Vontade e as páginas. O português não foi alterado.

## Resultado

- **Todos os 20 arquétipos do livro estão no compêndio**, nenhum a mais e nenhum a menos.
- **O texto em inglês já era idêntico ao livro.** Uma conferência frase a frase encontrou as 267 frases dessas páginas no compêndio: introdução, descrição, força, fraqueza e recuperação de Força de Vontade de cada arquétipo. Nada foi reescrito.
- **3 páginas estavam erradas** e foram corrigidas.
- **O quadro de regras** ganhou títulos, a página de cada parte e a lista dos 20 arquétipos com página, força, fraqueza e link.
- **A tela em inglês** tinha resumos escritos para o app; agora mostra as frases do livro.

## Recuperação de Força de Vontade (p. 268)

A regra geral está no quadro "Personality Archetypes: Nature and Demeanor", agora na seção **Regaining Willpower (p. 268)**:

> When you accomplish something that’s in tune with your Nature, your Storyteller might give you one to three Willpower points to replace the ones you’ve spent over the course of the story. This benefit applies only to refreshing a depleted Willpower Trait – you don’t get new Willpower dots for acting according to your Nature.

Cada arquétipo tem a sua condição de recuperação (a linha "Regain Willpower when…" do livro) no card **Willpower Regain**. Na tela de cada arquétipo, a seção que antes trazia um parágrafo montado pelo app ("If chosen as your Nature…") agora se chama **Nature and Willpower (M20, p. 268)** e reproduz a regra do livro.

## Páginas

| Arquétipo | Pág. | | Arquétipo | Pág. |
|---|---|---|---|---|
| Activist | 268 (era 267-268) | | Mad Scientist | 270-271 |
| Benefactor | 268 | | Martyr | 271 |
| Contrary | 268 | | Monster | 271 |
| Crusader | 268-269 | | Prophet | 272 (era 271-272) |
| Hacker | 269 | | Rogue | 272 |
| Idealist | 269-270 | | Sensualist | 272 |
| Innovator | 270 | | Survivor | 272 |
| Kid | 270 | | Traditionalist | 272 |
| Loner | 270 (era 270-271) | | Trickster | 272-273 |
| Machine | 270 | | Visionary | 273 |

O quadro geral (Nature and Demeanor) fica nas pp. 267-268.

## O que mudou nos arquivos

- `crates/mta_core/src/compendium/archetypes.rs`: páginas de Activist, Loner e Prophet.
- `data/compendium/archetypes/theory_nature_demeanor.en.md`: mesmo texto do livro, dividido em **Nature and Demeanor**, **Regaining Willpower** e **Strengths and Weaknesses**, com a página de cada parte, mais a lista **The Archetypes (pp. 268-273)**.
- `src/components/compendium/views/archetypes_view.rs`:
  - o quadro de regras usa o mesmo renderizador dos outros quadros do compêndio, com títulos e links;
  - em inglês, os três destaques do quadro (Demeanor, Nature, Willpower Regain) e a seção de cada arquétipo usam frases do livro, com a página.
- `data/compendium/archetypes.json`: regenerado pelos testes com as páginas novas.

## Verificação

- Script comparando o livro com os textos do compêndio, nos dois sentidos: nenhuma frase faltando e nenhuma frase que não seja do livro.
- `cargo test --features ssr --test all_tests`: 232 testes passam. Os testes do `mta_core` sobre arquétipos também passam.
- `cargo check --target wasm32-unknown-unknown --features hydrate`: compila.
- **Não verificado:** a aparência da tela no navegador; o app não foi executado.
