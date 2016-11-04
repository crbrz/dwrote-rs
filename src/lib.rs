/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate lazy_static;
#[macro_use(DEFINE_GUID)]
extern crate winapi;
extern crate kernel32;
extern crate libc;
extern crate serde;
extern crate dwrite;

mod comptr;
mod helpers;

#[cfg(test)]
mod test;

mod font_family;
pub use font_family::FontFamily;

mod font_collection;
pub use font_collection::FontCollection;

use dwrite::DWriteCreateFactory;
use winapi::dwrite::{DWRITE_FACTORY_TYPE_SHARED};
use winapi::dwrite::{IDWriteFactory};

use comptr::ComPtr;
use winapi::S_OK;

DEFINE_GUID!{UuidOfIDWriteFactory, 0xb859ee5a, 0xd838, 0x4b5b, 0xa2, 0xe8, 0x1a, 0xdc, 0x7d, 0x93, 0xdb, 0x48}

unsafe impl Sync for ComPtr<IDWriteFactory> { }

lazy_static! {
    static ref DWRITE_FACTORY_RAW_PTR: usize = {
        unsafe {
            let mut factory: ComPtr<IDWriteFactory> = ComPtr::new();
            let hr = DWriteCreateFactory(
                DWRITE_FACTORY_TYPE_SHARED,
                &UuidOfIDWriteFactory,
                factory.getter_addrefs());
            assert!(hr == S_OK);
            factory.forget() as usize
        }
    };
}

// FIXME vlad would be nice to return, say, FactoryPtr<IDWriteFactory>
// that has a DerefMut impl, so that we can write
// DWriteFactory().SomeOperation() as opposed to
// (*DWriteFactory()).SomeOperation()
#[allow(non_snake_case)]
fn DWriteFactory() -> *mut IDWriteFactory {
    (*DWRITE_FACTORY_RAW_PTR) as *mut IDWriteFactory
}