use crate::{log, ServiceRegistry};

/// The scheduler struct.
///
/// Currently doesn't do anything other than make the required function calls on the registered
/// [`Service`]s,
#[derive(Default)]
pub struct Scheduler {}

impl Scheduler {
    /// Schedules a [`LifeCycleHook`] initialization call.
    pub fn schedule_lifecycle_init_hook(&self, service_registry: &mut ServiceRegistry) {
        log!(Self, Info, "Calling lifecycle init hooks...");
        for hook in service_registry.lifecycle_hooks() {
            hook.init();
        }
    }

    /// Schedules a [`LifeCycleHook`] update call.
    pub fn schedule_lifecycle_update_hook(&self, service_registry: &mut ServiceRegistry) {
        log!(Self, Info, "Calling lifecycle init hooks...");
        for hook in service_registry.lifecycle_hooks() {
            hook.update();
        }
    }

    /// Schedules a [`LifeCycleHook`] cleanup call.
    pub fn schedule_lifecycle_cleanup_hook(&self, service_registry: &mut ServiceRegistry) {
        log!(Self, Info, "Calling lifecycle cleanup hooks...");
        for hook in service_registry.lifecycle_hooks() {
            hook.cleaup();
        }
    }
}
