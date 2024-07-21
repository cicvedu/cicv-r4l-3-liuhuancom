// SPDX-License-Identifier: GPL-2.0

//! Rust character device sample.

use core::result::Result::Err;

use kernel::prelude::*;
use kernel::sync::Mutex;
use kernel::{chrdev, file};

const GLOBALMEM_SIZE: usize = 0x1000;

module! {
    type: RustChrdev,
    name: "rust_chrdev",
    author: "Rust for Linux Contributors",
    description: "Rust character device sample",
    license: "GPL",
}

static GLOBALMEM_BUF: Mutex<[u8;GLOBALMEM_SIZE]> = unsafe {
    Mutex::new([0u8;GLOBALMEM_SIZE])
};

struct RustFile {
    #[allow(dead_code)]
    inner: &'static Mutex<[u8;GLOBALMEM_SIZE]>,
}

#[vtable]
impl file::Operations for RustFile {
    type Data = Box<Self>;

    fn open(_shared: &(), _file: &file::File) -> Result<Box<Self>> {
        Ok(
            Box::try_new(RustFile {
                inner: &GLOBALMEM_BUF
            })?
        )
    }

    fn write(_this: &Self,_file: &file::File,_reader: &mut impl kernel::io_buffer::IoBufferReader,_offset:u64,) -> Result<usize> {
        // Err(EPERM)
        
        let mut guard = _this.inner.lock(); 
        let buffer = &mut *guard;

        if _offset as usize >= GLOBALMEM_SIZE {
            return Err(EINVAL);
        }

        let remaining_space = GLOBALMEM_SIZE - _offset as usize;
        let data_to_write = core::cmp::min(_reader.len(), remaining_space);

        unsafe {
            _reader.read_raw(buffer.as_mut_ptr().add(_offset as usize), data_to_write)?;
        }

        Ok(data_to_write)
    }

    fn read(_this: &Self,_file: &file::File,_writer: &mut impl kernel::io_buffer::IoBufferWriter,_offset:u64,) -> Result<usize> {
        // Err(EPERM)
        
        let guard = _this.inner.lock();
        let buffer = &*guard;

        if _offset as usize >= GLOBALMEM_SIZE {
            return Ok(0);
        }

        let remaining_data = GLOBALMEM_SIZE - _offset as usize;
        let data_to_read = core::cmp::min(_writer.len(), remaining_data);

        unsafe {
            _writer.write_raw(buffer.as_ptr().add(_offset as usize), data_to_read)?;
        }

        Ok(data_to_read)
    }
}

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
}

impl kernel::Module for RustChrdev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust character device sample (init)\n");

        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;

        // Register the same kind of device twice, we're just demonstrating
        // that you can use multiple minors. There are two minors in this case
        // because its type is `chrdev::Registration<2>`
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustChrdev { _dev: chrdev_reg })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust character device sample (exit)\n");
    }
}
