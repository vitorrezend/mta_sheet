use leptos::*;
use crate::components::common::LabelColumn;
use super::info_header_fields::*;

#[component]
pub fn InfoHeader() -> impl IntoView {
    view! {
        <div class="info-header">
            <LabelColumn fields=COLUMN_1.to_vec() />
            <LabelColumn fields=COLUMN_2.to_vec() />
            <LabelColumn fields=COLUMN_3.to_vec() />
        </div>
    }
}
