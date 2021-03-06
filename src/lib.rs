// Copyright (c) 2015 Ignacio Corderi

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

// author: Ignacio Corderi

use ::std::io::{Read, Write, Result};
use ::std::io::{Error,ErrorKind};

// TODO: get rid of this
const DIGIT_LIMIT:usize = 64;

pub struct NetstringReader<R> {
    inner: R,
}

pub struct NetstringWriter<W> {
    inner: W,
}

impl<W: Write> NetstringWriter<W> {

    pub fn new(inner: W) -> Self {
        NetstringWriter { inner: inner }
    }

    pub fn write_netstring(&mut self, value: String) -> Result<()> {
        let s = format!("{}:{},", value.len(), value);
        try!(self.inner.write(s.as_bytes()));
        return Ok(());
    }

    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<R: Read> NetstringReader<R> {

    pub fn new(inner: R) -> Self {
        NetstringReader { inner: inner }
    }

    pub fn read_netstring(&mut self) -> Result<String> {
        let ln = try!(self.read_length());
        let mut data = vec![0u8;ln];

        let mut offset = 0usize;
        let mut done = false;

        while !done {
            let r = try!(self.inner.read(data[offset..].as_mut()));
            offset = offset + r;
            if r == 0 || offset == ln {
                done = true;
            }
        }

        // TODO: there has to be a a cleaner way to do this...
        // read tail ","
        let mut t = vec![0u8].into_boxed_slice();
        try!(self.inner.read(t[..].as_mut()));

        // return utf8 string
        match String::from_utf8(data) {
            Ok(s)  => Ok(s),
            Err(err) => Err(Error::new(ErrorKind::InvalidData, err)),
        }
    }

    fn read_length(&mut self) -> Result<usize> {
        let mut t = [0u8; DIGIT_LIMIT];
        let mut current = 0usize;
        let mut done = false;
        while !done {
            try!(self.inner.read(t[current..current+1].as_mut()));
            if t[current] == 0x3A {
                done = true;
            } else {
                current += 1;
            }
        }

        let s = match String::from_utf8(t[..current].to_vec()) {
                    Ok(s)  => s,
                    Err(err) => return Err(Error::new(ErrorKind::InvalidData, err)),
                };

        let ln = match s.parse::<u64>() {
                    Ok(x)  => x,
                    Err(err) => return Err(Error::new(ErrorKind::InvalidData, err)),
                 };


        Ok(ln as usize)
    }

}