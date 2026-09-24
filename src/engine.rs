use winit::event_loop::{ControlFlow, EventLoop};

use crate::{Scheduler, ServiceRegistry, app::App};

pub struct FractalEngine {
    scheduler: Scheduler
}

impl FractalEngine {
    pub fn new(service_registry: ServiceRegistry) -> Self {
        Self { scheduler: Scheduler::new(service_registry) }
    }

    pub fn run(&mut self) {
        self.scheduler.schedule_lifecycle_init_hook();

        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);
        let mut app = App::new(&mut self.scheduler);
        let _ = event_loop.run_app(&mut app);
    }
}
