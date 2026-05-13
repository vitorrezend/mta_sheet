# Design System & Layout Rules

Este documento estabelece os princípios de layout e padrões técnicos para a ficha de Mago: A Ascensão (MTA), garantindo consistência visual e facilidade de manutenção.

## 1. Sistema de Grid e Colunas

Para manter o alinhamento vertical perfeito entre diferentes seções (como Cabeçalho e Atributos), siga sempre estas regras:

- **Grid Padrão:** Use um layout de 3 colunas.
- **Definição CSS:** `display: grid; grid-template-columns: repeat(3, 1fr);`
- **Espaçamento (Gap):** Use um gap consistente de `0.6rem` (aprox. 10px).
- **Compensação de Padding:** Componentes que NÃO estão dentro de uma borda (como o `InfoHeader`) devem usar `padding: 0 0.8rem;` para alinhar com o conteúdo interno de componentes que ESTÃO dentro de uma `group-box`.

## 2. Arquitetura de Campos de Texto (Labels)

### LabelField
- **Layout:** `display: flex; justify-content: space-between; align-items: flex-end;`
- **Alinhamento da Label:** `text-align: left;` para o texto da etiqueta.
- **Comportamento do Input:** Inputs devem usar `flex: 1; width: 0; min-width: 0;` para preencher o espaço sem "empurrar" a coluna para fora.
- **Limites:** Máximo de 30 caracteres (`maxlength="30"`).
- **Excesso de Texto:** Use `text-overflow: ellipsis; overflow: hidden;` e forneça um balãozinho (tooltip) usando a classe `.tooltip-text`.

### Agrupamento
- **Modularidade:** Use `LabelColumn` para renderizar conjuntos de campos, separando a lógica de exibição da definição dos dados.

## 3. Espaçamento e Ritmo Vertical

- **Entre Grupos:** Use `margin-top: 1rem;` para elementos `group-box` para mantê-los próximos mas distintos.
- **Entre Linhas:** Use `gap: 0.4rem;` entre campos individuais dentro de uma coluna.

## 4. Padrões de Persistência

- **Chaves Internas:** Use sempre chaves ASCII simples para o `LocalStorage` (ex: use `"Tradicao"` em vez de `"Tradição"` como chave).
- **Labels da UI:** Acentos são encorajados para os nomes exibidos na tela para manter a ortografia correta em português.

---
> [!IMPORTANT]
> Sempre que criar um novo bloco de informações, verifique se a primeira e a última coluna alinham verticalmente com o bloco de Atributos.
