use winit::{application::ApplicationHandler, event::WindowEvent, keyboard::PhysicalKey, window::Window};

use crate::{Scheduler, ServiceRegistry};

pub struct App<'a> {
    window: Option<Window>,
    scheduler: &'a mut Scheduler,
    service_registry: &'a mut ServiceRegistry,
}

impl<'a> App<'a> {
    pub fn new(scheduler: &'a mut Scheduler, service_registry: &'a mut ServiceRegistry) -> Self {
        Self { window: Option::None, scheduler, service_registry }
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop.create_window(winit::window::WindowAttributes::default()).unwrap()
        );
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.scheduler.schedule_lifecycle_update_hook(self.service_registry);
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if event.physical_key == PhysicalKey::Code(winit::keyboard::KeyCode::KeyQ) {
                    event_loop.exit();
                }
            }
            _ => (),
        }
    }
}
