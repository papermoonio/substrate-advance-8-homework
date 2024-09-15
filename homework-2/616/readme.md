# building 
`cargo build --profile=production --features runtime-benchmarks`
![img](./build.PNG)
# testing 
`cargo test --profile=production`
![img](./test.PNG)

# bencchmark testing
```bash
./target/production/solochain-template-node benchmark pallet \
--chain dev \
--execution=wasm \
--wasm-execution=compiled \
--pallet pallet_poe \
--extrinsic "*" \
--steps 20 \
--repeat 10 \
--output pallets/poe/src/weights.rs \
--template .maintain/frame-weight-template.hbs
```

