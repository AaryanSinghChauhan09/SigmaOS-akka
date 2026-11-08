//! SigmaOS-akka — comprehensive actor framework component.
//!
//! Provides actor-based concurrency model with message passing,
//! supervision strategies, and actor lifecycle management.

pub mod actor;
pub mod system;
pub mod message;
pub mod supervisor;
pub mod router;
pub mod manager;
pub mod options;

pub use actor::{Actor, ActorRef, ActorContext, ActorState};
pub use system::{ActorSystem, SystemConfig, SystemStatistics};
pub use message::{Message, MessageEnvelope, MessagePriority};
pub use supervisor::{SupervisorStrategy, RestartStrategy, EscalateStrategy};
pub use router::{Router, RouterConfig, RoutingStrategy};
pub use manager::{ActorManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_actor_state() {
        let states = vec![
            ActorState::Created,
            ActorState::Starting,
            ActorState::Running,
            ActorState::Stopping,
            ActorState::Stopped,
            ActorState::Failed,
        ];
        assert_eq!(states.len(), 6);
    }
    
    #[test]
    fn test_message_priority() {
        let priorities = vec![
            MessagePriority::Low,
            MessagePriority::Normal,
            MessagePriority::High,
            MessagePriority::Critical,
        ];
        assert_eq!(priorities.len(), 4);
    }
    
    #[test]
    fn test_restart_strategy() {
        let strategies = vec![
            RestartStrategy::OneForOne,
            RestartStrategy::OneForAll,
            RestartStrategy::RestForOne,
        ];
        assert_eq!(strategies.len(), 3);
    }
    
    #[test]
    fn test_routing_strategy() {
        let strategies = vec![
            RoutingStrategy::RoundRobin,
            RoutingStrategy::Random,
            RoutingStrategy::ConsistentHash,
            RoutingStrategy::LeastBusy,
            RoutingStrategy::Broadcast,
            RoutingStrategy::ScatterGather,
        ];
        assert_eq!(strategies.len(), 6);
    }
    
    #[test]
    fn test_message_creation() {
        let message = Message::new("Hello, world!".into());
        assert_eq!(message.payload, "Hello, world!");
        assert_eq!(message.priority, MessagePriority::Normal);
    }
    
    #[test]
    fn test_message_envelope_creation() {
        let envelope = MessageEnvelope::new(
            "sender-1".into(),
            "recipient-1".into(),
            Message::new("Hello!".into()),
        );
        
        assert_eq!(envelope.sender, "sender-1");
        assert_eq!(envelope.recipient, "recipient-1");
        assert_eq!(envelope.message.payload, "Hello!");
    }
    
    #[test]
    fn test_actor_creation() {
        let actor = Actor::new("actor-1".into(), "TestActor".into());
        assert_eq!(actor.id, "actor-1");
        assert_eq!(actor.name, "TestActor");
        assert_eq!(actor.state, ActorState::Created);
    }
    
    #[test]
    fn test_actor_ref_creation() {
        let actor_ref = ActorRef::new("actor-1".into());
        assert_eq!(actor_ref.id, "actor-1");
    }
    
    #[test]
    fn test_actor_context_creation() {
        let context = ActorContext::new("actor-1".into());
        assert_eq!(context.actor_id, "actor-1");
    }
    
    #[test]
    fn test_supervisor_strategy_creation() {
        let strategy = SupervisorStrategy::new(RestartStrategy::OneForOne);
        assert_eq!(strategy.restart_strategy, RestartStrategy::OneForOne);
    }
    
    #[test]
    fn test_router_creation() {
        let router = Router::new("router-1".into(), RoutingStrategy::RoundRobin);
        assert_eq!(router.id, "router-1");
        assert_eq!(router.routing_strategy, RoutingStrategy::RoundRobin);
    }
    
    #[test]
    fn test_system_config_creation() {
        let config = SystemConfig::default();
        assert_eq!(config.max_actors, 1000);
        assert_eq!(config.max_messages_per_actor, 10000);
    }
    
    #[test]
    fn test_actor_system_creation() {
        let mut system = ActorSystem::new("test-system".into());
        assert_eq!(system.name, "test-system");
        assert!(system.list_actors().is_empty());
    }
    
    #[test]
    fn test_create_actor() {
        let mut system = ActorSystem::new("test-system".into());
        let actor_id = system.create_actor("actor-1".into(), "TestActor".into());
        
        assert!(system.get_actor(&actor_id).is_some());
        assert_eq!(system.list_actors().len(), 1);
    }
    
    #[test]
    fn test_send_message() {
        let mut system = ActorSystem::new("test-system".into());
        let actor_id = system.create_actor("actor-1".into(), "TestActor".into());
        
        let message = Message::new("Hello!".into());
        system.send_message(actor_id.clone(), message);
        
        let actor = system.get_actor(&actor_id).unwrap();
        assert_eq!(actor.mailbox.len(), 1);
    }
    
    #[test]
    fn test_actor_lifecycle() {
        let mut system = ActorSystem::new("test-system".into());
        let actor_id = system.create_actor("actor-1".into(), "TestActor".into());
        
        // Start actor
        system.start_actor(&actor_id);
        let actor = system.get_actor(&actor_id).unwrap();
        assert_eq!(actor.state, ActorState::Running);
        
        // Stop actor
        system.stop_actor(&actor_id);
        let actor = system.get_actor(&actor_id).unwrap();
        assert_eq!(actor.state, ActorState::Stopped);
    }
    
    #[test]
    fn test_router_creation_in_system() {
        let mut system = ActorSystem::new("test-system".into());
        let router_id = system.create_router("router-1".into(), RoutingStrategy::RoundRobin);
        
        assert!(system.get_router(&router_id).is_some());
        assert_eq!(system.list_routers().len(), 1);
    }
    
    #[test]
    fn test_export_import() {
        let mut system = ActorSystem::new("test-system".into());
        let actor_id = system.create_actor("actor-1".into(), "TestActor".into());
        system.send_message(actor_id, Message::new("Hello!".into()));
        
        let export = system.export_data();
        
        let mut system2 = ActorSystem::new("test-system2".into());
        system2.import_data(export);
        
        assert_eq!(system2.list_actors().len(), 1);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = ActorManager::new();
        assert!(manager.list_systems().is_empty());
    }
    
    #[test]
    fn test_create_system() {
        let mut manager = ActorManager::new();
        let system_id = manager.create_system("test-system".into());
        
        assert!(manager.get_system(&system_id).is_some());
        assert_eq!(manager.list_systems().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = ActorManager::new();
        
        manager.create_system("system1".into());
        manager.create_system("system2".into());
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_systems, 2);
        assert_eq!(stats.total_actors, 0);
    }
}
