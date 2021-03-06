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

use byteorder::{ByteOrder, NativeEndian};

// acknowledge object ID deletion
//
// This event is used internally by the object ID management
// logic.  When a client deletes an object, the server will send
// this event to acknowledge that it has seen the delete request.
// When the client receives this event, it will know that it can
// safely reuse the object ID.
#[allow(dead_code)]
pub struct DeleteId {
    pub sender_object_id: u32,
    pub id: u32, // uint: deleted object ID
}

impl super::super::super::event::Event for DeleteId {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 1;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.id);
        encode_offset += 4;
        let _ = encode_offset;
        Ok(())
    }
}

// fatal error event
//
// The error event is sent out when a fatal (non-recoverable)
// error has occurred.  The object_id argument is the object
// where the error occurred, most often in response to a request
// to that object.  The code identifies the error and is defined
// by the object interface.  As such, each interface defines its
// own set of error codes.  The message is a brief description
// of the error, for (debugging) convenience.
#[allow(dead_code)]
pub struct Error {
    pub sender_object_id: u32,
    pub object_id: u32,  // object: object where the error occurred
    pub code: u32,       // uint: error code
    pub message: String, // string: error description
}

impl super::super::super::event::Event for Error {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4 + { 4 + (self.message.len() + 1 + 3) / 4 * 4 };
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 0;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.object_id);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.code);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], (self.message.len() + 1) as u32);
        {
            let mut aligned = self.message.clone();
            aligned.push(0u8.into());
            while aligned.len() % 4 != 0 {
                aligned.push(0u8.into());
            }
            dst[(encode_offset + 4)..(encode_offset + 4 + aligned.len())]
                .copy_from_slice(aligned.as_bytes());
        }

        encode_offset += { 4 + (self.message.len() + 1 + 3) / 4 * 4 };
        let _ = encode_offset;
        Ok(())
    }
}
