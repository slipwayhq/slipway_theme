publisher := "slipwayhq"
name := "theme"

build configuration="debug":
  rm -rf artifacts
  mkdir -p artifacts/{{publisher}}.{{name}}

  slipway wit > wit/slipway.wit

  cd src && cargo build --target wasm32-wasip2 {{ if configuration == "release" { "--release" } else { "" } }}

  cp target/wasm32-wasip2/{{configuration}}/slipway_{{name}}.wasm artifacts/{{publisher}}.{{name}}/slipway_component.wasm
  cp slipway_component.json artifacts/{{publisher}}.{{name}}
  cp ../slipway_render/adaptive_cards_data/schema/host-config-with-defaults.schema.json artifacts/{{publisher}}.{{name}}/host-config-with-defaults.schema.json

  slipway package artifacts/{{publisher}}.{{name}}

test:
  cargo test
