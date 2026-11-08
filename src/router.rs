//! Router for akka.

use crate::message::Message;

/// Routing strategy
#[derive(Debug, Clone, PartialEq)]
pub enum RoutingStrategy {
    /// Round robin
    RoundRobin,
    /// Random
    Random,
    /// Consistent hash
    ConsistentHash,
    /// Least busy
    LeastBusy,
    /// Broadcast
    Broadcast,
    /// Scatter gather
    ScatterGather,
}

/// Router config
#[derive(Debug, Clone)]
pub struct RouterConfig {
    /// Routing strategy
    pub routing_strategy: RoutingStrategy,
    /// Max routes
    pub max_routes: usize,
    /// Route timeout in seconds
    pub route_timeout: u64,
    /// Enable backpressure
    pub enable_backpressure: bool,
    /// Enable load balancing
    pub enable_load_balancing: bool,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            routing_strategy: RoutingStrategy::RoundRobin,
            max_routes: 100,
            route_timeout: 30,
            enable_backpressure: false,
            enable_load_balancing: true,
        }
    }
}

/// Router
#[derive(Debug, Clone)]
pub struct Router {
    /// Router ID
    pub id: String,
    /// Router name
    pub name: String,
    /// Routing strategy
    pub routing_strategy: RoutingStrategy,
    /// Router config
    pub config: RouterConfig,
    /// Routes (actor IDs)
    pub routes: Vec<String>,
    /// Current route index
    pub current_index: usize,
    /// Route statistics
    pub statistics: RouterStatistics,
    /// Router created timestamp
    pub created_at: u64,
}

/// Router statistics
#[derive(Debug, Clone, Default)]
pub struct RouterStatistics {
    /// Total messages routed
    pub total_messages: u64,
    /// Successful routes
    pub successful_routes: u64,
    /// Failed routes
    pub failed_routes: u64,
    /// Average route time in milliseconds
    pub average_route_time: u64,
}

impl Router {
    /// Create new router
    pub fn new(id: String, routing_strategy: RoutingStrategy) -> Self {
        Self {
            id: id.clone(),
            name: id,
            routing_strategy,
            config: RouterConfig::default(),
            routes: vec![],
            current_index: 0,
            statistics: RouterStatistics::default(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    /// Set config
    pub fn set_config(&mut self, config: RouterConfig) {
        self.config = config;
    }
    
    /// Add route
    pub fn add_route(&mut self, actor_id: String) -> bool {
        if !self.routes.contains(&actor_id) && self.routes.len() < self.config.max_routes {
            self.routes.push(actor_id);
            return true;
        }
        false
    }
    
    /// Remove route
    pub fn remove_route(&mut self, actor_id: &str) -> bool {
        if let Some(pos) = self.routes.iter().position(|r| r == actor_id) {
            self.routes.remove(pos);
            // Reset index if needed
            if self.current_index >= self.routes.len() {
                self.current_index = 0;
            }
            return true;
        }
        false
    }
    
    /// Get routes
    pub fn get_routes(&self) -> &[String] {
        &self.routes
    }
    
    /// Route message
    pub fn route_message(&mut self, message: &Message) -> Option<String> {
        if self.routes.is_empty() {
            return None;
        }
        
        let selected_route = match self.routing_strategy {
            RoutingStrategy::RoundRobin => self.route_round_robin(),
            RoutingStrategy::Random => self.route_random(),
            RoutingStrategy::ConsistentHash => self.route_consistent_hash(message),
            RoutingStrategy::LeastBusy => self.route_least_busy(),
            RoutingStrategy::Broadcast => self.route_broadcast(),
            RoutingStrategy::ScatterGather => self.route_scatter_gather(),
        };
        
        if selected_route.is_some() {
            self.statistics.successful_routes += 1;
        } else {
            self.statistics.failed_routes += 1;
        }
        
        self.statistics.total_messages += 1;
        selected_route
    }
    
    /// Route round robin
    fn route_round_robin(&mut self) -> Option<String> {
        if self.routes.is_empty() {
            return None;
        }
        
        let route = self.routes[self.current_index].clone();
        self.current_index = (self.current_index + 1) % self.routes.len();
        Some(route)
    }
    
    /// Route random
    fn route_random(&self) -> Option<String> {
        if self.routes.is_empty() {
            return None;
        }
        
        // Simple random selection (in real implementation would use proper RNG)
        let index = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize) % self.routes.len();
        Some(self.routes[index].clone())
    }
    
    /// Route consistent hash
    fn route_consistent_hash(&self, message: &Message) -> Option<String> {
        if self.routes.is_empty() {
            return None;
        }
        
        // Simple hash-based routing
        let hash = message.payload.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
        let index = (hash as usize) % self.routes.len();
        Some(self.routes[index].clone())
    }
    
    /// Route least busy
    fn route_least_busy(&self) -> Option<String> {
        // In real implementation would check actor mailbox sizes
        // For now, just return first route
        self.routes.first().cloned()
    }
    
    /// Route broadcast
    fn route_broadcast(&self) -> Option<String> {
        // In real implementation would send to all routes
        // For now, just return first route
        self.routes.first().cloned()
    }
    
    /// Route scatter gather
    fn route_scatter_gather(&self) -> Option<String> {
        // In real implementation would send to multiple routes
        // For now, just return first route
        self.routes.first().cloned()
    }
    
    /// Get router age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get route count
    pub fn get_route_count(&self) -> usize {
        self.routes.len()
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &RouterStatistics {
        &self.statistics
    }
    
    /// Reset statistics
    pub fn reset_statistics(&mut self) {
        self.statistics = RouterStatistics::default();
    }
    
    /// Get success rate
    pub fn get_success_rate(&self) -> f64 {
        let total = self.statistics.total_messages;
        if total == 0 {
            return 0.0;
        }
        (self.statistics.successful_routes as f64 / total as f64) * 100.0
    }
}
