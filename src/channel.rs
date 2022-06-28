use std::collections::HashSet;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Channel {
    users : HashSet<usize>,
    mutelist : HashMap<usize, Instant>,
    operators : HashSet<usize>,
    owner_user_id : usize,
}
impl Channel {
    pub fn has_user(&self, session : usize) -> bool{
        self.users.get(&session).is_some()
    }

    pub fn mute(&mut self, client :usize, until : Instant) {
        self.mutelist.insert(client, until);
    }
    pub fn isMuted(&self, client : usize) -> bool {
        match self.mutelist.get(&client) {
            None => false,
            Some(v) => Instant::now() < *v
        }
    }

    /*
	def isAdmin(self, client):
		return client and ('admin' in client.accesslevels)

	def isMod(self, client):
		return client and (('mod' in client.accesslevels) or self.isAdmin(client))
    */

	fn isFounder(&self, session : usize) -> bool {
		return session == self.owner_user_id;
    }

	fn isOp(&self, session : usize) -> bool {
		return self.operators.contains(&session) || self.isFounder(session);
    }
}

