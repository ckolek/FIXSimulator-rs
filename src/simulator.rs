
use fix::messages::FIXMessage;
use fix::engine::MessageListener;
use fix::session::SessionId;

pub struct Simulator {
    id : String
}

impl Simulator {
    pub fn new(id : &str) -> Self {
        return Simulator {
            id: String::from(id)
        };
    }
}

impl MessageListener for Simulator {
    fn get_id(&self) -> &str {
        return &self.id;
    }
    
    fn on_receive(&mut self, session_id : &SessionId, message : &FIXMessage) {
        
    }
    
    fn on_send(&mut self, session_id : &SessionId, message : &FIXMessage) {
        
    }
}