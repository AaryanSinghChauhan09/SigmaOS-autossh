//! autossh model — automatically restart SSH sessions.

#[derive(Debug, Clone)]
pub struct Connection {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub status: String,
    pub retries: u32,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub connections: Vec<Connection>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { connections: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn connect(&mut self, host: &str, port: u16) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.connections.push(Connection { id: id.clone(), host: host.into(), port, status: "connected".into(), retries: 0 });
        id
    }

    pub fn reconnect(&mut self, id: &str) -> bool {
        if let Some(c) = self.connections.iter_mut().find(|c| c.id == id) {
            c.status = "reconnecting".into();
            c.retries += 1;
            true
        } else { false }
    }

    pub fn disconnect(&mut self, id: &str) -> bool {
        if let Some(c) = self.connections.iter_mut().find(|c| c.id == id) {
            c.status = "disconnected".into();
            true
        } else { false }
    }

    pub fn list(&self) -> &[Connection] { &self.connections }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn connect_reconnect_disconnect() {
        let mut a = Model::new();
        let id = a.connect("server.example.com", 22);
        assert!(a.reconnect(&id));
        assert!(a.disconnect(&id));
    }
}
