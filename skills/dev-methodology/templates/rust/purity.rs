use serde_json::Value;
use std::collections::HashSet;
use std::process::Command;

// I/O crates forbidden in the domain (constitution III): update the list when a new one enters the workspace
const FORBIDDEN: [&str; 4] = ["tokio", "sqlx", "reqwest", "axum"];

#[test]
fn the_domain_does_not_depend_on_io_crates() {
    let out = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--locked"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("cargo metadata failed");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let meta: Value = serde_json::from_slice(&out.stdout).unwrap();
    let nodes = meta["resolve"]["nodes"].as_array().unwrap();
    let root = nodes
        .iter()
        .find(|n| n["id"].as_str().unwrap().contains("{{DOMAIN_CRATE}}"))
        .expect("{{DOMAIN_CRATE}} missing");
    let mut seen = HashSet::new();
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        for d in n["deps"].as_array().unwrap() {
            // dev-only edges skipped: kind null = normal/build
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
                "forbidden dependency in the domain: {name}"
            );
            if seen.insert(d["pkg"].as_str().unwrap().to_string()) {
                stack.push(nodes.iter().find(|x| x["id"] == d["pkg"]).unwrap());
            }
        }
    }
}
