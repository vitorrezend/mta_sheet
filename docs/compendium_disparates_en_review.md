# Disparates — Textos em Inglês

**Data:** 2026-10-31
**Fonte:** M20, Capítulo Cinco, "Ascension Warriors": PDF pp. 203-224 (páginas impressas 202-223), do Ahl-i-Batin ao Wu Lung.
**Escopo:** texto integral em inglês, em Markdown, das 11 seitas da Aliança Disparate, no mesmo formato das Tradições e das Convenções. O português ainda não tem os textos.

## Resumo

- **11 textos** em `data/compendium/disparates/`.
- **Onde ficam:** na mesma seção das Tradições e das Convenções, que agora se chama **Facções da Ascensão** e tem 25 itens. As seitas aparecem num terceiro grupo da lista, "Os Disparates (pp. 202-223)", marcadas com ✧.
- **Links:**
  - as Esferas de afinidade levam ao verbete de cada Esfera;
  - as Práticas citadas em "Focus" levam ao verbete de cada Prática;
  - as Tradições e seitas citadas no texto levam aos seus verbetes (por exemplo: Akashayana no Ahl-i-Batin, Verbena, Ecstatics e Etherites nos Hollow Ones, Celestial Chorus nos Templários);
  - o "wild talent" dos Orphans leva à regra opcional de Talento Selvagem.
- **Fora do escopo:** a introdução da Parte IV e o quadro da Aliança Disparate (pp. 196-201) ficaram de fora, porque o pedido começava no Ahl-i-Batin.

## As 11 seitas

| Seita | Págs. | Esferas de afinidade |
|---|---|---|
| Ahl-i-Batin | 202-203 | Correspondence ou Mind — nunca Entropy |
| Bata’a | 204-205 | Spirit ou Life |
| Children of Knowledge | 206-207 | Tradicionalmente Matter; alguns grupos preferem Forces, Prime ou Entropy |
| Hollow Ones | 208-209 | Qualquer uma |
| Kopa Loei | 210-211 | Nenhuma: os Kopa Loei não acreditam em Esferas |
| Ngoma | 212-213 | Life, Mind, Prime ou Spirit |
| Orphans | 214-215 | Qualquer uma |
| Sisters of Hippolyta | 216-217 | Life ou Mind |
| Taftâni | 218-219 | Forces, Matter, Prime ou Spirit |
| Templar Knights | 220-221 | Forces, Life, Mind ou Prime |
| Wu Lung | 222-223 | Spirit, Forces, Matter ou Life (embora rejeitem esses termos) |

Cada texto segue a ordem do livro:
1. A epígrafe.
2. Um bloco com a página e as Esferas de afinidade.
3. A visão geral.
4. As seções **Organization**, **Initiation**, **Affinity Spheres**, **Focus** e **Stereotypes**. Nos estereótipos, The Traditions, The Technocracy e The Disparates (ou The Disparate Alliance) aparecem como itens separados.

Nos Kopa Loei, as três especialidades tradicionais (ali’i, kahuna e he ho’okele moana) viraram uma lista.

## Como aparece no app

- **Lista:** a mesma tela das Tradições e Convenções, com os Disparates no fim.
- **Links:** `mta://disparates/<id>` abre uma seita; os endereços `mta://traditions/...` também funcionam.
- **Busca:** encontra uma seita pelo nome em inglês ou português e por apelidos, como "Batini", "Solificati", "Darklings", "Hippolytoi" e "Knights Templar".
- **Em português:**
  - os nomes aparecem traduzidos onde há tradução usual (Filhos do Conhecimento, Ocos, Órfãos, Irmãs de Hipólita, Cavaleiros Templários);
  - os outros ficam como no original;
  - aparece o aviso "Texto disponível por enquanto apenas em inglês".

## Decisões de texto

- **Títulos:** os nomes das seitas e as epígrafes são imagens no PDF. Os nomes foram escritos à mão. O subtítulo usa outro nome que o próprio texto dá à seita (*The Subtle Ones*, *Solificati*, *The Darklings*, *The Weavers*, *Militia Christi*, *The Dragon Wizards* etc.).
- **Sisters of Hippolyta (Organization):** a extração do PDF embaralhou o começo da seção. O texto foi remontado na ordem certa: “Most Hippolytoi live together in conclaves that contain only women and children; Sisters who choose relationships with men must meet them outside the confines of the conclave. An elected council of seven Sisters known as the *epitropi* guides and governs each conclave.”
- **Correções:**

  | No livro | No compêndio |
  |---|---|
  | a particularly significance | a particular significance |
  | timehonored | time-honored |
  | familylike | family-like |
  | wizardpriests | wizard-priests |
  | Those-NotUs | Those-Not-Us |
  | nonChinese | non-Chinese |

  Outros deslizes do livro ficaram como estão, porque não atrapalham o sentido: “keep faith with in”, “Even their ties the Loa”, “According the Sisters’ version”.

## Arquivos

- `crates/mta_core/src/compendium/traditions.rs`:
  - `ALL_DISPARATES` e `find_disparate`;
  - `find_faction` agora busca Tradições, Convenções e Disparates.
- `src/components/compendium/views/traditions_view.rs`: terceiro grupo na lista.
- `navigation.rs`, `category_selector.rs` e `mod.rs`: endereço `mta://disparates/...`, nome da seção e páginas (pp. 147-223).
- `tests/suites/traditions_compendium_test.rs`: teste novo que confere:
  - os 11 textos e suas seções;
  - que todo link aponta para algo que existe;
  - o endereço `mta://disparates/...`;
  - a busca.

## Verificação

- Script comparando o livro com os textos, frase a frase: as 629 frases estão nos textos. Ficam diferentes só:
  - rodapés e epígrafes que a extração grudou no texto;
  - a frase do Ahl-i-Batin corrigida;
  - o trecho remontado das Sisters of Hippolyta.
- `cargo test --features ssr --test all_tests`: 236 testes passam.
- `cargo test -p mta_core`: 64 testes passam.
- `cargo check --target wasm32-unknown-unknown --features hydrate`: compila.
- **Não verificado:** a aparência da tela no navegador; o app não foi executado.
