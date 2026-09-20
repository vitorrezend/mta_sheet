use leptos::*;
use leptos_router::*;
use crate::state::{clone_sheet, get_public_sheets, toggle_sheet_like, CharacterSummary};

fn handle_clone_sheet(
    sheet_id: String,
    cloning_id: ReadSignal<Option<String>>,
    set_cloning_id: WriteSignal<Option<String>>,
    log_source: &'static str,
) {
    if cloning_id.get().is_some() {
        return;
    }
    let navigate = use_navigate();
    set_cloning_id.set(Some(sheet_id.clone()));
    spawn_local(async move {
        match clone_sheet(sheet_id, None).await {
            Ok(new_id) => {
                crate::logging::log_client(
                    "user_actions",
                    "INFO",
                    &format!("Ficha clonada via {}", log_source),
                    Some(&format!("new_id={}", new_id)),
                );
                navigate(&format!("/sheet/{}", new_id), Default::default());
            }
            Err(e) => {
                crate::logging::log_client(
                    "errors",
                    "ERROR",
                    &format!("Erro ao clonar ficha ({})", log_source),
                    Some(&e.to_string()),
                );
                set_cloning_id.set(None);
            }
        }
    });
}

#[component]
fn FeedCard(
    character: CharacterSummary,
    cloning_id: ReadSignal<Option<String>>,
    set_cloning_id: WriteSignal<Option<String>>,
) -> impl IntoView {
    let auth = use_context::<crate::AuthContext>();
    let user = auth.map(|a| a.user).unwrap_or_else(|| Signal::derive(|| None));

    let (is_liked, set_is_liked) = create_signal(character.is_liked);
    let (likes_count, set_likes_count) = create_signal(character.likes_count);
    let (is_toggling_like, set_is_toggling_like) = create_signal(false);

    let id_for_clone = character.id.clone();
    let id_for_view = character.id.clone();
    let sheet_id_for_like = character.id.clone();
    let is_gm = character.sheet_type.starts_with("gods_and_monsters");
    let splat_badge = if is_gm { "Deuses & Monstros" } else { "Mago: M20" };
    let badge_class = if is_gm { "badge-gm" } else { "badge-mage" };
    let author_opt = character.author_username.clone();
    let has_portrait = !character.photo_url.trim().is_empty();
    let photo_style = format!("object-position: {}% {}%;", character.photo_focus_x, character.photo_focus_y);

    let on_like_click = move |_| {
        if is_toggling_like.get() {
            return;
        }
        if user.get().is_none() {
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(window) = web_sys::window() {
                    let _ = window.alert_with_message("Faça login para curtir personagens da comunidade!");
                }
            }
            return;
        }

        let sid = sheet_id_for_like.clone();
        set_is_toggling_like.set(true);
        let prev_liked = is_liked.get();
        let prev_count = likes_count.get();
        if prev_liked {
            set_is_liked.set(false);
            set_likes_count.set((prev_count - 1).max(0));
        } else {
            set_is_liked.set(true);
            set_likes_count.set(prev_count + 1);
        }

        spawn_local(async move {
            match toggle_sheet_like(sid).await {
                Ok(res) => {
                    set_is_liked.set(res.is_liked);
                    set_likes_count.set(res.likes_count);
                    set_is_toggling_like.set(false);
                }
                Err(_) => {
                    set_is_liked.set(prev_liked);
                    set_likes_count.set(prev_count);
                    set_is_toggling_like.set(false);
                }
            }
        });
    };

    let cur_id_for_clone_check = character.id.clone();
    let is_being_cloned = Signal::derive(move || {
        let cur_cloning = cloning_id.get();
        cur_cloning.as_ref() == Some(&cur_id_for_clone_check)
    });

    view! {
        <article class="feed-card">
            <div class="feed-card-portrait-box">
                {if has_portrait {
                    view! {
                        <img
                            src=character.photo_url.clone()
                            alt=character.name.clone()
                            class="feed-card-portrait-img"
                            style=photo_style
                            loading="lazy"
                        />
                    }.into_view()
                } else {
                    view! {
                        <div class="feed-card-portrait-placeholder">
                            <span style="font-size: 2.2rem;">{if is_gm { "🐉" } else { "🔮" }}</span>
                            <span style="font-size: 0.8rem; opacity: 0.7;">"Sem Retrato"</span>
                        </div>
                    }.into_view()
                }}
                <div class="feed-card-portrait-gradient"></div>
                <span class=format!("feed-card-splat-badge {}", badge_class)>{splat_badge}</span>
            </div>

            <div class="feed-card-body">
                <div class="feed-card-name-row">
                    <h3 class="feed-card-name">
                        {if character.name.trim().is_empty() { "Sem Nome".to_string() } else { character.name.clone() }}
                    </h3>
                </div>

                {if let Some(auth) = author_opt {
                    let user_href = format!("/user/{}", auth);
                    let auth_label = format!("@{}", auth);
                    view! {
                        <A href=user_href class="feed-card-author-link">
                            <span>"👤"</span>
                            <span>{auth_label}</span>
                        </A>
                    }.into_view()
                } else {
                    view! { <span class="feed-card-author-link" style="opacity: 0.6;">"👤 Anônimo"</span> }.into_view()
                }}

                <div class="feed-card-tags-row">
                    {if !character.tradition.trim().is_empty() {
                        view! { <span class="feed-tag">{character.tradition.clone()}</span> }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }}
                    {if !character.essence.trim().is_empty() {
                        view! { <span class="feed-tag">{character.essence.clone()}</span> }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }}
                </div>

                <div class="feed-card-stats-row">
                    <div class="feed-stat-pill">
                        <span class="feed-stat-label">"Arete"</span>
                        <span class="feed-stat-val">{character.arete}</span>
                    </div>
                    <div class="feed-stat-pill">
                        <span class="feed-stat-label">"Vontade"</span>
                        <span class="feed-stat-val">{character.willpower}</span>
                    </div>
                </div>

                <div class="feed-card-spheres-row">
                    {character.spheres.into_iter().filter(|(_, val)| *val > 0).map(|(name, val)| {
                        view! {
                            <span class="feed-sphere-chip">
                                <span>{name}</span>
                                <strong>{val}</strong>
                            </span>
                        }
                    }).collect_view()}
                </div>

                <div class="feed-card-footer">
                    <span class="feed-card-date">
                        {if character.updated_at.len() >= 10 {
                            character.updated_at[..10].to_string()
                        } else {
                            character.updated_at.clone()
                        }}
                    </span>
                    <div class="feed-card-actions">
                        <button
                            type="button"
                            class="feed-action-btn feed-like-btn"
                            class:liked=move || is_liked.get()
                            on:click=on_like_click
                            aria-label=move || format!("Curtir ficha ({} curtidas)", likes_count.get())
                            title=move || if is_liked.get() { "Descurtir ficha" } else { "Curtir ficha" }
                        >
                            <span class="like-heart">{move || if is_liked.get() { "❤️" } else { "🤍" }}</span>
                            <span class="like-count">{move || likes_count.get()}</span>
                        </button>
                        <A href=format!("/sheet/{}", id_for_view) class="feed-action-btn feed-view-btn">
                            "👁️ Ver"
                        </A>
                        <button
                            type="button"
                            class="feed-action-btn feed-clone-btn"
                            disabled=move || is_being_cloned.get()
                            on:click=move |_| handle_clone_sheet(id_for_clone.clone(), cloning_id, set_cloning_id, "Feed público")
                        >
                            {move || if is_being_cloned.get() { "⏳ Clonando..." } else { "📋 Clonar" }}
                        </button>
                    </div>
                </div>
            </div>
        </article>
    }
}

