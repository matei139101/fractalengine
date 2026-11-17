use winit::event_loop::{ControlFlow, EventLoop};

use crate::{Scheduler, Service, app::App, log, ServiceRegistry};

/// The main engine struct.
///
/// Handles engine related calls such as registering services, scheduling of tasks and calling
/// hooks. Publishes various functions relating to basic engine operations, further game related
/// code and mechanics must be called through hooks inside other services.
///
/// # Examples
/// ```ignore
/// use fractalengine::FractalEngine;
/// use example_service::ExampleService;
///
/// let mut engine = FractalEngine::default();
/// 
/// let example_service = ExampleService::default();
///
/// engine.register_service(example_service);
///
/// engine.init();
/// engine.run();
/// engine.shutdown();
/// ```
#[derive(Default)]
pub struct FractalEngine {
    service_registry: ServiceRegistry,
    scheduler: Scheduler,

    initialized: bool,
}

impl FractalEngine {
    /// Registers a [`Service`] into the engine's [`ServiceRegistry`].
    ///
    /// Services must be registered before calling [`init`](Self::init).
    pub fn register_service<T: Service + 'static>(&mut self, service: T) {
        self.service_registry.register(service);
    }

    /// Runs the lifecycle init hooks of all registered services.
    ///
    /// Use this for work a [`Service`] must finish before the game loop starts. Call it exactly
    /// once, after all services are registered and before [`run`](Self::run).
    ///
    /// # Panics
    ///
    /// Panics if engine has already been initialized once during its lifetime. The initialization
    /// is only meant to be called once.
    pub fn init(&mut self) {
        if self.initialized {
            log!(Self, Critical, "Engine has already been initialized!");
            panic!()
        }

        self.scheduler.schedule_lifecycle_init_hook(&mut self.service_registry);
        self.initialized = true;
    }

    /// Enters the main game loop, blocking the current thread until the event loop exits.
    ///
    /// Returns when the window is closed or the loop is otherwise terminated. Call
    /// [`shutdown`](Self::shutdown) afterwards to run cleanup hooks.
    ///
    /// # Panics
    ///
    /// Panics if [`run`](Self::run) is called before the engine has been initialized.
    pub fn run(&mut self) {
        if !self.initialized {
            log!(Self, Critical, "Engine hasn't been initialized yet! Initialize before running.");
            panic!()
        }

        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);
        let mut app = App::new(&mut self.scheduler, &mut self.service_registry);
        let _ = event_loop.run_app(&mut app);
    }

    /// Runs the lifecycle cleanup hooks of all registered services.
    ///
    /// Use this for work a [`Service`] must do before the process exits. 
    pub fn cleanup(&mut self) {
        self.scheduler.schedule_lifecycle_cleanup_hook(&mut self.service_registry);
    }
}
