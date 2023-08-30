use std::sync::{Arc, Mutex};
use rand::Rng;

pub struct LoadBalancer {
    servers: Arc<Mutex<Vec<Server>>>,
    max_failures: usize,
}

struct Server {
    url: String,
    healthy: bool,
    failure_count: usize,
    weight: usize,
}

impl LoadBalancer {
    pub fn new(max_failures: usize) -> Self {
        LoadBalancer {
            servers: Arc::new(Mutex::new(vec![
                Server {
                    url: "http://server1:8000".to_string(),
                    healthy: true,
                    failure_count: 0,
                    weight: 5,
                },
                Server {
                    url: "http://server2:8000".to_string(),
                    healthy: true,
                    failure_count: 0,
                    weight: 10,
                },
            ])),
            max_failures,
        }
    }

    pub fn add_server(&self, server: String, weight: usize) {
        let mut servers = self.servers.lock().unwrap();
        servers.push(Server {
            url: server,
            healthy: true,
            failure_count: 0,
            weight,
        });
    }

    pub fn get_server(&self, algorithm: LoadBalancingAlgorithm) -> Option<String> {
        let servers = self.servers.lock().unwrap();
        let healthy_servers: Vec<&Server> = servers
            .iter()
            .filter(|s| s.healthy)
            .collect();

        match algorithm {
            LoadBalancingAlgorithm::RoundRobin => {
                if healthy_servers.is_empty() {
                    None
                } else {
                    // ... (same as in your original LoadBalancer::get_server implementation)
                }
            }
            LoadBalancingAlgorithm::Random => {
                if healthy_servers.is_empty() {
                    None
                } else {
                    // ... (same as in your original LoadBalancer::get_server implementation)
                }
            }
        }
    }

    pub fn update_failure_count(&self, server_url: &str, increment: bool) {
        let mut servers = self.servers.lock().unwrap();
        if let Some(server) = servers.iter_mut().find(|s| s.url == server_url) {
            if increment {
                server.failure_count += 1;
            } else {
                server.failure_count = 0;
            }

            if server.failure_count >= self.max_failures {
                server.healthy = false;
            }
        }
    }

    pub fn update_weight(&self, server_url: &str, new_weight: usize) {
        let mut servers = self.servers.lock().unwrap();
        if let Some(server) = servers.iter_mut().find(|s| s.url == server_url) {
            server.weight = new_weight;
        }
    }
}

enum LoadBalancingAlgorithm {
    RoundRobin,
    Random,
}
