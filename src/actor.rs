//! Actor definitions for akka.

use crate::message::{Message, MessageEnvelope};
use crate::supervisor::SupervisorStrategy;

/// Actor state
#[derive(Debug, Clone, PartialEq)]
pub enum ActorState {
    /// Created
    Created,
    /// Starting
    Starting,
    /// Running
    Running,
    /// Stopping
    Stopping,
    /// Stopped
    Stopped,
    /// Failed
    Failed,
}

/// Actor reference
#[derive(Debug, Clone)]
pub struct ActorRef {
    /// Actor ID
    pub id: String,
    /// Actor path
    pub path: String,
}

impl ActorRef {
    /// Create new actor reference
    pub fn new(id: String) -> Self {
        Self {
            id: id.clone(),
            path: format!("/user/{}", id),
        }
    }
    
    /// Send message to actor
    pub fn send_message(&self, message: Message) -> MessageEnvelope {
        MessageEnvelope::new("system".into(), self.id.clone(), message)
    }
    
    /// Stop actor
    pub fn stop(&self) -> MessageEnvelope {
        MessageEnvelope::new("system".into(), self.id.clone(), Message::new("stop".into()))
    }
}

/// Actor context
#[derive(Debug, Clone)]
pub struct ActorContext {
    /// Actor ID
    pub actor_id: String,
    /// Actor path
    pub actor_path: String,
    /// Parent actor ID
    pub parent_id: Option<String>,
    /// Child actor IDs
    pub children: Vec<String>,
    /// Actor state
    pub state: ActorState,
    /// Actor mailbox size
    pub mailbox_size: usize,
}

impl ActorContext {
    /// Create new actor context
    pub fn new(actor_id: String) -> Self {
        Self {
            actor_id: actor_id.clone(),
            actor_path: format!("/user/{}", actor_id),
            parent_id: None,
            children: vec![],
            state: ActorState::Created,
            mailbox_size: 0,
        }
    }
    
    /// Set parent
    pub fn set_parent(&mut self, parent_id: String) {
        self.parent_id = Some(parent_id);
    }
    
    /// Add child
    pub fn add_child(&mut self, child_id: String) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }
    
    /// Remove child
    pub fn remove_child(&mut self, child_id: &str) -> bool {
        if let Some(pos) = self.children.iter().position(|c| c == child_id) {
            self.children.remove(pos);
            return true;
        }
        false
    }
    
    /// Update state
    pub fn update_state(&mut self, state: ActorState) {
        self.state = state;
    }
    
    /// Increment mailbox size
    pub fn increment_mailbox(&mut self) {
        self.mailbox_size += 1;
    }
    
    /// Decrement mailbox size
    pub fn decrement_mailbox(&mut self) {
        if self.mailbox_size > 0 {
            self.mailbox_size -= 1;
        }
    }
}

/// Actor
#[derive(Debug, Clone)]
pub struct Actor {
    /// Actor ID
    pub id: String,
    /// Actor name
    pub name: String,
    /// Actor type
    pub actor_type: String,
    /// Actor state
    pub state: ActorState,
    /// Actor context
    pub context: ActorContext,
    /// Actor supervisor strategy
    pub supervisor_strategy: Option<SupervisorStrategy>,
    /// Actor mailbox
    pub mailbox: Vec<Message>,
    /// Actor created timestamp
    pub created_at: u64,
    /// Actor started timestamp
    pub started_at: Option<u64>,
    /// Actor stopped timestamp
    pub stopped_at: Option<u64>,
    /// Actor message count
    pub message_count: u64,
    /// Actor error count
    pub error_count: u64,
    /// Actor custom data
    pub custom_data: std::collections::HashMap<String, String>,
}

impl Actor {
    /// Create new actor
    pub fn new(id: String, name: String) -> Self {
        Self {
            id: id.clone(),
            name,
            actor_type: "generic".into(),
            state: ActorState::Created,
            context: ActorContext::new(id),
            supervisor_strategy: None,
            mailbox: vec![],
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            started_at: None,
            stopped_at: None,
            message_count: 0,
            error_count: 0,
            custom_data: std::collections::HashMap::new(),
        }
    }
    
