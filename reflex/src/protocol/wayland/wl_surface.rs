// Copyright © 2008-2011 Kristian Høgsberg
// Copyright © 2010-2011 Intel Corporation
// Copyright © 2012-2013 Collabora, Ltd.
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation files
// (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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

// an onscreen surface
//
// A surface is a rectangular area that is displayed on the screen.
// It has a location, size and pixel contents.
//
// The size of a surface (and relative positions on it) is described
// in surface-local coordinates, which may differ from the buffer
// coordinates of the pixel content, in case a buffer_transform
// or a buffer_scale is used.
//
// A surface without a "role" is fairly useless: a compositor does
// not know where, when or how to present it. The role is the
// purpose of a wl_surface. Examples of roles are a cursor for a
// pointer (as set by wl_pointer.set_cursor), a drag icon
// (wl_data_device.start_drag), a sub-surface
// (wl_subcompositor.get_subsurface), and a window as defined by a
// shell protocol (e.g. wl_shell.get_shell_surface).
//
// A surface can have only one role at a time. Initially a
// wl_surface does not have a role. Once a wl_surface is given a
// role, it is set permanently for the whole lifetime of the
// wl_surface object. Giving the current role again is allowed,
// unless explicitly forbidden by the relevant interface
// specification.
//
// Surface roles are given by requests in other interfaces such as
// wl_pointer.set_cursor. The request should explicitly mention
// that this request gives a role to a wl_surface. Often, this
// request also creates a new protocol object that represents the
// role and adds additional functionality to wl_surface. When a
// client wants to destroy a wl_surface, they must destroy this 'role
// object' before the wl_surface.
//
// Destroying the role object does not remove the role from the
// wl_surface, but it may stop the wl_surface from "playing the role".
// For instance, if a wl_subsurface object is destroyed, the wl_surface
// it was created for will be unmapped and forget its position and
// z-order. It is allowed to create a wl_subsurface for the same
// wl_surface again, but it is not allowed to use the wl_surface as
// a cursor (cursor is a different role than sub-surface, and role
// switching is not allowed).
pub struct WlSurface {}

impl WlSurface {
    // set the surface contents
    //
    // Set a buffer as the content of this surface.
    //
    // The new size of the surface is calculated based on the buffer
    // size transformed by the inverse buffer_transform and the
    // inverse buffer_scale. This means that the supplied buffer
    // must be an integer multiple of the buffer_scale.
    //
    // The x and y arguments specify the location of the new pending
    // buffer's upper left corner, relative to the current buffer's upper
    // left corner, in surface-local coordinates. In other words, the
    // x and y, combined with the new surface size define in which
    // directions the surface's size changes.
    //
    // Surface contents are double-buffered state, see wl_surface.commit.
    //
    // The initial surface contents are void; there is no content.
    // wl_surface.attach assigns the given wl_buffer as the pending
    // wl_buffer. wl_surface.commit makes the pending wl_buffer the new
    // surface contents, and the size of the surface becomes the size
    // calculated from the wl_buffer, as described above. After commit,
    // there is no pending buffer until the next attach.
    //
    // Committing a pending wl_buffer allows the compositor to read the
    // pixels in the wl_buffer. The compositor may access the pixels at
    // any time after the wl_surface.commit request. When the compositor
    // will not access the pixels anymore, it will send the
    // wl_buffer.release event. Only after receiving wl_buffer.release,
    // the client may reuse the wl_buffer. A wl_buffer that has been
    // attached and then replaced by another attach instead of committed
    // will not receive a release event, and is not used by the
    // compositor.
    //
    // Destroying the wl_buffer after wl_buffer.release does not change
    // the surface contents. However, if the client destroys the
    // wl_buffer before receiving the wl_buffer.release event, the surface
    // contents become undefined immediately.
    //
    // If wl_surface.attach is sent with a NULL wl_buffer, the
    // following wl_surface.commit will remove the surface content.
    pub fn attach(
        context: Context<WlSurface>,
        _buffer: u32, // object: buffer of surface contents
        _x: i32,      // int: surface-local x coordinate
        _y: i32,      // int: surface-local y coordinate
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("wl_surface::attach is not implemented yet".to_string())
    }

    // commit pending surface state
    //
    // Surface state (input, opaque, and damage regions, attached buffers,
    // etc.) is double-buffered. Protocol requests modify the pending state,
    // as opposed to the current state in use by the compositor. A commit
    // request atomically applies all pending state, replacing the current
    // state. After commit, the new pending state is as documented for each
    // related request.
    //
    // On commit, a pending wl_buffer is applied first, and all other state
    // second. This means that all coordinates in double-buffered state are
    // relative to the new wl_buffer coming into use, except for
    // wl_surface.attach itself. If there is no pending wl_buffer, the
    // coordinates are relative to the current surface contents.
    //
    // All requests that need a commit to become effective are documented
    // to affect double-buffered state.
    //
    // Other interfaces may add further double-buffered surface state.
    pub fn commit(
        context: Context<WlSurface>,
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        println!("WlSurface::commit");
        context.ok()
    }

