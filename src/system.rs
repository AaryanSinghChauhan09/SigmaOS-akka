//! Actor system for akka.

use std::collections::HashMap;
use crate::actor::{Actor, ActorRef, ActorState};
use crate::message::{Message, MessageEnvelope};
use crate::router::{Router, RoutingStrategy};
use crate::supervisor::SupervisorStrategy;

/// System config
#[derive(Debug, Clone)]
pub struct SystemConfig {
    /// Max actors
    pub max_actors: usize,
    /// Max messages per actor
    pub max_messages_per_actor: usize,
    /// Default supervision strategy
    pub default_supervision_strategy: Option<SupervisorStrategy>,
    /// Enable remote actors
    pub enable_remote_actors: bool,
    /// Enable clustering
    pub enable_clustering: bool,
    /// Enable persistence
    pub enable_persistence: bool,
    /// Log level
    pub log_level: String,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            max_actors: 1000,
            max_messages_per_actor: 10000,
            default_supervision_strategy: None,
            enable_remote_actors: false,
            enable_clustering: false,
            enable_persistence: false,
            log_level: "info".into(),
        }
    }
}

/// System statistics
#[derive(Debug, Clone)]
pub struct SystemStatistics {
    /// Total actors
    pub total_actors: usize,
    /// Running actors
    pub running_actors: usize,
    /// Stopped actors
    pub stopped_actors: usize,
    /// Failed actors
    pub failed_actors: usize,
    /// Total messages
    pub total_messages: u64,
    /// Total routers
    pub total_routers: usize,
    /// System uptime in seconds
    pub uptime: u64,
}

/// Actor system
#[derive(Debug, Clone)]
pub struct ActorSystem {
    /// System name
    pub name: String,
    /// System config
    pub config: SystemConfig,
    /// Actors
    actors: HashMap<String, Actor>,
    /// Routers
    routers: HashMap<String, Router>,
    /// System created timestamp
    pub created_at: u64,
    /// System started timestamp
    pub started_at: Option<u64>,
    /// System statistics
    pub statistics: SystemStatistics,
}

impl ActorSystem {
    /// Create new actor system
    pub fn new(name: String) -> Self {
        Self {
            name,
            config: SystemConfig::default(),
            actors: HashMap::new(),
            routers: HashMap::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            started_at: None,
            statistics: SystemStatistics {
                total_actors: 0,
                running_actors: 0,
                stopped_actors: 0,
                failed_actors: 0,
                total_messages: 0,
                total_routers: 0,
                uptime: 0,
            },
        }
    }
    
    /// Create actor system with config
    pub fn with_config(name: String, config: SystemConfig) -> Self {
        Self {
            name,
            config,
            actors: HashMap::new(),
            routers: HashMap::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            started_at: None,
            statistics: SystemStatistics {
                total_actors: 0,
                running_actors: 0,
                stopped_actors: 0,
                failed_actors: 0,
                total_messages: 0,
                total_routers: 0,
                uptime: 0,
            },
        }
    }
    
    /// Start system
    pub fn start(&mut self) {
        self.started_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        self.update_statistics();
    }
    
    /// Stop system
    pub fn stop(&mut self) {
        // Stop all actors
        for actor in self.actors.values_mut() {
            actor.stop();
        }
        self.update_statistics();
    }
    
    /// Create actor
    pub fn create_actor(&mut self, id: String, name: String) -> String {
        let actor = Actor::new(id.clone(), name);
        self.actors.insert(id.clone(), actor);
        self.update_statistics();
        id
    }
    
    /// Get actor
    pub fn get_actor(&self, actor_id: &str) -> Option<&Actor> {
        self.actors.get(actor_id)
    }
    
    /// Get actor mutably
    pub fn get_actor_mut(&mut self, actor_id: &str) -> Option<&mut Actor> {
        self.actors.get_mut(actor_id)
    }
    
    /// Get all actors
    pub fn list_actors(&self) -> Vec<&Actor> {
        self.actors.values().collect()
    }
    
