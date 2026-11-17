use std::{ rc::Rc};

/// The service registry struct.
///
/// Services use this to register themselves to their required capabilities so they obtain the
/// necessary hooks. 
#[derive(Default)]
pub struct ServiceRegistry {
    services: Vec<Rc<dyn Service>>,
    lifecycle_services: Vec<Rc<dyn LifeCycleHook>>,
}

impl ServiceRegistry {
    pub(crate) fn register<T: Service + 'static>(&mut self, service: T) {
        let service_ptr = Rc::new(service);
        self.services.push(service_ptr.clone());

        if let Some(lifecycle_service) = T::capabilities(service_ptr).lifecycle {
            self.lifecycle_services.push(lifecycle_service);
        }
    }

    /// Returns an iterator of all [`LifeCycleHook`] supporting [`Service`]s.
    pub(crate) fn lifecycle_hooks(&mut self) -> impl Iterator<Item = &Rc<dyn LifeCycleHook>> {
        self.lifecycle_services.iter()
    }

    /// Returns an iterator of all registered [`Service`]'s.
    pub fn _services(&mut self) -> impl Iterator<Item = &Rc<dyn Service>> {
        self.services.iter()
    }
}

/// The main trait required to be implemented by every engine service.
///
/// This is required for uniformity between services and it ensures every service follows the same
/// registration protocol. This is also why it is a very barebone trait with few functions, as most
/// if not all actual functionality is intended to be implemented through utilizing hooks.
pub trait Service {
    /// This returns the name of the service. Technically optional but having a name sometimes helps
    /// in minor cases.
    fn get_name(&self) -> &str;

    fn capabilities(self_ptr: Rc<Self>) -> Capabilities where Self:Sized + 'static;
}

pub trait LifeCycleHook {
    /// This hook is called once before the engine enters the main lifecycle loop.
    ///
    /// Use this to execute work before the services enter the full lifecycle loop such as creating
    /// a vulkan instance.
    fn init(&self);

    /// This hook is called every window redraw during the engine's lyfecycle loop.
    ///
    /// Use this for work meant to be executed each frame, such as rendering the frame or handling
    /// input.
    fn update(&self);

    /// This hook is called once after exiting the main lifecycle loop
    ///
    /// Use this for work that must be done before exiting the game loop. Such as dropping a vulkan
    /// instance.
    fn cleaup(&self);
}

#[derive(Default)]
pub struct Capabilities {
    pub(crate) lifecycle: Option<Rc<dyn LifeCycleHook>>,
}

impl Capabilities {
    pub fn lifecycle(mut self, service: Rc<impl LifeCycleHook + 'static>) -> Self {
        self.lifecycle = Some(service);
        self
    }
}
