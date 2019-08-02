use crate::protocol::wayland_event::WaylandEvent;
use crate::protocol::wayland_request::WaylandRequest;
use crate::protocol::wl_resource::WlResource;
use crate::session_state::SessionState;
use futures::future::Future;
use std::sync::{Arc, RwLock};
use bytes::BytesMut;
use std::io::Cursor;
use byteorder::NativeEndian;

pub struct XdgToplevel {}

impl XdgToplevel {
    fn destroy(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        _tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let removed = {
            let object_map = &mut session_state.write().unwrap().object_map;
            if let Some(WlResource::XdgToplevel(_)) = object_map.get(&sender_object_id) {
                object_map.remove(&sender_object_id);
                true
            } else {
                false
            }
        };
        if removed {
            Box::new(futures::future::ok(()))
        } else {
            Box::new(futures::future::err(()))
        }
    }

    fn handle(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        opcode: u16,
        args: Vec<u8>,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let mut cursor = Cursor::new(&args);
        match opcode {
            0 if args.len() == 0 => {
                return Self::destroy(sender_object, session_state, tx, sender_object_id);
            }
            _ => {}
        };

        Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "XdgToplevel@{} opcode={} args={:?} not found",
                    sender_object_id, opcode, args,
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        )
    }
}
