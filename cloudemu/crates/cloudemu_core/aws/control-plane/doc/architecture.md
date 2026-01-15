# Control Plane Architecture

The Control Plane (`src/services/`) acts as the fleet of servers that accepts your API requests. It is responsible for the business logic, orchestration, and interface layer of the emulator, mirroring the role of the Control Plane in AWS.

## Responsibilities

The Control Plane handles the following key functions:

1.  **Authentication**: Verifies identify and permissions.
    *   *Question:* "Is this request signed by a valid user?" (IAM)
2.  **Validation**: Ensures resources and requests meet constraints.
    *   *Question:* "Is this bucket name available?"
3.  **Routing**: directing requests to the appropriate backend.
    *   *Question:* "Which storage node holds this data?"

## Design Constraint

It **doesn't store the actual data**; it manages the metadata, validation, and orchestration of requests before delegating persistence to the Data Plane.
