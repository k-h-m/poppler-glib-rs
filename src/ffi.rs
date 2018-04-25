use std::os::raw::{c_char, c_double, c_int, c_uint, c_ushort};
use glib_sys;


// FIXME: is this the correct way to get opaque types?
// FIXME: alternative: https://docs.rs/cairo-sys-rs/0.5.0/src/cairo_sys/lib.rs.html#64
// NOTE: https://github.com/rust-lang/rust/issues/27303
// NOTE: ask F/O about this
pub enum PopplerDocument {}
pub enum PopplerPage {}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct PopplerRectangle {
    pub x1: c_double,
    pub y1: c_double,
    pub x2: c_double,
    pub y2: c_double
}

#[derive(Debug)]
#[repr(C)]
pub struct PopplerTextAttributes {
    pub font_name: *const c_char,
    pub font_size: c_double,
    pub is_underlined: glib_sys::gboolean,
    pub color: PopplerColor,
    pub start_index: c_int,
    pub end_index: c_int
}

#[derive(Debug)]
#[repr(C)]
pub struct PopplerColor {
    pub red: c_ushort,
    pub green: c_ushort,
    pub blue: c_ushort
}

// FIXME: *const instead of mut pointers?

#[link(name = "poppler-glib")]
extern "C" {
    pub fn poppler_document_new_from_file(
        uri: *const c_char,
        password: *const c_char,
        error: *mut *mut glib_sys::GError,
    ) -> *mut PopplerDocument;
    pub fn poppler_document_get_n_pages(document: *mut PopplerDocument) -> c_int;
    pub fn poppler_document_get_page(
        document: *mut PopplerDocument,
        index: c_int,
    ) -> *mut PopplerPage;

    pub fn poppler_page_get_size(
        page: *mut PopplerPage,
        width: *mut c_double,
        height: *mut c_double,
    );

    pub fn poppler_page_get_text(page: *mut PopplerPage) -> *const c_char;


    pub fn poppler_page_get_text_layout(page: *mut PopplerPage, rectangles: *mut *mut PopplerRectangle,
                              n_rectangles: *mut c_uint) -> glib_sys::gboolean;

    pub fn poppler_page_get_text_attributes(page: *mut PopplerPage) -> *const glib_sys::GList;

    pub fn poppler_page_free_text_attributes(list: *const glib_sys::GList);
}