    /// Delete actor
    pub fn delete_actor(&mut self, actor_id: &str) -> bool {
        if self.actors.remove(actor_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Start actor
    pub fn start_actor(&mut self, actor_id: &str) -> bool {
        if let Some(actor) = self.actors.get_mut(actor_id) {
            if actor.start() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Stop actor
    pub fn stop_actor(&mut self, actor_id: &str) -> bool {
        if let Some(actor) = self.actors.get_mut(actor_id) {
            if actor.stop() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Send message to actor
    pub fn send_message(&mut self, actor_id: String, message: Message) -> bool {
        if let Some(actor) = self.actors.get_mut(&actor_id) {
            actor.receive_message(message);
            self.statistics.total_messages += 1;
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Send message envelope to actor
    pub fn send_message_envelope(&mut self, envelope: MessageEnvelope) -> bool {
        self.send_message(envelope.recipient, envelope.message)
    }
    
    /// Process actor messages
    pub fn process_actor_messages(&mut self, actor_id: &str) -> usize {
        let mut processed = 0;
        
        if let Some(actor) = self.actors.get_mut(actor_id) {
            while let Some(message) = actor.process_message() {
                // Process message (in real implementation would call actor's receive method)
                processed += 1;
            }
        }
        
        processed
    }
    
    /// Create router
    pub fn create_router(&mut self, id: String, routing_strategy: RoutingStrategy) -> String {
        let router = Router::new(id.clone(), routing_strategy);
        self.routers.insert(id.clone(), router);
        self.update_statistics();
        id
    }
    
    /// Get router
    pub fn get_router(&self, router_id: &str) -> Option<&Router> {
        self.routers.get(router_id)
    }
    
    /// Get router mutably
    pub fn get_router_mut(&mut self, router_id: &str) -> Option<&mut Router> {
        self.routers.get_mut(router_id)
    }
    
    /// Get all routers
    pub fn list_routers(&self) -> Vec<&Router> {
        self.routers.values().collect()
    }
    
    /// Delete router
    pub fn delete_router(&mut self, router_id: &str) -> bool {
        if self.routers.remove(router_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add actor to router
    pub fn add_actor_to_router(&mut self, router_id: &str, actor_id: String) -> bool {
        if let Some(router) = self.routers.get_mut(router_id) {
            if self.actors.contains_key(&actor_id) {
                router.add_route(actor_id);
                return true;
            }
        }
        false
    }
    
    /// Remove actor from router
    pub fn remove_actor_from_router(&mut self, router_id: &str, actor_id: &str) -> bool {
        if let Some(router) = self.routers.get_mut(router_id) {
            if router.remove_route(actor_id) {
                return true;
            }
        }
        false
    }
    
    /// Route message through router
    pub fn route_message(&mut self, router_id: &str, message: Message) -> bool {
        let actor_id = {
            if let Some(router) = self.routers.get_mut(router_id) {
                router.route_message(&message)
            } else {
                None
            }
        };
        
        if let Some(actor_id) = actor_id {
            self.send_message(actor_id, message)
        } else {
            false
        }
    }
    
    /// Get actors by state
    pub fn get_actors_by_state(&self, state: &ActorState) -> Vec<&Actor> {
        self.actors.values()
            .filter(|a| a.state == *state)
            .collect()
    }
    
    /// Get running actors
    pub fn get_running_actors(&self) -> Vec<&Actor> {
        self.get_actors_by_state(&ActorState::Running)
    }
    
    /// Get stopped actors
    pub fn get_stopped_actors(&self) -> Vec<&Actor> {
        self.get_actors_by_state(&ActorState::Stopped)
    }
    
    /// Get failed actors
    pub fn get_failed_actors(&self) -> Vec<&Actor> {
        self.get_actors_by_state(&ActorState::Failed)
    }
    
    /// Get actors with high message count
    pub fn get_actors_with_high_message_count(&self, threshold: u64) -> Vec<&Actor> {
        self.actors.values()
            .filter(|a| a.message_count > threshold)
            .collect()
    }
    
    /// Get actors with high error count
    pub fn get_actors_with_high_error_count(&self, threshold: u64) -> Vec<&Actor> {
        self.actors.values()
            .filter(|a| a.error_count > threshold)
            .collect()
    }
    
    /// Get system uptime
    pub fn get_uptime(&self) -> Option<u64> {
        if let Some(started) = self.started_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            Some(now - started)
        } else {
            None
        }
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_actors = self.actors.len();
        self.statistics.running_actors = self.actors.values()
            .filter(|a| a.state == ActorState::Running)
            .count();
        self.statistics.stopped_actors = self.actors.values()
            .filter(|a| a.state == ActorState::Stopped)
            .count();
        self.statistics.failed_actors = self.actors.values()
            .filter(|a| a.state == ActorState::Failed)
            .count();
        self.statistics.total_messages = self.actors.values()
            .map(|a| a.message_count)
            .sum();
        self.statistics.total_routers = self.routers.len();
        self.statistics.uptime = self.get_uptime().unwrap_or(0);
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &SystemStatistics {
        &self.statistics
    }
    
    /// Export system data
    pub fn export_data(&self) -> SystemExport {
        SystemExport {
            name: self.name.clone(),
            config: self.config.clone(),
            actors: self.actors.values().cloned().collect(),
            routers: self.routers.values().cloned().collect(),
        }
    }
    
    /// Import system data
    pub fn import_data(&mut self, export: SystemExport) {
        self.name = export.name;
        self.config = export.config;
        
        self.actors.clear();
        for actor in export.actors {
            self.actors.insert(actor.id.clone(), actor);
        }
        
        self.routers.clear();
        for router in export.routers {
            self.routers.insert(router.id.clone(), router);
        }
        
        self.update_statistics();
    }
}

/// System export data
#[derive(Debug, Clone)]
pub struct SystemExport {
    /// System name
    pub name: String,
    /// System config
    pub config: SystemConfig,
    /// Actors
    pub actors: Vec<Actor>,
    /// Routers
    pub routers: Vec<Router>,
}
