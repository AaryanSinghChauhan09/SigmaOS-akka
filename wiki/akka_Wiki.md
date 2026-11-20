# SigmaOS-akka Component Wiki

## Overview
SigmaOS-akka provides Akka actor model and distributed computing support for the SigmaOS operating system. It demonstrates the actor model subsystem and distributed computing within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic actor initialization.

## Principles
- Single Responsibility
- Actor Abstraction
- Fail-Fast on Actor Initialization Errors
- Deterministic Actor Setup
- Clear Abstraction Over Actor APIs
- Composition over Inheritance for actor extensions

## Architecture
SigmaOS-akka follows an actor pattern:
- **Actor Core**: Provides actor system creation, message passing, and actor lifecycle management
- **Distribution**: Supports remote actor communication, cluster membership, and distributed deployment
- **Message Routing**: Supports message addressing, routing, and mailbox management
- **Error Handling**: Fail-fast on invalid actor configurations or unsupported actor types

## API
The component provides the following public APIs:
- `actor::system::create(system_name)` - Create a new actor system with given name
- `actor::actor::spawn(system, behavior)` - Spawn a new actor in the given system
- `actor::message::send(recipient, message)` - Send a message to an actor
- `actor::cluster::join(node)` - Join an actor cluster node
- Error types for actor failure scenarios or unsupported actor types

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific actor configuration
2. **Act**: Execute the code under test (actor spawning, message sending)
3. **Assert**: Verify the expected actor results

Example:
```rust
#[test]
fn test_actor_spawn_aaa() {
    // Arrange
    let system = actor::system::create("test-system");
    
    // Act
    let actor = actor::actor::spawn(system, |ctx| {});
    
    // Assert
    assert!(actor.is_ok());
    let spawned = actor.unwrap();
    assert!(spawned.is_alive());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Actor initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all actor failure scenarios
- Unit tests provided for actor system creation and message sending
- Integration tests for full actor system with actual distributed deployment

## TODO
- Add support for additional actor features (remote deployment, cluster sharding, persistence)
- Expand message routing capabilities ( multicast, broadcast, router actors)
- Add cluster management features (cluster singleton, downing, membership)
- Create integration tests with actual distributed actor systems
- Implement actor persistence and state management