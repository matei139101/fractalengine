use crate::{ServiceRegistry, log};

pub struct Scheduler {
}

impl Scheduler {
    pub fn new() -> Self {
        Self{}
    }

    pub fn schedule_lifecycle_init_hook(&self, service_registry: &mut ServiceRegistry) {
        log!(Self, Info, "Calling lifecycle init hooks...");
        for hook in service_registry.lifecycle_hooks() {
            hook.init();
        }
    }

    pub fn schedule_lifecycle_update_hook(&self, service_registry: &mut ServiceRegistry) {
        log!(Self, Info, "Calling lifecycle init hooks...");
        for hook in service_registry.lifecycle_hooks() {
            hook.update();
        }
    }

    pub fn schedule_lifecycle_cleanup_hook(&self, service_registry: &mut ServiceRegistry) {
        log!(Self, Info, "Calling lifecycle cleanup hooks...");
        for hook in service_registry.lifecycle_hooks() {
            hook.cleaup();
        }
    }
}
