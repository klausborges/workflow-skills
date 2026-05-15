#!/bin/sh
set -eu

mode="write"
if [ "${1:-}" = "--check" ]; then
  mode="check"
elif [ "${1:-}" != "" ]; then
  echo "usage: $0 [--check]" >&2
  exit 2
fi

root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
shared_dir="$root/skills/_shared"
status=0

for skill_dir in "$root"/skills/*; do
  [ -d "$skill_dir" ] || continue
  [ "$(basename "$skill_dir")" != "_shared" ] || continue
  [ -f "$skill_dir/SKILL.md" ] || continue

  target_dir="$skill_dir/references"

  if [ "$mode" = "write" ]; then
    mkdir -p "$target_dir"
    for source in "$shared_dir"/*.md; do
      cp "$source" "$target_dir"/
    done
    continue
  fi

  if [ ! -d "$target_dir" ]; then
    echo "missing generated references: ${target_dir#$root/}" >&2
    status=1
    continue
  fi

  for source in "$shared_dir"/*.md; do
    target="$target_dir/$(basename "$source")"
    if [ ! -f "$target" ]; then
      echo "missing generated reference: ${target#$root/}" >&2
      status=1
    elif ! cmp -s "$source" "$target"; then
      echo "stale generated reference: ${target#$root/}" >&2
      status=1
    fi
  done

  for target in "$target_dir"/*.md; do
    [ -e "$target" ] || continue
    source="$shared_dir/$(basename "$target")"
    if [ ! -f "$source" ]; then
      echo "orphan generated reference: ${target#$root/}" >&2
      status=1
    fi
  done
done

if [ "$mode" = "check" ] && [ "$status" -ne 0 ]; then
  echo "run: mise run sync-references" >&2
fi

exit "$status"
