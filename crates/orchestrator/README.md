# actrpc-orchestrator

`actrpc-orchestrator` is the runtime engine for ActRPC.

It runs the interception pipeline, invokes interceptors, executes requested actions, forwards JSON-RPC messages to their destination, and processes the response on the way back.

## Purpose

The orchestrator is the control point of the ActRPC pipeline.

It is responsible for:

- outbound interception
- action execution
- downstream forwarding
- inbound interception
- final response production

## What It Does

The orchestrator:

1. receives a JSON-RPC message
2. creates an `InterceptionRequest`
3. invokes outbound interceptors in order
4. collects requested actions
5. executes those actions
6. forwards the resulting message to the destination endpoint
7. receives the response
8. invokes inbound interceptors in order
9. executes any requested inbound actions
10. returns the final response

## Action Model

This crate defines the execution model for actions.

Actions are pluggable.
To be usable by the orchestrator, an action implementation must implement the orchestrator’s action execution trait.

The orchestrator does not define a built-in action catalog.

## Interceptor Model

Interceptors are resolved through the configured interceptor registry.

Each interceptor entry is responsible for defining how that interceptor is contacted. This may include transport-specific details.

The orchestrator does not own embedded interceptor implementations by default.

## Design

- Built on `actrpc-core`
- Defines the canonical orchestration runtime
- Defines the action execution contract
- Uses pluggable action executors
- Uses configured interceptor registry entries
- Keeps transport concerns outside the protocol model

## Usage (Conceptual)

```rust
let orchestrator = Orchestrator::builder()
    .with_interceptor_registry(interceptors)
    .with_action_registry(actions)
    .build();

let response = orchestrator.handle(message, destination);
```

## Scope

This crate provides:

- the orchestration engine
- the action execution trait
- the action registry model
- interceptor pipeline execution

It does not include:

- protocol definitions
- built-in actions
- built-in interceptors
- concrete transport implementations

## Summary

`actrpc-orchestrator` defines how the ActRPC pipeline runs.

It is the crate that coordinates interceptors, executes actions, and determines the final outcome.
