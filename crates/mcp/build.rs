//! Bakes the packaged build's expiry into `dharma-mcp` — read from
//! <repo root>/dharma-build.toml `[package]` (`expiry_days`/`expiry_hours`),
//! resolved against `.env` (env/dharma-build.env.example), same values
//! `xtask` prints at package time. `days == -1` means the build never
//! expires — no `DHARMA_EXPIRY` env is baked in, and `check_expiry()` in
//! `src/main.rs` is then a no-op.

fn main() {
    common::env::load_dotenv();

    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let Some(root) = manifest_dir.parent().and_then(|p| p.parent()) else { return };

    println!("cargo:rerun-if-changed={}", root.join("dharma-build.toml").display());
    println!("cargo:rerun-if-changed={}", root.join(".env").display());

    let Ok(cfg) = common::config::load_build_config(root) else { return };
    let (days, hours) = cfg.resolve_expiry();
    if days == -1 {
        return;
    }
    println!("cargo:rustc-env=DHARMA_EXPIRY={}", common::env::expiry_rfc3339_from_now(days, hours));
}
