/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::slice;
use std::ptr;
use std::cell::UnsafeCell;
use std::mem::zeroed;

use comptr::ComPtr;
use winapi;
use winapi::dwrite;

pub struct FontFace {
    native: UnsafeCell<ComPtr<dwrite::IDWriteFontFace>>,
}

impl FontFace {
    pub fn take(native: ComPtr<dwrite::IDWriteFontFace>) -> FontFace {
        FontFace {
            native: UnsafeCell::new(native)
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut dwrite::IDWriteFontFace {
        (*self.native.get()).as_ptr()
    }

    pub fn get_glyph_count(&self) -> u16 {
        unsafe {
            (*self.native.get()).GetGlyphCount()
        }
    }

    pub fn get_metrics(&self) -> winapi::DWRITE_FONT_METRICS {
        unsafe {
            let mut metrics: winapi::DWRITE_FONT_METRICS = zeroed();
            (*self.native.get()).GetMetrics(&mut metrics);
            metrics
        }
    }

    pub fn get_glyph_indices(&self, code_points: &[u32]) -> Vec<u16> {
        unsafe {
            let mut glyph_indices: Vec<u16> = vec![0; code_points.len()];
            let hr = (*self.native.get()).GetGlyphIndices(code_points.as_ptr(),
                                                          code_points.len() as u32,
                                                          glyph_indices.as_mut_ptr());
            assert!(hr == 0);
            glyph_indices
        }
    }

    pub fn get_design_glyph_metrics(&self, glyph_indices: &[u16], is_sideways: bool) -> Vec<winapi::DWRITE_GLYPH_METRICS> {
        unsafe {
            let mut metrics: Vec<winapi::DWRITE_GLYPH_METRICS> = vec![zeroed(); glyph_indices.len()];
            let hr = (*self.native.get()).GetDesignGlyphMetrics(glyph_indices.as_ptr(),
                                                                glyph_indices.len() as u32,
                                                                metrics.as_mut_ptr(),
                                                                is_sideways as winapi::BOOL);
            assert!(hr == 0);
            metrics
        }
    }

    pub fn get_font_table(&self, opentype_table_tag: u32) -> Option<Vec<u8>> {
        unsafe {
            let mut table_data_ptr: *const u8 = ptr::null_mut();
            let mut table_size: u32 = 0;
            let mut table_context: *mut winapi::c_void = ptr::null_mut();
            let mut exists: winapi::BOOL = winapi::FALSE;

            let hr = (*self.native.get()).TryGetFontTable(opentype_table_tag,
                                                          &mut table_data_ptr as *mut *const _ as *mut *const winapi::c_void,
                                                          &mut table_size,
                                                          &mut table_context,
                                                          &mut exists);
            assert!(hr == 0);

            if exists == winapi::FALSE {
                return None;
            }

            let table_bytes = slice::from_raw_parts(table_data_ptr, table_size as usize).to_vec();

            (*self.native.get()).ReleaseFontTable(table_context);

            Some(table_bytes)
        }
    }
}