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

run-configuration-server:
  @cargo run -- configuration serve
alias config := run-configuration-server
alias config-server := run-configuration-server
alias configuration-server := run-configuration-server

start metadata="example/metadata.json":
  @cargo run -- serve --configuration {{metadata}}

update-example-metadata port="9100":
  @echo "Looking for the config server at localhost:{{ port }}..."
  @curl localhost:{{ port }}/health 2> /dev/null \
    || (echo "Hmm... maybe 'just configuration-server'?" && exit 1)

  @echo "Stripping current example metadata..."
  @cat example/metadata.json | jq '{ api_endpoint }' > example/metadata.json.tmp
  @mv example/metadata.json.tmp example/metadata.json

  @curl localhost:{{ port }} 2> /dev/null \
    -H 'Content-Type: application/json' -X POST \
    -d '@example/metadata.json' | jq > example/metadata.json.tmp
  @mv example/metadata.json.tmp example/metadata.json

  @echo "Successfully updated 'example/metadata.json'."
alias update-example := update-example-metadata
