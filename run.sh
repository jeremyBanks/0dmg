#!/bin/bash
set -veux;

PARENT_CRATE="rust_issue_51798_example_parent";
PARENT_ID="arbitrary_parent_id";

CHILD_CRATE="rust_issue_51798_example_child";
CHILD_ID="arbitrary_child_id";

OUT_DIR="./target"
rm -rfv "$OUT_DIR";
mkdir -pv "$OUT_DIR";

RUSTC_WITH_DEFAULTS="
rustc
  --edition=2018
  --crate-type lib
  --out-dir $OUT_DIR
  -L dependency=$OUT_DIR
";

$RUSTC_WITH_DEFAULTS \
  child.rs \
  --crate-name "$CHILD_CRATE" \
  -C metadata="$CHILD_ID" \
  -C extra-filename=-"$CHILD_ID";

$RUSTC_WITH_DEFAULTS \
  parent.rs \
  --crate-name "$PARENT_CRATE" \
  -C metadata="$PARENT_ID" \
  -C extra-filename=-"$PARENT_ID" \
  --extern "$CHILD_CRATE"="$OUT_DIR"/lib"$CHILD_CRATE"-"$CHILD_ID".rlib;
