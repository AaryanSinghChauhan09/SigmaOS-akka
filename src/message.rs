//! Message definitions for akka.

/// Message priority
#[derive(Debug, Clone, PartialEq)]
pub enum MessagePriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// Message
#[derive(Debug, Clone)]
pub struct Message {
    /// Message payload
    pub payload: String,
    /// Message priority
    pub priority: MessagePriority,
    /// Message timestamp
    pub timestamp: u64,
    /// Message TTL in seconds
    pub ttl: Option<u64>,
    /// Message headers
    pub headers: std::collections::HashMap<String, String>,
    /// Message correlation ID
    pub correlation_id: Option<String>,
    /// Message causation ID
    pub causation_id: Option<String>,
}

impl Message {
    /// Create new message
    pub fn new(payload: String) -> Self {
        Self {
            payload,
            priority: MessagePriority::Normal,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl: None,
            headers: std::collections::HashMap::new(),
            correlation_id: None,
            causation_id: None,
        }
    }
    
    /// Create message with priority
    pub fn with_priority(payload: String, priority: MessagePriority) -> Self {
        let mut message = Self::new(payload);
        message.priority = priority;
        message
    }
    
    /// Set TTL
    pub fn set_ttl(&mut self, ttl: u64) {
        self.ttl = Some(ttl);
    }
    
    /// Add header
    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
    
    /// Get header
    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
    
    /// Set correlation ID
    pub fn set_correlation_id(&mut self, correlation_id: String) {
        self.correlation_id = Some(correlation_id);
    }
    
    /// Set causation ID
    pub fn set_causation_id(&mut self, causation_id: String) {
        self.causation_id = Some(causation_id);
    }
    
    /// Check if message is expired
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now - self.timestamp > ttl
        } else {
            false
        }
    }
    
    /// Get message age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.timestamp
    }
    
    /// Check if message is old
    pub fn is_old(&self, max_age: u64) -> bool {
        self.get_age_seconds() > max_age
    }
}

/// Message envelope
#[derive(Debug, Clone)]
pub struct MessageEnvelope {
    /// Sender actor ID
    pub sender: String,
    /// Recipient actor ID
    pub recipient: String,
    /// Message
    pub message: Message,
    /// Envelope timestamp
    pub timestamp: u64,
    /// Envelope ID
    pub envelope_id: String,
}

impl MessageEnvelope {
    /// Create new message envelope
    pub fn new(sender: String, recipient: String, message: Message) -> Self {
        Self {
            sender,
            recipient,
            message,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            envelope_id: format!("env-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
        }
    }
    
    /// Get envelope age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.timestamp
    }
    
    /// Check if envelope is expired
    pub fn is_expired(&self) -> bool {
        self.message.is_expired()
    }
}
