use log::{debug, error, info};
use std::time::SystemTime;

use crate::protocol::Protocol;

//#[derive(Default)]
pub struct Client {
    lastdata: SystemTime,
    protocol: Protocol,
    msg_id: String,
    pub message_queue: String,
}

impl<'a> Client {
    pub fn new() -> Self {
        Self {
            lastdata: SystemTime::now(),
            protocol: Default::default(),
            msg_id: Default::default(),
            message_queue: Default::default(),
        }
    }

    pub fn is_logged(&self) -> bool {
        return true; // TODO implement
    }

    pub fn Handle(&mut self, msg: &str) {
        // TODO here implement flood limit

        self.lastdata = SystemTime::now();
        let msg = self.set_msg_id(msg);

        self.HandleProtocolCommand(msg);
    }

    pub fn HandleProtocolCommand(&mut self, msg: &str) {
        let cmd = msg.trim_start_matches(' ').trim_end_matches('\r');
        let executor = self.protocol._handle(cmd);
        match executor {
            Ok(v) => v.execute(self),
            Err(err) => error!("{}", err),
        }
    }

    // appends to buffer which will be later handled by server
    pub fn Send(&mut self, msg: &str) {
        debug!("Client sending message {}", msg);
        if !self.msg_id.is_empty() {
            self.message_queue.push_str(&self.msg_id);
            self.msg_id.clear();
        }

        self.message_queue.push_str(msg);
        self.message_queue.push_str(msg);
        self.message_queue.push('\n');
    }

    pub fn set_msg_id(&mut self, msg: &'a str) -> &'a str {
        if !self.msg_id.is_empty() {
            debug!("self.msg_id is not empty!: {}", self.msg_id); // TODO how does it work? it
                                                                  // should be cleared in
                                                                  // self.Send()
        }
        self.msg_id.clear();

        if !msg.starts_with('#') {
            return msg;
        }
        let vec = msg.split(' ').collect::<Vec<&str>>();
        if vec.len() < 1 {
            return msg;
        }

        let num = vec[0][1..].parse::<usize>();

        match num {
            Ok(val) => {
                self.msg_id = format!("#{} ", val);
                let pos = msg.find(' ').unwrap();
                return &msg[pos+1..];
            }
            Err(_) => msg,
        }
    }
}

// TODO test for set_msg_id
