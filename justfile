build:
  @cargo build

format:
  @cargo fmt --all
alias fmt := format

check-format:
  @cargo fmt --all --check

machete:
  @cargo machete

test:
  @cargo test

configuration-server:
  @cargo run -- --configuration serve

start metadata="example/metadata.json":
  @cargo run -- serve --configuration {{metadata}}

update-example-metadata port="9100":
  @curl localhost:{{ port }}/health 2> /dev/null \
    || (echo "Hmm... maybe 'just configuration-server'?" && exit 1)

  @curl localhost:{{ port }} 2> /dev/null \
    -H 'Content-Type: application/json' -X POST \
    -d '@example/metadata.json' | jq > example/metadata.json.tmp

  @mv example/metadata.json{.tmp,}

  @echo "Successfully updated 'example/metadata.json'."
alias update-example := update-example-metadata
