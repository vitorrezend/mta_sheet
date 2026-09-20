use leptos::*;
use leptos_router::*;
use crate::i18n::{tr, Language};
use crate::state::get_system_stats;

#[component]
pub fn HomeShowcase(
    lang: Signal<Language>,
    is_logged_in: Signal<bool>,
) -> impl IntoView {
    let stats = create_local_resource(|| (), |_| async move { get_system_stats().await });

    view! {
        <section class="home-hero-section">
            <div class="home-hero-badge-container">
                <span class="home-hero-badge">
                    <span class="hero-badge-sparkle">"🔮"</span>
                    <span>{move || tr("home_hero_badge", lang.get())}</span>
                </span>
            </div>

            <h1 class="home-hero-title">
                {move || tr("home_hero_title", lang.get())}
            </h1>

            <p class="home-hero-subtitle">
                {move || tr("home_hero_subtitle", lang.get())}
            </p>

            <div class="home-hero-pillars">
                <span class="pillar-tag pillar-m20">{move || tr("home_pillar_m20", lang.get())}</span>
                <span class="pillar-tag pillar-gm">{move || tr("home_pillar_gm", lang.get())}</span>
                <span class="pillar-tag pillar-do">{move || tr("home_pillar_do", lang.get())}</span>
                <span class="pillar-tag pillar-rooms">{move || tr("home_pillar_rooms", lang.get())}</span>
                <span class="pillar-tag pillar-dossier">{move || tr("home_pillar_dossier", lang.get())}</span>
            </div>

            {move || if !is_logged_in.get() {
                Some(view! {
                    <div class="home-hero-actions">
                        <A href="/login" class="hero-cta-primary">
                            <span>{move || tr("home_hero_cta_start", lang.get())}</span>
                        </A>
                        <A href="/rooms" class="hero-cta-secondary">
                            <span>{move || tr("home_hero_cta_rooms", lang.get())}</span>
                        </A>
                    </div>
                })
            } else {
                None
            }}
        </section>

        <section class="home-stats-section">
            <div class="stats-header">
                <div class="stats-live-badge">
                    <span class="live-pulse-dot"></span>
                    <span>{move || tr("home_stats_badge", lang.get())}</span>
                </div>
                <h2 class="stats-title">{move || tr("home_stats_title", lang.get())}</h2>
                <p class="stats-subtitle">{move || tr("home_stats_sub", lang.get())}</p>
            </div>

            <div class="home-stats-grid">
                // Card 1: Usuários Despertos
                <div class="stat-card stat-card-users">
                    <div class="stat-card-top">
                        <span class="stat-card-icon">"👥"</span>
                        <span class="stat-card-badge stat-badge-users">"Awakened"</span>
                    </div>
                    <div class="stat-card-number">
                        {move || stats.get().and_then(|r| r.ok()).map(|s| s.total_users).unwrap_or(0)}
                    </div>
                    <div class="stat-card-label">{move || tr("home_stats_users", lang.get())}</div>
                    <div class="stat-card-desc">{move || tr("home_stats_users_desc", lang.get())}</div>
                </div>

                // Card 2: Salas de Crônica
                <div class="stat-card stat-card-rooms">
                    <div class="stat-card-top">
                        <span class="stat-card-icon">"🏰"</span>
                        <span class="stat-card-badge stat-badge-rooms">"Salas"</span>
                    </div>
                    <div class="stat-card-number">
                        {move || stats.get().and_then(|r| r.ok()).map(|s| s.total_rooms).unwrap_or(0)}
                    </div>
                    <div class="stat-card-label">{move || tr("home_stats_rooms", lang.get())}</div>
                    <div class="stat-card-desc">{move || tr("home_stats_rooms_desc", lang.get())}</div>
                </div>

                // Card 3: Fichas Mago: A Ascensão (M20)
                <div class="stat-card stat-card-m20">
                    <div class="stat-card-top">
                        <span class="stat-card-icon">"🔮"</span>
                        <span class="stat-card-badge stat-badge-m20">"M20"</span>
                    </div>
                    <div class="stat-card-number">
                        {move || stats.get().and_then(|r| r.ok()).map(|s| s.total_mage()).unwrap_or(0)}
                    </div>
                    <div class="stat-card-label">{move || tr("home_stats_m20", lang.get())}</div>
                    <div class="stat-pill-group">
                        <span class="stat-pill-public">
                            "🌐 " {move || stats.get().and_then(|r| r.ok()).map(|s| s.mage_sheets_public).unwrap_or(0)} " " {move || tr("home_stats_pub", lang.get())}
                        </span>
                        <span class="stat-pill-private">
                            "🔒 " {move || stats.get().and_then(|r| r.ok()).map(|s| s.mage_sheets_private).unwrap_or(0)} " " {move || tr("home_stats_priv", lang.get())}
                        </span>
                    </div>
                </div>

                // Card 4: Fichas Gods & Monsters
                <div class="stat-card stat-card-gm">
                    <div class="stat-card-top">
                        <span class="stat-card-icon">"🐾"</span>
                        <span class="stat-card-badge stat-badge-gm">"G&M"</span>
                    </div>
                    <div class="stat-card-number">
                        {move || stats.get().and_then(|r| r.ok()).map(|s| s.total_gm()).unwrap_or(0)}
                    </div>
                    <div class="stat-card-label">{move || tr("home_stats_gm", lang.get())}</div>
                    <div class="stat-pill-group">
                        <span class="stat-pill-public">
                            "🌐 " {move || stats.get().and_then(|r| r.ok()).map(|s| s.gm_sheets_public).unwrap_or(0)} " " {move || tr("home_stats_pub", lang.get())}
                        </span>
                        <span class="stat-pill-private">
                            "🔒 " {move || stats.get().and_then(|r| r.ok()).map(|s| s.gm_sheets_private).unwrap_or(0)} " " {move || tr("home_stats_priv", lang.get())}
                        </span>
                    </div>
                </div>
            </div>
        </section>


        <section class="home-features-showcase">
            <div class="showcase-header">
                <h2 class="showcase-title">{move || tr("home_showcase_title", lang.get())}</h2>
                <p class="showcase-subtitle">{move || tr("home_showcase_sub", lang.get())}</p>
            </div>

            <div class="showcase-cards-grid">
                <div class="showcase-card showcase-card-sheet">
                    <div class="showcase-card-icon-box">
                        <span class="showcase-card-icon">"📜"</span>
                        <span class="showcase-card-badge">"M20"</span>
                    </div>
                    <h3 class="showcase-card-title">{move || tr("home_feat1_title", lang.get())}</h3>
                    <p class="showcase-card-desc">{move || tr("home_feat1_desc", lang.get())}</p>
                </div>

                <div class="showcase-card showcase-card-rules">
                    <div class="showcase-card-icon-box">
                        <span class="showcase-card-icon">"⚖️"</span>
                        <span class="showcase-card-badge">"Auditoria"</span>
                    </div>
                    <h3 class="showcase-card-title">{move || tr("home_feat2_title", lang.get())}</h3>
                    <p class="showcase-card-desc">{move || tr("home_feat2_desc", lang.get())}</p>
                </div>

                <div class="showcase-card showcase-card-combat">
                    <div class="showcase-card-icon-box">
                        <span class="showcase-card-icon">"🥋"</span>
                        <span class="showcase-card-badge">"Akashayana"</span>
                    </div>
                    <h3 class="showcase-card-title">{move || tr("home_feat3_title", lang.get())}</h3>
                    <p class="showcase-card-desc">{move || tr("home_feat3_desc", lang.get())}</p>
                </div>

                <div class="showcase-card showcase-card-rooms">
                    <div class="showcase-card-icon-box">
                        <span class="showcase-card-icon">"🏰"</span>
                        <span class="showcase-card-badge">"Multijogador"</span>
                    </div>
                    <h3 class="showcase-card-title">{move || tr("home_feat4_title", lang.get())}</h3>
                    <p class="showcase-card-desc">{move || tr("home_feat4_desc", lang.get())}</p>
                </div>
            </div>
        </section>
    }
}
