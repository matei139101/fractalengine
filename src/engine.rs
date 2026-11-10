use winit::event_loop::{ControlFlow, EventLoop};

use crate::{Scheduler, Service, ServiceRegistry, app::App};

pub struct FractalEngine {
    service_registry: ServiceRegistry,
    scheduler: Scheduler,
}

impl FractalEngine {
    pub fn new() -> Self {
        Self {
            service_registry: ServiceRegistry::new(),
            scheduler: Scheduler::new(),
        }
    }

    pub fn register_service<T: Service + 'static>(&mut self, service: T) {
        service.register(&mut self.service_registry);
    }

    pub fn init(&mut self) {
        self.scheduler.schedule_lifecycle_init_hook(&mut self.service_registry);
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);
        let mut app = App::new(&mut self.scheduler, &mut self.service_registry);
        let _ = event_loop.run_app(&mut app);
    }

    pub fn shutdown(&mut self) {
        self.scheduler.schedule_lifecycle_cleanup_hook(&mut self.service_registry);
    }
}
