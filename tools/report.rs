// ============================================================
//  report.rs — Relatório de Carregamento
// ============================================================

use colored::*;
use serde::Serialize;
use std::collections::HashMap;

use crate::loader::LibResult;

#[derive(Serialize)]
struct JsonReport<'a> {
    timestamp:     String,
    hostname:      String,
    total:         usize,
    loaded:        usize,
    missing:       usize,
    success_rate:  f32,
    by_group:      HashMap<String, GroupStats>,
    libraries:     Vec<LibEntry<'a>>,
}

#[derive(Serialize)]
struct GroupStats {
    total:  usize,
    loaded: usize,
}

#[derive(Serialize)]
struct LibEntry<'a> {
    group:    &'a str,
    name:     &'a str,
    soname:   &'a str,
    loaded:   bool,
    version:  Option<&'a str>,
    load_ms:  u128,
    error:    Option<&'a str>,
}

/// Imprime o relatório detalhado no terminal
pub fn print_report(results: &[LibResult]) {

    let groups = ["GNOME", "GNU", "XFCE", "KDE", "LXDE"];

    for group in &groups {
        let group_libs: Vec<&LibResult> = results.iter()
            .filter(|r| r.group == *group)
            .collect();

        let total  = group_libs.len();
        let loaded = group_libs.iter().filter(|r| r.loaded).count();
        let pct    = if total > 0 { (loaded as f32 / total as f32) * 100.0 } else { 0.0 };
        let bar    = progress_bar(pct, 20);

        println!(
            "  {:<8}  {}  {}/{} ({:.0}%)",
            group.cyan().bold(),
            bar,
            loaded.to_string().green(),
            total.to_string().yellow(),
            pct
        );
    }

    let total  = results.len();
    let loaded = results.iter().filter(|r| r.loaded).count();
    let missing = total - loaded;
    let pct = (loaded as f32 / total as f32) * 100.0;

    if missing > 0 {
        println!("\n{}", "Bibliotecas ausentes:".yellow().bold());
        for r in results.iter().filter(|r| !r.loaded) {
            println!("  {} [{:<6}] {}", "·".red(), r.group.dimmed(), r.name.dimmed());
        }
    }
}

/// Salva um relatório JSON em ~/.de_loader_report.json
pub fn save_json_report(results: &[LibResult]) -> Result<String, Box<dyn std::error::Error>> {
    use chrono::Local;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let hostname  = std::fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "desconhecido".to_string())
        .trim()
        .to_string();

    let total   = results.len();
    let loaded  = results.iter().filter(|r| r.loaded).count();
    let missing = total - loaded;

    let mut by_group: HashMap<String, GroupStats> = HashMap::new();
    for r in results {
        let e = by_group.entry(r.group.clone()).or_insert(GroupStats { total: 0, loaded: 0 });
        e.total += 1;
        if r.loaded { e.loaded += 1; }
    }

    let libraries: Vec<LibEntry> = results.iter().map(|r| LibEntry {
        group:   &r.group,
        name:    &r.name,
        soname:  &r.soname,
        loaded:  r.loaded,
        version: r.version.as_deref(),
        load_ms: r.load_time.as_millis(),
        error:   r.error.as_deref(),
    }).collect();

    let report = JsonReport {
        timestamp,
        hostname,
        total,
        loaded,
        missing,
        success_rate: (loaded as f32 / total as f32) * 100.0,
        by_group,
        libraries,
    };

    let path = dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join(".de_loader_report.json");

    let json = serde_json::to_string_pretty(&report)?;
    std::fs::write(&path, json)?;

    Ok(path.to_string_lossy().to_string())
}

fn progress_bar(pct: f32, width: usize) -> String {
    let filled = ((pct / 100.0) * width as f32) as usize;
    let empty  = width - filled.min(width);
    let bar = format!(
        "[{}{}]",
        "█".repeat(filled).green(),
        "░".repeat(empty).dimmed()
    );
    bar
}