    // mark part of the surface damaged
    //
    // This request is used to describe the regions where the pending
    // buffer is different from the current surface contents, and where
    // the surface therefore needs to be repainted. The compositor
    // ignores the parts of the damage that fall outside of the surface.
    //
    // Damage is double-buffered state, see wl_surface.commit.
    //
    // The damage rectangle is specified in surface-local coordinates,
    // where x and y specify the upper left corner of the damage rectangle.
    //
    // The initial value for pending damage is empty: no damage.
    // wl_surface.damage adds pending damage: the new pending damage
    // is the union of old pending damage and the given rectangle.
    //
    // wl_surface.commit assigns pending damage as the current damage,
    // and clears pending damage. The server will clear the current
    // damage as it repaints the surface.
    //
    // Alternatively, damage can be posted with wl_surface.damage_buffer
    // which uses buffer coordinates instead of surface coordinates,
    // and is probably the preferred and intuitive way of doing this.
    pub fn damage(
        context: Context<WlSurface>,
        _x: i32,      // int: surface-local x coordinate
        _y: i32,      // int: surface-local y coordinate
        _width: i32,  // int: width of damage rectangle
        _height: i32, // int: height of damage rectangle
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("wl_surface::damage is not implemented yet".to_string())
    }

    // mark part of the surface damaged using buffer coordinates
    //
    // This request is used to describe the regions where the pending
    // buffer is different from the current surface contents, and where
    // the surface therefore needs to be repainted. The compositor
    // ignores the parts of the damage that fall outside of the surface.
    //
    // Damage is double-buffered state, see wl_surface.commit.
    //
    // The damage rectangle is specified in buffer coordinates,
    // where x and y specify the upper left corner of the damage rectangle.
    //
    // The initial value for pending damage is empty: no damage.
    // wl_surface.damage_buffer adds pending damage: the new pending
    // damage is the union of old pending damage and the given rectangle.
    //
    // wl_surface.commit assigns pending damage as the current damage,
    // and clears pending damage. The server will clear the current
    // damage as it repaints the surface.
    //
    // This request differs from wl_surface.damage in only one way - it
    // takes damage in buffer coordinates instead of surface-local
    // coordinates. While this generally is more intuitive than surface
    // coordinates, it is especially desirable when using wp_viewport
    // or when a drawing library (like EGL) is unaware of buffer scale
    // and buffer transform.
    //
    // Note: Because buffer transformation changes and damage requests may
    // be interleaved in the protocol stream, it is impossible to determine
    // the actual mapping between surface and buffer damage until
    // wl_surface.commit time. Therefore, compositors wishing to take both
    // kinds of damage into account will have to accumulate damage from the
    // two requests separately and only transform from one to the other
    // after receiving the wl_surface.commit.
    pub fn damage_buffer(
        context: Context<WlSurface>,
        _x: i32,      // int: buffer-local x coordinate
        _y: i32,      // int: buffer-local y coordinate
        _width: i32,  // int: width of damage rectangle
        _height: i32, // int: height of damage rectangle
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("wl_surface::damage_buffer is not implemented yet".to_string())
    }

    // delete surface
    //
    // Deletes the surface and invalidates its object ID.
    pub fn destroy(
        context: Context<WlSurface>,
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("wl_surface::destroy is not implemented yet".to_string())
    }

    // request a frame throttling hint
    //
    // Request a notification when it is a good time to start drawing a new
    // frame, by creating a frame callback. This is useful for throttling
    // redrawing operations, and driving animations.
    //
    // When a client is animating on a wl_surface, it can use the 'frame'
    // request to get notified when it is a good time to draw and commit the
    // next frame of animation. If the client commits an update earlier than
    // that, it is likely that some updates will not make it to the display,
    // and the client is wasting resources by drawing too often.
    //
    // The frame request will take effect on the next wl_surface.commit.
    // The notification will only be posted for one frame unless
    // requested again. For a wl_surface, the notifications are posted in
    // the order the frame requests were committed.
    //
    // The server must send the notifications so that a client
    // will not send excessive updates, while still allowing
    // the highest possible update rate for clients that wait for the reply
    // before drawing again. The server should give some time for the client
    // to draw and commit after sending the frame callback events to let it
    // hit the next output refresh.
    //
    // A server should avoid signaling the frame callbacks if the
    // surface is not visible in any way, e.g. the surface is off-screen,
    // or completely obscured by other opaque surfaces.
    //
    // The object returned by this request will be destroyed by the
    // compositor after the callback is fired and as such the client must not
    // attempt to use it after that point.
    //
    // The callback_data passed in the callback is the current time, in
    // milliseconds, with an undefined base.
    pub fn frame(
        context: Context<WlSurface>,
        _callback: u32, // new_id: callback object for the frame request
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("wl_surface::frame is not implemented yet".to_string())
    }

