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

update metadata="example/metadata.json" port="9100":
  @echo "Looking for the config server at localhost:{{ port }}..."
  @curl localhost:{{ port }}/health 2> /dev/null \
    || (echo "Hmm... maybe 'just configuration-server'?" && exit 1)

  @echo "Stripping current example metadata..."
  @cat {{metadata}} | jq '{ api_endpoint }' > {{metadata}}.tmp
  @mv {{metadata}}.tmp {{metadata}}

  @echo "Fetching updated schema..."
  @curl localhost:{{ port }} \
    -H 'Content-Type: application/json' -X POST \
    -d '@{{metadata}}' | jq > {{metadata}}.tmp
  @mv {{metadata}}.tmp {{metadata}}

  @echo "Successfully updated '{{metadata}}'."
