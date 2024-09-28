License: MIT-0

1、
cargo build --profile production --features runtime-benchmarks

2、
./target/production/solochain-template-node benchmark pallet \
--chain dev \
--wasm-execution=compiled \
--pallet pallet_kitties \
--extrinsic "*" \
--steps 20 \
--repeat 10 \
--output pallets/kitties/src/weights.rs \
--template .maintain/frame-weight-template.hbs
3、
cargo test
