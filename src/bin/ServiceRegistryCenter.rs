use std::collections::HashMap;

struct Service {
    host: String,
    port: u16,
    // 其他服务元数据 ...
}

struct ServiceRegistry {
    services: HashMap<String, Service>,
}

impl ServiceRegistry {
    fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    fn register_service(&mut self, name: String, service: Service) {
        self.services.insert(name, service);
    }

    fn deregister_service(&mut self, name: &str) {
        self.services.remove(name);
    }

    fn find_service(&self, name: &str) -> Option<&Service> {
        self.services.get(name)
    }
}

fn main() {
    let mut registry = ServiceRegistry::new();

    // 注册一个服务
    let service = Service {
        host: "127.0.0.1".to_string(),
        port: 8000,
    };
    registry.register_service("my-service".to_string(), service);

    // 查找一个服务
    if let Some(service) = registry.find_service("my-service") {
        println!("Found service: {}:{}", service.host, service.port);
    } else {
        println!("Service not found");
    }

    // 注销一个服务
    registry.deregister_service("my-service");

    // 再次查找被注销的服务
    if let Some(service) = registry.find_service("my-service") {
        println!("Found service: {}:{}", service.host, service.port);
    } else {
        println!("Service not found");
    }
}