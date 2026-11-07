//! libreswan model — IPsec VPN (Libreswan fork of Openswan).

#[derive(Debug, Clone)]
pub struct Connection {
    pub name: String,
    pub local: String,
    pub remote: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub connections: Vec<Connection>,
}

impl Default for Model {
    fn default() -> Self { Self { connections: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, conn: Connection) { self.connections.push(conn); }

    pub fn up(&mut self, name: &str) -> bool {
        if let Some(c) = self.connections.iter_mut().find(|c| c.name == name) { c.status = "established".into(); true } else { false }
    }

    pub fn down(&mut self, name: &str) -> bool {
        if let Some(c) = self.connections.iter_mut().find(|c| c.name == name) { c.status = "down".into(); true } else { false }
    }

    pub fn list(&self) -> &[Connection] { &self.connections }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_up_down() {
        let mut l = Model::new();
        l.add(Connection { name: "site1".into(), local: "10.0.0.1".into(), remote: "10.0.0.2".into(), status: "down".into() });
        assert!(l.up("site1"));
        assert!(l.down("site1"));
    }
}
