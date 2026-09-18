use crate::state::SheetFolder;

pub const FOLDER_ICON_PRESETS: &[&str] = &[
    "📁", "🔮", "🧙", "🧛", "🐺", "👻", "⚰️", "🎯", "🐉", "🗡️", "📜", "🎭", "🏛️", "⚡", "🏰", "💀", "👁️", "✨", "🩸", "🌿", "🏺", "🧪", "🌌",
];

pub fn get_folder_path(fid: &str, all_folders: &[SheetFolder]) -> String {
    let mut names = Vec::new();
    let mut curr = Some(fid.to_string());
    while let Some(c) = curr {
        if let Some(f) = all_folders.iter().find(|item| item.id == c) {
            names.push(f.name.clone());
            curr = f.parent_id.clone();
        } else {
            break;
        }
    }
    names.reverse();
    names.join(" / ")
}
