#!/usr/bin/env bash
# KatWork fork check: the KatWork build profile leaves the gated crates out of the build graph.
#
#   scripts/katwork-profile.sh   reads scripts/katwork-profile.tsv and exits 1 on any failed row
#
# Each row: <package> <native|wasm32> <katwork-features|-> <crate> <present|absent>
#   The KatWork build is `--no-default-features [--features <katwork-features>]`; with it, <crate> must not be in the
#   normal-dependency graph of <package>. The last column is the control: with the default features (the upstream
#   build) <crate> must be `present`, which proves the gate defaults on and that <crate> is spelt right; `absent`
#   marks a row whose crate is not in that graph even upstream (the assertion is then vacuous, and is printed so).
#   An `absent` control accepts only the "nothing to print" form: it proves <crate> is spelt right (it is in the
#   workspace's resolve, for another target or edge kind); a misspelt crate gives "did not match" and fails.
#
# `cargo tree -i X` has two "not in the graph" outputs: exit 101 with "did not match any packages" (X is not in
# the resolved graph) and exit 0 with "nothing to print" (X is only in other targets or edge kinds). A typo gives
# the first, which is why every row with `present` checks the default build too.
set -u
cd "$(git rev-parse --show-toplevel)" || exit 1
table=scripts/katwork-profile.tsv
fail=0 n=0

tree() {  # tree <package> <target> <crate> [cargo args...]; sets rc and out
  local pkg=$1 target=$2 crate=$3; shift 3
  local args=(tree --locked -p "$pkg" -e normal -i "$crate" "$@")
  [ "$target" = wasm32 ] && args+=(--target wasm32-unknown-unknown)
  out=$(cargo "${args[@]}" 2>&1); rc=$?
}
absent() {
  { [ "$rc" = 101 ] && grep -q 'did not match any packages' <<<"$out"; } ||
    { [ "$rc" = 0 ] && grep -q 'nothing to print' <<<"$out"; }
}

# `|| [ -n "$pkg" ]` keeps a last row that has no final newline.
while read -r pkg target feats crate control || [ -n "$pkg" ]; do
  case "$pkg" in ''|'#'*) continue ;; esac
  n=$((n + 1))
  case "$target" in
    native|wasm32) ;;
    *) echo "FAIL: row $n: target must be native or wasm32, not '$target'"; fail=1; continue ;;
  esac
  if [ "$feats" = - ]; then extra=(); else extra=(--features "$feats"); fi
  tree "$pkg" "$target" "$crate" --no-default-features "${extra[@]}"
  if absent; then
    why=$([ "$rc" = 101 ] && echo "not in the resolved graph" || echo "nothing to print")
    echo "ok: $crate is not in $pkg ($target, KatWork features: $feats): $why"
  else
    echo "FAIL: $crate is in $pkg ($target, KatWork features: $feats):"; sed 's/^/  /' <<<"$out" | head -20; fail=1
  fi
  tree "$pkg" "$target" "$crate"
  case "$control" in
    present)
      if [ "$rc" = 0 ] && grep -q "^$crate v" <<<"$out"; then
        echo "ok: control: $crate is in $pkg with the default features"
      else
        echo "FAIL: control: $crate is not in $pkg ($target) with the default features (gate not on by default, or a typo):"
        sed 's/^/  /' <<<"$out" | head -10; fail=1
      fi ;;
    absent)
      if [ "$rc" = 0 ] && grep -q 'nothing to print' <<<"$out"; then echo "note: $crate is not in $pkg ($target) upstream either; this row is vacuous"
      else echo "FAIL: control: $crate is in $pkg ($target) upstream, or is not in the workspace at all (a typo?), but the row says absent:"; sed 's/^/  /' <<<"$out" | head -10; fail=1; fi ;;
    *) echo "FAIL: row $n: control must be present or absent, not '$control'"; fail=1 ;;
  esac
done < "$table"

[ "$n" -gt 0 ] || { echo "FAIL: $table has no rows"; fail=1; }
[ "$fail" = 0 ] && echo "katwork-profile: ok ($n rows)"
exit "$fail"
