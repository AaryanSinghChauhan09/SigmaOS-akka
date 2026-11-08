//! Supervisor strategy for akka.

/// Restart strategy
#[derive(Debug, Clone, PartialEq)]
pub enum RestartStrategy {
    /// One for one - restart only failed child
    OneForOne,
    /// One for all - restart all children
    OneForAll,
    /// Rest for one - restart failed child and children started after it
    RestForOne,
}

/// Escalate strategy
#[derive(Debug, Clone, PartialEq)]
pub enum EscalateStrategy {
    /// Stop strategy
    Stop,
    /// Restart strategy
    Restart,
    /// Escalate strategy
    Escalate,
}

/// Supervisor strategy
#[derive(Debug, Clone)]
pub struct SupervisorStrategy {
    /// Restart strategy
    pub restart_strategy: RestartStrategy,
    /// Escalate strategy
    pub escalate_strategy: EscalateStrategy,
    /// Max restarts
    pub max_restarts: u32,
    /// Restart window in seconds
    pub restart_window: u64,
    /// Current restart count
    pub current_restarts: u32,
    /// Last restart timestamp
    pub last_restart: u64,
}

impl SupervisorStrategy {
    /// Create new supervisor strategy
    pub fn new(restart_strategy: RestartStrategy) -> Self {
        Self {
            restart_strategy,
            escalate_strategy: EscalateStrategy::Restart,
            max_restarts: 3,
            restart_window: 60,
            current_restarts: 0,
            last_restart: 0,
        }
    }
    
    /// Set escalate strategy
    pub fn set_escalate_strategy(&mut self, escalate_strategy: EscalateStrategy) {
        self.escalate_strategy = escalate_strategy;
    }
    
    /// Set max restarts
    pub fn set_max_restarts(&mut self, max_restarts: u32) {
        self.max_restarts = max_restarts;
    }
    
    /// Set restart window
    pub fn set_restart_window(&mut self, restart_window: u64) {
        self.restart_window = restart_window;
    }
    
    /// Reset restart count
    pub fn reset_restarts(&mut self) {
        self.current_restarts = 0;
        self.last_restart = 0;
    }
    
    /// Increment restart count
    pub fn increment_restarts(&mut self) {
        self.current_restarts += 1;
        self.last_restart = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if can restart
    pub fn can_restart(&self) -> bool {
        if self.current_restarts >= self.max_restarts {
            return false;
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check if restart window has passed
        if now - self.last_restart > self.restart_window {
            return true;
        }
        
        self.current_restarts < self.max_restarts
    }
    
    /// Get restart count
    pub fn get_restart_count(&self) -> u32 {
        self.current_restarts
    }
    
    /// Get time since last restart in seconds
    pub fn get_time_since_last_restart(&self) -> u64 {
        if self.last_restart == 0 {
            return 0;
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now - self.last_restart
    }
}
