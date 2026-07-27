use crate::environment::Environment;
use crate::native_function::NativeRegistry;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// Security capabilities for sandboxing standard library system APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Capability {
    FileSystem,
    Environment,
    Process,
    Network,
}

/// Runtime configurations.
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub strict_mode: bool,
    pub max_recursion_depth: usize,
    pub enable_assertions: bool,
    pub capabilities: HashSet<Capability>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        let mut capabilities = HashSet::new();
        capabilities.insert(Capability::FileSystem);
        capabilities.insert(Capability::Environment);
        capabilities.insert(Capability::Process);
        capabilities.insert(Capability::Network);
        Self {
            strict_mode: false,
            max_recursion_depth: 1000,
            enable_assertions: true,
            capabilities,
        }
    }
}

/// Safe registry table mapping integer handles to OS and platform objects.
pub struct ResourceTable {
    next_id: u32,
    resources: HashMap<u32, Box<dyn Any>>,
}

impl ResourceTable {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            resources: HashMap::new(),
        }
    }

    pub fn insert<T: Any>(&mut self, resource: T) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.resources.insert(id, Box::new(resource));
        id
    }

    pub fn get<T: Any>(&self, id: u32) -> Option<&T> {
        self.resources
            .get(&id)
            .and_then(|any| any.downcast_ref::<T>())
    }

    pub fn get_mut<T: Any>(&mut self, id: u32) -> Option<&mut T> {
        self.resources
            .get_mut(&id)
            .and_then(|any| any.downcast_mut::<T>())
    }

    pub fn remove<T: Any>(&mut self, id: u32) -> Option<T> {
        let any = self.resources.remove(&id)?;
        match any.downcast::<T>() {
            Ok(boxed) => Some(*boxed),
            Err(any) => {
                // Restore if type mismatch
                self.resources.insert(id, any);
                None
            }
        }
    }
}

/// The state execution context passed through compiler interpreters.
pub struct RuntimeContext {
    pub config: RuntimeConfig,
    pub global_env: Rc<RefCell<Environment>>,
    pub registry: NativeRegistry,
    pub resources: Rc<RefCell<ResourceTable>>,
}

impl RuntimeContext {
    /// Creates a RuntimeContext using the config.
    pub fn new(config: RuntimeConfig) -> Self {
        let global_env = Rc::new(RefCell::new(Environment::new(None)));
        Self {
            config,
            global_env,
            registry: NativeRegistry::new(),
            resources: Rc::new(RefCell::new(ResourceTable::new())),
        }
    }
}
