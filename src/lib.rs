extern crate libc;
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;

pub mod ffi;
mod util;

use std::ffi::CString;
use std::os::raw::{c_double, c_int, c_uint, c_void};
use std::path;


#[derive(Debug)]
pub struct TextAttr {
    pub font_name: Option<String>,
    pub font_size: f64,
    pub is_underlined: bool,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub start_index: usize,
    pub end_index: usize
}

impl glib::translate::GlibPtrDefault for TextAttr {
    type GlibType = *const ffi::PopplerTextAttributes;
}

impl glib::translate::FromGlibPtrNone<*const ffi::PopplerTextAttributes> for TextAttr {
    unsafe fn from_glib_none(ptr: *const ffi::PopplerTextAttributes) -> Self {
        let p = &*ptr;
        assert!(p.start_index >=0 && p.end_index >= p.start_index);
        let attr = TextAttr {
            font_name: match std::ffi::CStr::from_ptr(p.font_name).to_str() {
                        Err(_) => None,
                        Ok(x) => Some(x.to_string())},
            font_size: p.font_size,
            is_underlined: glib::translate::from_glib(p.is_underlined),
            red: p.color.red,
            green: p.color.green,
            blue: p.color.blue,
            start_index: p.start_index as usize,
            end_index: p.end_index as usize
        };
        attr
    }
}

impl glib::translate::FromGlibPtrFull<*const ffi::PopplerTextAttributes> for TextAttr {
    unsafe fn from_glib_full(ptr: *const ffi::PopplerTextAttributes) -> Self {
        let p = &*ptr;
        assert!(p.start_index >=0 && p.end_index >= p.start_index);
        let attr = TextAttr {
            font_name: match std::ffi::CStr::from_ptr(p.font_name).to_str() {
                        Err(_) => None,
                        Ok(x) => Some(x.to_string())},
            font_size: p.font_size,
            is_underlined: glib::translate::from_glib(p.is_underlined),
            red: p.color.red,
            green: p.color.green,
            blue: p.color.blue,
            start_index: p.start_index as usize,
            end_index: p.end_index as usize
        };
        attr
    }
}



#[derive(Debug)]
pub struct PopplerDocument(*mut ffi::PopplerDocument);

#[derive(Debug)]
pub struct PopplerPage(*mut ffi::PopplerPage);

impl PopplerDocument {
    pub fn new_from_file<P: AsRef<path::Path>>(
        p: P,
        password: &str,
    ) -> Result<PopplerDocument, glib::error::Error> {
        let pw = CString::new(password).map_err(|_| {
            glib::error::Error::new(
                glib::FileError::Inval,
                "Password invalid (possibly contains NUL characters)",
            )
        })?;

        let path_cstring = util::path_to_glib_url(p)?;
        let doc = util::call_with_gerror(|err_ptr| unsafe {
            ffi::poppler_document_new_from_file(path_cstring.as_ptr(), pw.as_ptr(), err_ptr)
        })?;

        Ok(PopplerDocument(doc))
    }

    pub fn get_n_pages(&self) -> usize {
        // FIXME: what's the correct type here? can we assume a document
        //        has a positive number of pages?
        (unsafe { ffi::poppler_document_get_n_pages(self.0) }) as usize
    }

    pub fn get_page(&self, index: usize) -> Result<PopplerPage, glib::error::Error> {
        match unsafe { ffi::poppler_document_get_page(self.0, index as c_int) } {
            ptr if ptr.is_null() => Err(
                glib::error::Error::new(
                    glib::FileError::Failed,
                    "poppler-glib-rs: Unknown error in function ffi::popler_document_get_page",)
                ),
            ptr => Ok(PopplerPage(ptr)),
        }
    }
}


impl Drop for PopplerDocument {
    fn drop(&mut self) {
        unsafe {
            gobject_sys::g_object_unref(self.0 as *mut gobject_sys::GObject);
        }
    }
}

impl PopplerPage {
    pub fn get_size(&self) -> (f64, f64) {
        let mut width: f64 = 0.0;
        let mut height: f64 = 0.0;

        unsafe {
            ffi::poppler_page_get_size(
                self.0,
                &mut width as *mut f64 as *mut c_double,
                &mut height as *mut f64 as *mut c_double,
            )
        }

        (width, height)
    }

    pub fn get_text(&self) -> String {
        unsafe {
            let p = ffi::poppler_page_get_text(self.0);
            let s = String::from(std::ffi::CStr::from_ptr(p).to_str().unwrap());
            libc::free(p as *mut libc::c_void);
            s
        }
    }

    pub fn get_text_lossy(&self) -> String {
        unsafe {
            let p = ffi::poppler_page_get_text(self.0);
            let s = std::ffi::CStr::from_ptr(p).to_string_lossy().into_owned();
            libc::free(p as *mut libc::c_void);
            s
        }
    }

    pub fn get_text_layout(&self) -> Result<Vec<ffi::PopplerRectangle>, glib::error::Error> {
        unsafe {
            let mut arr: *mut ffi::PopplerRectangle = std::ptr::null_mut();
            let mut len: c_uint = 0;
            let res = ffi::poppler_page_get_text_layout(self.0, &mut arr, &mut len);
            let b: bool = glib::translate::from_glib(res);
            if !b {
                return Err(glib::error::Error::new(
                    glib::FileError::Failed, 
                    "poppler-glib-rs: Unknown error in function ffi::poppler_page_get_text_layout",))
            }
            let res = Vec::from(std::slice::from_raw_parts(arr, len as usize));
            glib_sys::g_free(arr as *mut libc::c_void);
            Ok(res)
        }
    }

    pub fn get_text_attributes(&self) -> Vec<TextAttr> {
        unsafe {
            let ll = ffi::poppler_page_get_text_attributes(self.0);
            let res = glib::translate::FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ll);
            ffi::poppler_page_free_text_attributes(ll);
            res
        }
    }
}

impl Drop for PopplerPage {
    fn drop(&mut self) {
        unsafe {
            gobject_sys::g_object_unref(self.0 as *mut gobject_sys::GObject);
        }
    }
}

#[derive(Debug)]
pub struct PoppperPageRef {
    ptr: *mut c_void,
}