    // sets the buffer scaling factor
    //
    // This request sets an optional scaling factor on how the compositor
    // interprets the contents of the buffer attached to the window.
    //
    // Buffer scale is double-buffered state, see wl_surface.commit.
    //
    // A newly created surface has its buffer scale set to 1.
    //
    // wl_surface.set_buffer_scale changes the pending buffer scale.
    // wl_surface.commit copies the pending buffer scale to the current one.
    // Otherwise, the pending and current values are never changed.
    //
    // The purpose of this request is to allow clients to supply higher
    // resolution buffer data for use on high resolution outputs. It is
    // intended that you pick the same buffer scale as the scale of the
    // output that the surface is displayed on. This means the compositor
    // can avoid scaling when rendering the surface on that output.
    //
    // Note that if the scale is larger than 1, then you have to attach
    // a buffer that is larger (by a factor of scale in each dimension)
    // than the desired surface size.
    //
    // If scale is not positive the invalid_scale protocol error is
    // raised.
    pub fn set_buffer_scale(
        context: Context<WlSurface>,
        _scale: i32, // int: positive scale for interpreting buffer contents
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("wl_surface::set_buffer_scale is not implemented yet".to_string())
    }

    // sets the buffer transformation
    //
    // This request sets an optional transformation on how the compositor
    // interprets the contents of the buffer attached to the surface. The
    // accepted values for the transform parameter are the values for
    // wl_output.transform.
    //
    // Buffer transform is double-buffered state, see wl_surface.commit.
    //
    // A newly created surface has its buffer transformation set to normal.
    //
    // wl_surface.set_buffer_transform changes the pending buffer
    // transformation. wl_surface.commit copies the pending buffer
    // transformation to the current one. Otherwise, the pending and current
    // values are never changed.
    //
    // The purpose of this request is to allow clients to render content
    // according to the output transform, thus permitting the compositor to
    // use certain optimizations even if the display is rotated. Using
    // hardware overlays and scanning out a client buffer for fullscreen
    // surfaces are examples of such optimizations. Those optimizations are
    // highly dependent on the compositor implementation, so the use of this
    // request should be considered on a case-by-case basis.
    //
    // Note that if the transform value includes 90 or 270 degree rotation,
    // the width of the buffer will become the surface height and the height
    // of the buffer will become the surface width.
    //
    // If transform is not one of the values from the
    // wl_output.transform enum the invalid_transform protocol error
    // is raised.
    pub fn set_buffer_transform(
        context: Context<WlSurface>,
        _transform: i32, // int: transform for interpreting buffer contents
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context
            .invalid_method("wl_surface::set_buffer_transform is not implemented yet".to_string())
    }

    // set input region
    //
    // This request sets the region of the surface that can receive
    // pointer and touch events.
    //
    // Input events happening outside of this region will try the next
    // surface in the server surface stack. The compositor ignores the
    // parts of the input region that fall outside of the surface.
    //
    // The input region is specified in surface-local coordinates.
    //
    // Input region is double-buffered state, see wl_surface.commit.
    //
    // wl_surface.set_input_region changes the pending input region.
    // wl_surface.commit copies the pending region to the current region.
    // Otherwise the pending and current regions are never changed,
    // except cursor and icon surfaces are special cases, see
    // wl_pointer.set_cursor and wl_data_device.start_drag.
    //
    // The initial value for an input region is infinite. That means the
    // whole surface will accept input. Setting the pending input region
    // has copy semantics, and the wl_region object can be destroyed
    // immediately. A NULL wl_region causes the input region to be set
    // to infinite.
    pub fn set_input_region(
        context: Context<WlSurface>,
        _region: u32, // object: input region of the surface
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("wl_surface::set_input_region is not implemented yet".to_string())
    }

    // set opaque region
    //
    // This request sets the region of the surface that contains
    // opaque content.
    //
    // The opaque region is an optimization hint for the compositor
    // that lets it optimize the redrawing of content behind opaque
    // regions.  Setting an opaque region is not required for correct
    // behaviour, but marking transparent content as opaque will result
    // in repaint artifacts.
    //
    // The opaque region is specified in surface-local coordinates.
    //
    // The compositor ignores the parts of the opaque region that fall
    // outside of the surface.
    //
    // Opaque region is double-buffered state, see wl_surface.commit.
    //
    // wl_surface.set_opaque_region changes the pending opaque region.
    // wl_surface.commit copies the pending region to the current region.
    // Otherwise, the pending and current regions are never changed.
    //
    // The initial value for an opaque region is empty. Setting the pending
    // opaque region has copy semantics, and the wl_region object can be
    // destroyed immediately. A NULL wl_region causes the pending opaque
    // region to be set to empty.
    pub fn set_opaque_region(
        context: Context<WlSurface>,
        _region: u32, // object: opaque region of the surface
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("wl_surface::set_opaque_region is not implemented yet".to_string())
    }
}
