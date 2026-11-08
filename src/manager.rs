//! Actor manager for akka.

use std::collections::HashMap;
use crate::system::{ActorSystem, SystemConfig};
use crate::actor::{Actor, ActorState};
use crate::router::{Router, RoutingStrategy};
use crate::message::Message;

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max systems
    pub max_systems: usize,
    /// Default system config
    pub default_config: SystemConfig,
    /// Auto-start systems
    pub auto_start_systems: bool,
    /// Enable system monitoring
    pub enable_monitoring: bool,
    /// Monitoring interval in seconds
    pub monitoring_interval: u64,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_systems: 10,
            default_config: SystemConfig::default(),
            auto_start_systems: true,
            enable_monitoring: true,
            monitoring_interval: 60,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total systems
    pub total_systems: usize,
    /// Total actors
    pub total_actors: usize,
    /// Total routers
    pub total_routers: usize,
    /// Running systems
    pub running_systems: usize,
    /// Total messages
    pub total_messages: u64,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Actor manager
#[derive(Debug, Clone)]
pub struct ActorManager {
    /// Systems
    systems: HashMap<String, ActorSystem>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for ActorManager {
    fn default() -> Self {
        Self {
            systems: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_systems: 0,
                total_actors: 0,
                total_routers: 0,
                running_systems: 0,
                total_messages: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl ActorManager {
    /// Create new actor manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            systems: HashMap::new(),
            options,
            statistics: ManagerStatistics {
                total_systems: 0,
                total_actors: 0,
                total_routers: 0,
                running_systems: 0,
                total_messages: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        self.update_statistics();
    }
    
    /// Create system
    pub fn create_system(&mut self, name: String) -> String {
        let id = format!("system-{}", self.systems.len() + 1);
        let mut system = ActorSystem::with_config(name, self.options.default_config.clone());
        
        if self.options.auto_start_systems {
            system.start();
        }
        
        self.systems.insert(id.clone(), system);
        self.update_statistics();
        id
    }
    
    /// Create system with config
    pub fn create_system_with_config(&mut self, name: String, config: SystemConfig) -> String {
        let id = format!("system-{}", self.systems.len() + 1);
        let mut system = ActorSystem::with_config(name, config);
        
        if self.options.auto_start_systems {
            system.start();
        }
        
        self.systems.insert(id.clone(), system);
        self.update_statistics();
        id
    }
    
    /// Get system
    pub fn get_system(&self, system_id: &str) -> Option<&ActorSystem> {
        self.systems.get(system_id)
    }
    
    /// Get system mutably
    pub fn get_system_mut(&mut self, system_id: &str) -> Option<&mut ActorSystem> {
        self.systems.get_mut(system_id)
    }
    
    /// Get all systems
    pub fn list_systems(&self) -> Vec<&ActorSystem> {
        self.systems.values().collect()
    }
    
    /// Delete system
    pub fn delete_system(&mut self, system_id: &str) -> bool {
        if self.systems.remove(system_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Start system
    pub fn start_system(&mut self, system_id: &str) -> bool {
        if let Some(system) = self.systems.get_mut(system_id) {
            system.start();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Stop system
    pub fn stop_system(&mut self, system_id: &str) -> bool {
        if let Some(system) = self.systems.get_mut(system_id) {
            system.stop();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Create actor in system
    pub fn create_actor_in_system(&mut self, system_id: &str, actor_id: String, actor_name: String) -> Option<String> {
        if let Some(system) = self.systems.get_mut(system_id) {
            let id = system.create_actor(actor_id, actor_name);
            self.update_statistics();
            Some(id)
        } else {
            None
        }
    }
    
    /// Send message to actor in system
    pub fn send_message_to_actor(&mut self, system_id: &str, actor_id: String, message: Message) -> bool {
        if let Some(system) = self.systems.get_mut(system_id) {
            if system.send_message(actor_id, message) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Create router in system
    pub fn create_router_in_system(&mut self, system_id: &str, router_id: String, routing_strategy: RoutingStrategy) -> Option<String> {
        if let Some(system) = self.systems.get_mut(system_id) {
            let id = system.create_router(router_id, routing_strategy);
            self.update_statistics();
            Some(id)
        } else {
            None
        }
    }
    
    /// Get systems by name
    pub fn get_systems_by_name(&self, name: &str) -> Vec<&ActorSystem> {
        self.systems.values()
            .filter(|s| s.name == name)
            .collect()
    }
    
    /// Get systems with actors
    pub fn get_systems_with_actors(&self) -> Vec<&ActorSystem> {
        self.systems.values()
            .filter(|s| !s.list_actors().is_empty())
            .collect()
    }
    
    /// Get systems with routers
    pub fn get_systems_with_routers(&self) -> Vec<&ActorSystem> {
        self.systems.values()
            .filter(|s| !s.list_routers().is_empty())
            .collect()
    }
    
    /// Get total actors across all systems
    pub fn get_total_actors(&self) -> usize {
        self.systems.values()
            .map(|s| s.statistics.total_actors)
            .sum()
    }
    
    /// Get total messages across all systems
    pub fn get_total_messages(&self) -> u64 {
        self.systems.values()
            .map(|s| s.statistics.total_messages)
            .sum()
    }
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_systems = self.systems.len();
        self.statistics.total_actors = self.get_total_actors();
        self.statistics.total_routers = self.systems.values()
            .map(|s| s.statistics.total_routers)
            .sum();
        self.statistics.running_systems = self.systems.values()
            .filter(|s| s.started_at.is_some())
            .count();
        self.statistics.total_messages = self.get_total_messages();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            systems: self.systems.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.systems.clear();
        for system in export.systems {
            self.systems.insert(format!("system-{}", self.systems.len() + 1), system);
        }
        self.update_statistics();
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Systems
    pub systems: Vec<ActorSystem>,
}
