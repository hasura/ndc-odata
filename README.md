# OData NDC

A Native Data Connector (NDC) for OData APIs, specifically for those adhering
to [OData v4.01](https://docs.oasis-open.org/odata/odata/v4.01/).

See the `example/` directory for sample metadata corresponding to the OData
reference service: [TripPin](https://services.odata.org/TripPinRESTierService).

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
- [X] Schema
    - [X] Scalar Types
    - [X] Object Types
    - [X] Collections
    - [ ] Functions
    - [ ] Procedures
- [X] Queries
    - [X] Field Selection
    - [ ] Filtering
      - [X] Equality with scalars
      - [X] Equality between local columns
      - [ ] Equality with root columns
      - [X] Nullability checks
      - [ ] Existential predicates
    - [X] Sorting
    - [X] Pagination
    - [ ] Aggregates
    - [ ] Arguments
    - [X] Relationships
    - [ ] Variables
- [ ] Mutations
    - [ ] Procedures
- [X] Explain
