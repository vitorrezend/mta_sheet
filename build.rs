fn main() {
    // Garante a existência dos diretórios de assets esperados pelo rust-embed
    let _ = std::fs::create_dir_all("target/site/pkg");
    let _ = std::fs::create_dir_all("styles");
}