#[component]
pub fn FeedPage() -> impl IntoView {
    let public_sheets = create_local_resource(|| (), |_| async move {
        get_public_sheets().await
    });

    let (search_query, set_search_query) = create_signal(String::new());
    let (splat_filter, set_splat_filter) = create_signal("all");
    let (cloning_id, set_cloning_id) = create_signal(Option::<String>::None);

    view! {
        <div class="feed-container">
            <leptos_meta::Title text="Feed da Comunidade | MTA Sheet" />

            <div class="page-breadcrumb-nav">
                <A href="/" class="breadcrumb-back-link">
                    "← Voltar para Minhas Fichas"
                </A>
                <span class="breadcrumb-separator">"•"</span>
                <span class="breadcrumb-current">"Feed da Comunidade"</span>
            </div>

            <header class="feed-header">
                <h1>"🌐 Feed da Comunidade"</h1>
                <p>"Explore personagens públicos criados por outros Despertos, estude seus paradigmas e clone fichas para usar em suas próprias crônicas."</p>
            </header>

            // Controls & Filters
            <section class="feed-controls-section">
                <div class="feed-search-row">
                    <div class="feed-search-input-wrapper">
                        <span class="feed-search-icon">"🔍"</span>
                        <input
                            type="text"
                            class="feed-search-input"
                            placeholder="Buscar por nome, autor, tradição ou essência..."
                            prop:value=move || search_query.get()
                            on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                <div class="feed-filter-pills">
                    <span class="feed-filter-label">"Sistema:"</span>
                    <button
                        type="button"
                        class="feed-pill-btn"
                        class:active=move || splat_filter.get() == "all"
                        on:click=move |_| set_splat_filter.set("all")
                    >
                        "✨ Todos"
                    </button>
                    <button
                        type="button"
                        class="feed-pill-btn"
                        class:active=move || splat_filter.get() == "mage"
                        on:click=move |_| set_splat_filter.set("mage")
                    >
                        "🔮 Mago: A Ascensão (M20)"
                    </button>
                    <button
                        type="button"
                        class="feed-pill-btn"
                        class:active=move || splat_filter.get() == "gm"
                        on:click=move |_| set_splat_filter.set("gm")
                    >
                        "🐉 Deuses & Monstros"
                    </button>
                </div>
            </section>

            // Feed Content Grid
            <Transition fallback=move || view! { <div class="feed-empty-state"><p>"Carregando fichas públicas da comunidade..."</p></div> }>
                {move || {
                    let sheets = match public_sheets.get() {
                        Some(Ok(list)) => list,
                        Some(Err(e)) => return view! {
                            <div class="feed-empty-state">
                                <span class="feed-empty-icon">"⚠️"</span>
                                <p>{format!("Erro ao carregar o feed: {}", e)}</p>
                            </div>
                        }.into_view(),
                        None => return view! {
                            <div class="feed-empty-state"><p>"Carregando..."</p></div>
                        }.into_view(),
                    };

                    let query = search_query.get().trim().to_lowercase();
                    let current_filter = splat_filter.get();

                    let filtered: Vec<CharacterSummary> = sheets.into_iter().filter(|s| {
                        let is_gm = s.sheet_type.starts_with("gods_and_monsters");
                        if current_filter == "mage" && is_gm {
                            return false;
                        }
                        if current_filter == "gm" && !is_gm {
                            return false;
                        }

                        if !query.is_empty() {
                            let name_match = s.name.to_lowercase().contains(&query);
                            let trad_match = s.tradition.to_lowercase().contains(&query);
                            let ess_match = s.essence.to_lowercase().contains(&query);
                            let auth_match = s.author_username.as_ref().map(|a| a.to_lowercase().contains(&query)).unwrap_or(false);
                            name_match || trad_match || ess_match || auth_match
                        } else {
                            true
                        }
                    }).collect();

                    let total_count = filtered.len();

                    if filtered.is_empty() {
                        view! {
                            <div class="feed-empty-state">
                                <span class="feed-empty-icon">"📜"</span>
                                <h3>"Nenhuma ficha pública encontrada"</h3>
                                <p>"Nenhum personagem corresponde aos filtros selecionados ou nenhuma ficha foi tornada pública ainda."</p>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div>
                                <div class="feed-stats-bar" style="margin-bottom: 1.5rem;">
                                    <span>{format!("Exibindo {} personagem(ns) público(s)", total_count)}</span>
                                </div>
                                <div class="feed-cards-grid">
                                    {filtered.into_iter().map(|character| {
                                        view! {
                                            <FeedCard
                                                character=character
                                                cloning_id=cloning_id
                                                set_cloning_id=set_cloning_id
                                            />
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }.into_view()
                    }
                }}
            </Transition>
        </div>
    }
}
