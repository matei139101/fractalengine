use std::{rc::Rc};

use crate::log;

pub struct ServiceRegistry {
    services: Vec<Rc<dyn Service>>,
    lifecycle_services: Vec<Rc<dyn LifeCycleHook>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            lifecycle_services: Vec::new(),
        }
    }

    pub fn register_service(&mut self, service_pointer: Rc<dyn Service>) {
        self.services.push(service_pointer);
    }

    pub fn register_lifecycled_service(&mut self, service_pointer: Rc<dyn LifeCycleHook>) {
        self.lifecycle_services.push(service_pointer);
    }

    pub fn lifecycle_hooks(&mut self) -> impl Iterator<Item = &Rc<dyn LifeCycleHook>> {
        self.lifecycle_services.iter()
    }

    pub fn services(&mut self) -> impl Iterator<Item = &Rc<dyn Service>> {
        self.services.iter()
    }
}

pub struct Scheduler {
    service_registry: ServiceRegistry,
}

impl Scheduler {
    pub fn new(service_registry: ServiceRegistry) -> Self {
        Self{service_registry} 
    }

    pub fn schedule_lifecycle_init_hook(&mut self) {
        log!(Self, Info, "Calling lifecycle init hooks...");
        for hook in self.service_registry.lifecycle_hooks() {
            hook.init();
        }
    }

    pub fn schedule_lifecycle_update_hook(&mut self) {
        log!(Self, Info, "Calling lifecycle init hooks...");
        for hook in self.service_registry.lifecycle_hooks() {
            hook.update();
        }
    }

    pub fn schedule_lifecycle_cleanup_hook(&mut self) {
        log!(Self, Info, "Calling lifecycle cleanup hooks...");
        for hook in self.service_registry.lifecycle_hooks() {
            hook.cleaup();
        }
    }
}

pub trait Service {
    fn get_name(&self) -> &str;
}

pub trait LifeCycleHook {
    fn init(&self);
    fn update(&self);
    fn cleaup(&self);
}
