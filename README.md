# OData NDC

I had some time over Christmas and decided to learn Rust and the NDC Spec. See
the `example/` directory for some sample metadata relating to the OData
reference example, [TripPin](https://services.odata.org/V4/TripPinServiceRW).

## Getting started

* `just config-server` will run the configuration server.
* `just update` will use the running configuration server to update a given
  metadata file (or, by default, `example/metadata.json`).
* `just start` will run the NDC with the given metadata file (or, by default,
  `example/metadata.json`).

A Docker setup is also provided: `docker compose up` will run the NDC with the
example configuration, or whatever file `METADATA_PATH` points to.

## Roadmap

The connector currently implements the following features. This list was taken
from the [`ndc-spec` list](https://hasura.github.io/ndc-spec).

- [X] Service Health
- [ ] Metrics
- [ ] Telemetry
- [X] Capabilities
- [ ] Types
- [X] Schema
    - [X] Scalar Types
    - [X] Object Types
    - [X] Collections
    - [ ] Functions
    - [ ] Procedures
- [X] Queries
    - [X] Field Selection
    - [ ] Filtering
    - [X] Sorting
    - [X] Pagination
    - [ ] Aggregates
    - [ ] Arguments
    - [ ] Relationships
    - [ ] Variables
- [ ] Mutations
    - [ ] Procedures
- [ ] Explain
