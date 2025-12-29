set shell := ["bash", "-uc"]

default:
    @just --list

run package *args='':
    @cargo run -p {{ package }} -- {{ args }}

build package *args='':
    @cargo build -p {{ package }} {{ args }}

run-server *args="": (run "ddns-server" args)

run-client *args="": (run "ddns-client" args)

build-server *args="--release": (build "ddns-server" args)

build-client *args="--release": (build "ddns-client" args)

generate-install:
    @cd contract && pnpm install && cd ..

generate-client:
    @cd contract && pnpm gen:client && cd ..

generate-server:
    @cd contract && pnpm gen:server && cd ..

generate-all:
    @cd contract && pnpm gen:all && cd ..

clean-generate:
    @rm -rf gen/client
    @rm -rf gen/server

clean:
    @cargo clean