    /// Set actor type
    pub fn set_actor_type(&mut self, actor_type: String) {
        self.actor_type = actor_type;
    }
    
    /// Set supervisor strategy
    pub fn set_supervisor_strategy(&mut self, strategy: SupervisorStrategy) {
        self.supervisor_strategy = Some(strategy);
    }
    
    /// Start actor
    pub fn start(&mut self) -> bool {
        if self.state == ActorState::Created {
            self.state = ActorState::Starting;
            self.context.update_state(ActorState::Starting);
            
            // Simulate start
            self.state = ActorState::Running;
            self.context.update_state(ActorState::Running);
            self.started_at = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
            return true;
        }
        false
    }
    
    /// Stop actor
    pub fn stop(&mut self) -> bool {
        if self.state == ActorState::Running {
            self.state = ActorState::Stopping;
            self.context.update_state(ActorState::Stopping);
            
            // Simulate stop
            self.state = ActorState::Stopped;
            self.context.update_state(ActorState::Stopped);
            self.stopped_at = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
            return true;
        }
        false
    }
    
    /// Fail actor
    pub fn fail(&mut self) {
        self.state = ActorState::Failed;
        self.context.update_state(ActorState::Failed);
        self.error_count += 1;
    }
    
    /// Receive message
    pub fn receive_message(&mut self, message: Message) {
        self.mailbox.push(message);
        self.message_count += 1;
        self.context.increment_mailbox();
    }
    
    /// Process message
    pub fn process_message(&mut self) -> Option<Message> {
        if !self.mailbox.is_empty() {
            let message = self.mailbox.remove(0);
            self.context.decrement_mailbox();
            Some(message)
        } else {
            None
        }
    }
    
    /// Get next message
    pub fn get_next_message(&self) -> Option<&Message> {
        self.mailbox.first()
    }
    
    /// Clear mailbox
    pub fn clear_mailbox(&mut self) {
        self.mailbox.clear();
        self.context.mailbox_size = 0;
    }
    
    /// Set custom data
    pub fn set_custom_data(&mut self, key: String, value: String) {
        self.custom_data.insert(key, value);
    }
    
    /// Get custom data
    pub fn get_custom_data(&self, key: &str) -> Option<&String> {
        self.custom_data.get(key)
    }
    
    /// Check if actor is running
    pub fn is_running(&self) -> bool {
        self.state == ActorState::Running
    }
    
    /// Check if actor is stopped
    pub fn is_stopped(&self) -> bool {
        self.state == ActorState::Stopped
    }
    
    /// Check if actor has failed
    pub fn has_failed(&self) -> bool {
        self.state == ActorState::Failed
    }
    
    /// Get actor age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get uptime in seconds
    pub fn get_uptime_seconds(&self) -> Option<u64> {
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
    
    /// Get mailbox size
    pub fn get_mailbox_size(&self) -> usize {
        self.mailbox.len()
    }
    
    /// Get message count
    pub fn get_message_count(&self) -> u64 {
        self.message_count
    }
    
    /// Get error count
    pub fn get_error_count(&self) -> u64 {
        self.error_count
    }
    
    /// Get child count
    pub fn get_child_count(&self) -> usize {
        self.context.children.len()
    }
    
    /// Get actor info
    pub fn get_info(&self) -> ActorInfo {
        ActorInfo {
            id: self.id.clone(),
            name: self.name.clone(),
            actor_type: self.actor_type.clone(),
            state: self.state.clone(),
            mailbox_size: self.mailbox.len(),
            message_count: self.message_count,
            error_count: self.error_count,
            child_count: self.context.children.len(),
            uptime: self.get_uptime_seconds(),
        }
    }
}

/// Actor info
#[derive(Debug, Clone)]
pub struct ActorInfo {
    /// Actor ID
    pub id: String,
    /// Actor name
    pub name: String,
    /// Actor type
    pub actor_type: String,
    /// Actor state
    pub state: ActorState,
    /// Mailbox size
    pub mailbox_size: usize,
    /// Message count
    pub message_count: u64,
    /// Error count
    pub error_count: u64,
    /// Child count
    pub child_count: usize,
    /// Uptime in seconds
    pub uptime: Option<u64>,
}
