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

use crate::protocol::resource::Resource;
#[allow(unused_imports)]
use crate::protocol::session::{Context, NextAction, Session};
#[allow(unused_imports)]
use futures::future::{err, ok, Future};
#[allow(unused_imports)]
use futures::sink::Sink;
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};

pub mod enums;
pub mod events;
mod lib;
pub use lib::*;

// create desktop-style surfaces
//
// The xdg_wm_base interface is exposed as a global object enabling clients
// to turn their wl_surfaces into windows in a desktop environment. It
// defines the basic functionality needed for clients and the compositor to
// create windows that can be dragged, resized, maximized, etc, as well as
// creating transient windows such as popup menus.
pub struct XdgWmBase {}

impl XdgWmBase {
    // create a positioner object
    //
    // Create a positioner object. A positioner object is used to position
    // surfaces relative to some parent surface. See the interface description
    // and xdg_surface.get_popup for details.
    pub fn create_positioner(
        context: Context<Arc<RwLock<XdgWmBase>>>,
        _id: u32, // new_id:
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("xdg_wm_base::create_positioner is not implemented yet".to_string())
    }

    // destroy xdg_wm_base
    //
    // Destroy this xdg_wm_base object.
    //
    // Destroying a bound xdg_wm_base object while there are surfaces
    // still alive created by this xdg_wm_base object instance is illegal
    // and will result in a protocol error.
    pub fn destroy(
        context: Context<Arc<RwLock<XdgWmBase>>>,
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("xdg_wm_base::destroy is not implemented yet".to_string())
    }

    // create a shell surface from a surface
    //
    // This creates an xdg_surface for the given surface. While xdg_surface
    // itself is not a role, the corresponding surface may only be assigned
    // a role extending xdg_surface, such as xdg_toplevel or xdg_popup.
    //
    // This creates an xdg_surface for the given surface. An xdg_surface is
    // used as basis to define a role to a given surface, such as xdg_toplevel
    // or xdg_popup. It also manages functionality shared between xdg_surface
    // based surface roles.
    //
    // See the documentation of xdg_surface for more details about what an
    // xdg_surface is and how it is used.
    pub fn get_xdg_surface(
        mut context: Context<Arc<RwLock<XdgWmBase>>>,
        id: u32,      // new_id:
        surface: u32, // object:
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        println!("XdgWmBase::get_xdg_surface(id={}, surface={})", id, surface);
        if let Some(Resource::WlSurface(wl_surface)) = context.resources.get_mut(&surface) {
            wl_surface.xdg_surface_id = Some(id);
        } else {
            return context.invalid_method("unknown surface id".into());
        };
        context.resources.insert(
            id,
            crate::protocol::xdg_shell::xdg_surface::XdgSurface {
                wl_surface_id: surface,
            }
            .into(),
        );
        context.ok()
    }

    // respond to a ping event
    //
    // A client must respond to a ping event with a pong request or
    // the client may be deemed unresponsive. See xdg_wm_base.ping.
    pub fn pong(
        context: Context<Arc<RwLock<XdgWmBase>>>,
        _serial: u32, // uint: serial of the ping event
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("xdg_wm_base::pong is not implemented yet".to_string())
    }
}
