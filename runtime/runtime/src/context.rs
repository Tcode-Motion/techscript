use crate::environment::Environment;
use crate::native_function::NativeRegistry;
use std::cell::RefCell;
use std::collections::HashSet;
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

/// The state execution context passed through compiler interpreters.
pub struct RuntimeContext {
    pub config: RuntimeConfig,
    pub global_env: Rc<RefCell<Environment>>,
    pub registry: NativeRegistry,
}

impl RuntimeContext {
    /// Creates a RuntimeContext using the config.
    pub fn new(config: RuntimeConfig) -> Self {
        let global_env = Rc::new(RefCell::new(Environment::new(None)));
        Self {
            config,
            global_env,
            registry: NativeRegistry::new(),
        }
    }
}
