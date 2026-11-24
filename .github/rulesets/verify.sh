#!/usr/bin/env bash
# Verify that the GitHub ruleset matches the versioned file. Fails on any error or difference.
set -euo pipefail
file="${1:-.github/rulesets/main.json}"
name="${2:-protect-main}"
repo="$(gh repo set-default --view)"
[ -n "$repo" ] || { echo "verify: no gh default repository set" >&2; exit 1; }
id="$(gh api "repos/$repo/rulesets" --jq ".[] | select(.name==\"$name\") | .id")"
[ -n "$id" ] || { echo "verify: ruleset '$name' not found" >&2; exit 1; }
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
gh api "repos/$repo/rulesets/$id" \
  | jq -S '{name, target, enforcement, bypass_actors, conditions, rules}' > "$tmp/remote.json"
jq -S . "$file" > "$tmp/local.json"
diff -u "$tmp/local.json" "$tmp/remote.json"
