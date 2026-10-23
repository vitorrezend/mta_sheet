use leptos::*;
use crate::rooms::{ChatMessageKind, ChatRollData, RoomChatMessage};

/// Converte o horário UTC do servidor (`YYYY-MM-DD HH:MM:SS`) para `HH:MM` local.
///
/// O Safari não entende datas com espaço no lugar do `T` (`new Date("2026-10-22 17:00:00")`
/// vira "Invalid Date"), então montamos sempre o formato ISO completo com `Z`.
pub fn format_local_time(created_at: &str) -> String {
    let fallback = || created_at.get(11..16).unwrap_or_default().to_string();
    if created_at.len() < 16 {
        return fallback();
    }
    #[cfg(target_arch = "wasm32")]
    {
        let iso = format!("{}Z", created_at.replacen(' ', "T", 1).trim_end_matches('Z'));
        let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(&iso));
        if !date.get_time().is_nan() {
            return format!("{:02}:{:02}", date.get_hours(), date.get_minutes());
        }
    }
    fallback()
}

#[component]
pub fn ChatMessageItem(msg: RoomChatMessage, #[prop(into)] is_mine: bool) -> impl IntoView {
    let kind = msg.message_type;
    let time = format_local_time(&msg.created_at);
    let (kind_class, pill) = match kind {
        ChatMessageKind::Roll => ("bubble-roll", "🎲 ROLAGEM"),
        ChatMessageKind::Ic => ("bubble-ic", "🎭 EM CENA"),
        ChatMessageKind::System => ("bubble-system", "📣 SISTEMA"),
        ChatMessageKind::Ooc => ("bubble-ooc", "💬 OFF"),
    };

    let avatar = match msg.sender_avatar {
        Some(src) => view! { <img class="sender-avatar" src=src alt="" loading="lazy" decoding="async" /> }.into_view(),
        None => view! { <div class="sender-avatar-placeholder" aria-hidden="true">"👤"</div> }.into_view(),
    };
    let names = match msg.sender_character {
        Some(character) => view! {
            <span class="char-name">{character}</span>
            <span class="player-sub">"(" {msg.sender_name} ")"</span>
        }.into_view(),
        None => view! { <span class="player-name">{msg.sender_name}</span> }.into_view(),
    };
    let body = match msg.roll_data {
        Some(roll) => view! { <RollCard roll=roll /> }.into_view(),
        None => view! { <div class="bubble-text-content">{msg.content}</div> }.into_view(),
    };

    view! {
        <article class=format!("chat-bubble {}", kind_class) class:bubble-mine=is_mine>
            <header class="bubble-header">
                <div class="sender-identity-group">
                    {avatar}
                    <div class="sender-names">{names}</div>
                </div>
                <div class="bubble-meta">
                    <span class=format!("msg-type-pill pill-{}", kind.as_str())>{pill}</span>
                    <time class="bubble-time" datetime=msg.created_at.clone()>{time}</time>
                </div>
            </header>
            {body}
        </article>
    }
}

#[component]
pub fn RollCard(roll: ChatRollData) -> impl IntoView {
    let difficulty = roll.difficulty;
    let dice = roll.dice_results.iter().map(|&die| {
        let class = if die == 10 {
            "die-box die-ten"
        } else if die == 1 {
            "die-box die-one"
        } else if die >= difficulty {
            "die-box die-success"
        } else {
            "die-box die-fail"
        };
        view! { <div class=class>{die}</div> }
    }).collect_view();

    let outcome = if roll.is_botch {
        view! { <span class="outcome-text">"💥 FALHA CRÍTICA (BOTCH)!"</span> }.into_view()
    } else if roll.successes > 0 {
        view! { <span class="outcome-text text-success">"✨ " {roll.successes} " Sucesso(s)!"</span> }.into_view()
    } else {
        view! { <span class="outcome-text text-neutral">"Falha Simples (0 sucessos)"</span> }.into_view()
    };

    view! {
        <div class="roll-result-card" class:botch-card=roll.is_botch>
            <div class="roll-header-line">
                <span class="roll-action-label">{roll.label}</span>
                <span class="roll-diff-badge">
                    {roll.dice_count} "d10 · Dif " {difficulty}
                    {roll.specialty.then_some(" · ⭐")}
                </span>
            </div>
            <div class="dice-grid">{dice}</div>
            <div class="roll-outcome-banner" class:outcome-botch=roll.is_botch>{outcome}</div>
        </div>
    }
}
