use serde_json::Value;
use std::collections::HashSet;
use std::process::Command;

// crate di I/O vietati nel dominio (constitution III): si aggiorna quando ne entra uno nuovo nel workspace
const FORBIDDEN: [&str; 4] = ["tokio", "sqlx", "reqwest", "axum"];

#[test]
fn the_domain_does_not_depend_on_io_crates() {
    let out = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--locked"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("cargo metadata non eseguibile");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let meta: Value = serde_json::from_slice(&out.stdout).unwrap();
    let nodi = meta["resolve"]["nodes"].as_array().unwrap();
    let root = nodi
        .iter()
        .find(|n| n["id"].as_str().unwrap().contains("{{DOMAIN_CRATE}}"))
        .expect("{{DOMAIN_CRATE}} assente");
    let mut seen = HashSet::new();
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        for d in n["deps"].as_array().unwrap() {
            // archi solo dev esclusi: kind null = normale/build
            if d["dep_kinds"]
                .as_array()
                .unwrap()
                .iter()
                .all(|k| k["kind"] == "dev")
            {
                continue;
            }
            let name = d["name"].as_str().unwrap();
            assert!(
                !FORBIDDEN.contains(&name),
                "dipendenza vietata nel dominio: {name}"
            );
            if seen.insert(d["pkg"].as_str().unwrap().to_string()) {
                stack.push(nodi.iter().find(|x| x["id"] == d["pkg"]).unwrap());
            }
        }
    }
}
