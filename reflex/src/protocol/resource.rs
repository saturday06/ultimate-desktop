
use std::sync::Arc;
use std::cell::RefCell;

#[derive(Clone)]
pub enum Resource {
    WlBuffer(Arc<RefCell<super::wayland::wl_buffer::WlBuffer>>),
    WlCallback(Arc<RefCell<super::wayland::wl_callback::WlCallback>>),
    WlCompositor(Arc<RefCell<super::wayland::wl_compositor::WlCompositor>>),
    WlDataDevice(Arc<RefCell<super::wayland::wl_data_device::WlDataDevice>>),
    WlDataDeviceManager(Arc<RefCell<super::wayland::wl_data_device_manager::WlDataDeviceManager>>),
    WlDataOffer(Arc<RefCell<super::wayland::wl_data_offer::WlDataOffer>>),
    WlDataSource(Arc<RefCell<super::wayland::wl_data_source::WlDataSource>>),
    WlDisplay(Arc<RefCell<super::wayland::wl_display::WlDisplay>>),
    WlKeyboard(Arc<RefCell<super::wayland::wl_keyboard::WlKeyboard>>),
    WlOutput(Arc<RefCell<super::wayland::wl_output::WlOutput>>),
    WlPointer(Arc<RefCell<super::wayland::wl_pointer::WlPointer>>),
    WlRegion(Arc<RefCell<super::wayland::wl_region::WlRegion>>),
    WlRegistry(Arc<RefCell<super::wayland::wl_registry::WlRegistry>>),
    WlSeat(Arc<RefCell<super::wayland::wl_seat::WlSeat>>),
    WlShell(Arc<RefCell<super::wayland::wl_shell::WlShell>>),
    WlShellSurface(Arc<RefCell<super::wayland::wl_shell_surface::WlShellSurface>>),
    WlShm(Arc<RefCell<super::wayland::wl_shm::WlShm>>),
    WlShmPool(Arc<RefCell<super::wayland::wl_shm_pool::WlShmPool>>),
    WlSubcompositor(Arc<RefCell<super::wayland::wl_subcompositor::WlSubcompositor>>),
    WlSubsurface(Arc<RefCell<super::wayland::wl_subsurface::WlSubsurface>>),
    WlSurface(Arc<RefCell<super::wayland::wl_surface::WlSurface>>),
    WlTouch(Arc<RefCell<super::wayland::wl_touch::WlTouch>>),
    XdgPopup(Arc<RefCell<super::xdg_shell::xdg_popup::XdgPopup>>),
    XdgPositioner(Arc<RefCell<super::xdg_shell::xdg_positioner::XdgPositioner>>),
    XdgSurface(Arc<RefCell<super::xdg_shell::xdg_surface::XdgSurface>>),
    XdgToplevel(Arc<RefCell<super::xdg_shell::xdg_toplevel::XdgToplevel>>),
    XdgWmBase(Arc<RefCell<super::xdg_shell::xdg_wm_base::XdgWmBase>>),
}

pub fn dispatch_request(resource: Resource, session: &mut super::session::Session, tx: tokio::sync::mpsc::Sender<Box<super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = (), Error = ()>> {
    match resource {
        Resource::WlBuffer(object) => {
            super::wayland::wl_buffer::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlCallback(object) => {
            super::wayland::wl_callback::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlCompositor(object) => {
            super::wayland::wl_compositor::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlDataDevice(object) => {
            super::wayland::wl_data_device::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlDataDeviceManager(object) => {
            super::wayland::wl_data_device_manager::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlDataOffer(object) => {
            super::wayland::wl_data_offer::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlDataSource(object) => {
            super::wayland::wl_data_source::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlDisplay(object) => {
            super::wayland::wl_display::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlKeyboard(object) => {
            super::wayland::wl_keyboard::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlOutput(object) => {
            super::wayland::wl_output::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlPointer(object) => {
            super::wayland::wl_pointer::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlRegion(object) => {
            super::wayland::wl_region::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlRegistry(object) => {
            super::wayland::wl_registry::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlSeat(object) => {
            super::wayland::wl_seat::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlShell(object) => {
            super::wayland::wl_shell::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlShellSurface(object) => {
            super::wayland::wl_shell_surface::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlShm(object) => {
            super::wayland::wl_shm::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlShmPool(object) => {
            super::wayland::wl_shm_pool::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlSubcompositor(object) => {
            super::wayland::wl_subcompositor::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlSubsurface(object) => {
            super::wayland::wl_subsurface::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlSurface(object) => {
            super::wayland::wl_surface::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::WlTouch(object) => {
            super::wayland::wl_touch::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::XdgPopup(object) => {
            super::xdg_shell::xdg_popup::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::XdgPositioner(object) => {
            super::xdg_shell::xdg_positioner::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::XdgSurface(object) => {
            super::xdg_shell::xdg_surface::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::XdgToplevel(object) => {
            super::xdg_shell::xdg_toplevel::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
        Resource::XdgWmBase(object) => {
            super::xdg_shell::xdg_wm_base::dispatch_request(object, session, tx, sender_object_id, opcode, args)
        }
    }
}
