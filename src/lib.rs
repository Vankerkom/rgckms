use std::slice::Iter;

#[derive(Debug)]
pub struct Server {
    pub ip: String,
    pub port: String,
}

impl Server {
    pub fn new(ip: String, port: String) -> Server {
        Server { ip, port }
    }
}

#[derive(Debug)]
pub struct ServerList {
    servers: Vec<Server>,
}

impl ServerList {
    pub fn new() -> ServerList {
        ServerList {
            servers: Vec::new(),
        }
    }

    pub fn add(&mut self, server: Server) -> () {
        self.servers.push(server)
    }

    pub fn servers(&self) -> Iter<Server> {
        return self.servers.iter();
    }
}
