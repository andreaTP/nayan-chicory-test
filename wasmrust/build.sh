#! /bin/bash
set -euxo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

rm -f ${SCRIPT_DIR}/../greet.wasm

(
  cd ${SCRIPT_DIR}
  cargo build --target wasm32-unknown-unknown --release
)
cp ${SCRIPT_DIR}/target/wasm32-unknown-unknown/release/greet.wasm ${SCRIPT_DIR}/../greet.wasm
