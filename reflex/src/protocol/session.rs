use super::resource::Resource;
use super::wayland::wl_compositor::WlCompositor;
use super::wayland::wl_registry::WlRegistry;
use super::wayland::wl_shm::WlShm;
use super::xdg_shell::xdg_wm_base::XdgWmBase;
use super::event::Event;
use tokio::sync::mpsc::Sender;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Session {
    pub resources: HashMap<u32, Resource>,
    pub wl_registry: WlRegistry,
    pub wl_compositor: WlCompositor,
    pub wl_shm: WlShm,
    pub xdg_wm_base: XdgWmBase,
    pub tx: Sender<Box<Event + Send>>,
}

pub struct Context<T> where T: Into<Resource> {
    session: Session,
    pub sender_object_id: u32,
    pub sender_object: T,
}

impl<T> Into<Session> for Context<T> where T: Into<Resource> {
    fn into(mut self) -> Session {
        self.session.resources.insert(self.sender_object_id, self.sender_object.into());
        self.session
    }
}