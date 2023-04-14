set windows-shell := ["powershell"]

# run the server
run: build
    cargo run --package scrab

# build everything
build: emit-ts-types
    cd frontend; npm run build
    cargo build --package scrab

# build the frontend/src/types.d.ts file
emit-ts-types:
    cd frontend/src; cargo run --package scrab_public_types --bin emit_types