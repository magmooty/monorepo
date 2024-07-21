use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

pub struct Route<T, E> {
    pub path: String,
    pub handler: fn(payload: Value) -> Result<T, E>,
}

pub struct Router<T, E> {
    path_prefix: String,
    routes: HashMap<String, Route<T, E>>,
}

impl<T, E> Router<T, E> {
    pub fn for_root() -> Router<T, E> {
        Router {
            path_prefix: "/".to_string(),
            routes: HashMap::new(),
        }
    }

    pub fn for_child(prefix: String) -> Router<T, E> {
        Router {
            path_prefix: prefix,
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, mut route: Route<T, E>) -> () {
        if self.routes.contains_key(&route.path) {
            panic!("Route already exists");
        }

        let base_path = Path::new(self.path_prefix.as_str());

        let prefixed_path = base_path
            .join(route.path.clone())
            .to_str()
            .expect("Invalid route provided")
            .to_string();

        route.path = prefixed_path.clone();

        self.routes.insert(prefixed_path, route);
    }

    pub fn join_router(&mut self, router: Router<T, E>) {
        for (_, route) in router.routes {
            if self.routes.contains_key(&route.path) {
                panic!("Route already exists");
            }
            self.add_route(route);
        }
    }

    pub fn handle_route(&self, route: String, body: Value) -> Result<T, E> {
        if !self.routes.contains_key(route.as_str()) {
            panic!("Route not found");
        }
        (self.routes.get(route.as_str()).unwrap().handler)(body)
    }
}
