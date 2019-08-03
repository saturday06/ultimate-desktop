// Copyright © 2008-2013 Kristian Høgsberg
// Copyright © 2013      Rafael Antognolli
// Copyright © 2013      Jasper St. Pierre
// Copyright © 2010-2013 Intel Corporation
// Copyright © 2015-2017 Samsung Electronics Co., Ltd
// Copyright © 2015-2017 Red Hat Inc.
// 
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#[allow(unused_imports)] use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
#[allow(unused_imports)] use futures::future::Future;
#[allow(unused_imports)] use futures::sink::Sink;
#[allow(unused_imports)] use std::convert::TryInto;
#[allow(unused_imports)] use std::io::{Cursor, Read};
#[allow(unused_imports)] use std::sync::{Arc, RwLock};

pub const VERSION: u32 = 2;

#[allow(unused_variables)]
pub fn dispatch_request(request: crate::protocol::session::Context<crate::protocol::xdg_shell::xdg_popup::XdgPopup>, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {

            if Ok(cursor.position()) != args.len().try_into() {
                let tx = request.tx.clone();
                return Box::new(tx.send(Box::new(crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: request.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "xdg_popup@{} opcode={} args={:?} not found",
                        request.sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| request.into()));
            }
            return super::XdgPopup::destroy(request);
        },
        1 => {
            let seat = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                let tx = request.tx.clone();
                return Box::new(tx.send(Box::new(crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: request.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "xdg_popup@{} opcode={} args={:?} not found",
                        request.sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| request.into()));
            };
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                let tx = request.tx.clone();
                return Box::new(tx.send(Box::new(crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: request.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "xdg_popup@{} opcode={} args={:?} not found",
                        request.sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| request.into()));
            };

            if Ok(cursor.position()) != args.len().try_into() {
                let tx = request.tx.clone();
                return Box::new(tx.send(Box::new(crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: request.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "xdg_popup@{} opcode={} args={:?} not found",
                        request.sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| request.into()));
            }
            return super::XdgPopup::grab(request, seat, serial);
        },
        _ => {},
    };
    Box::new(futures::future::ok(request.into()))
}

impl Into<crate::protocol::resource::Resource> for crate::protocol::xdg_shell::xdg_popup::XdgPopup {
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::XdgPopup(self)
    }
}
