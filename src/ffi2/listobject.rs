use crate::ffi2::object::*;
use crate::ffi2::pyport::Py_ssize_t;
use std::os::raw::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyListObject {
    #[cfg(py_sys_config = "Py_TRACE_REFS")]
    pub _ob_next: *mut PyObject,
    #[cfg(py_sys_config = "Py_TRACE_REFS")]
    pub _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
    pub ob_size: Py_ssize_t,
    pub ob_item: *mut *mut PyObject,
    pub allocated: Py_ssize_t,
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyList_Type")]
    pub static mut PyList_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PyList_Check(op: *mut PyObject) -> c_int {
    PyType_FastSubclass(Py_TYPE(op), Py_TPFLAGS_LIST_SUBCLASS)
}

#[inline]
pub unsafe fn PyList_CheckExact(op: *mut PyObject) -> c_int {
    let u: *mut PyTypeObject = &mut PyList_Type;
    (Py_TYPE(op) == u) as c_int
}

/// Macro, trading safety for speed
#[inline]
#[cfg_attr(PyPy, link_name = "PyPyList_GET_ITEM")]
pub unsafe fn PyList_GET_ITEM(op: *mut PyObject, i: Py_ssize_t) -> *mut PyObject {
    *(*(op as *mut PyListObject)).ob_item.offset(i as isize)
}

#[inline]
#[cfg_attr(PyPy, link_name = "PyPyList_GET_SIZE")]
pub unsafe fn PyList_GET_SIZE(op: *mut PyObject) -> Py_ssize_t {
    Py_SIZE(op)
}

/// Macro, *only* to be used to fill in brand new lists
#[inline(always)]
#[cfg_attr(PyPy, link_name = "PyPyList_SET_ITEM")]
pub unsafe fn PyList_SET_ITEM(op: *mut PyObject, i: Py_ssize_t, v: *mut PyObject) {
    *(*(op as *mut PyListObject)).ob_item.offset(i as isize) = v;
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyList_New")]
    pub fn PyList_New(size: Py_ssize_t) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyList_Size")]
    pub fn PyList_Size(list: *mut PyObject) -> Py_ssize_t;
    #[cfg_attr(PyPy, link_name = "PyPyList_GetItem")]
    pub fn PyList_GetItem(list: *mut PyObject, index: Py_ssize_t) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyList_SetItem")]
    pub fn PyList_SetItem(list: *mut PyObject, index: Py_ssize_t, item: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyList_Insert")]
    pub fn PyList_Insert(list: *mut PyObject, index: Py_ssize_t, item: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyList_Append")]
    pub fn PyList_Append(list: *mut PyObject, item: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyList_GetSlice")]
    pub fn PyList_GetSlice(list: *mut PyObject, low: Py_ssize_t, high: Py_ssize_t)
        -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyList_SetSlice")]
    pub fn PyList_SetSlice(
        list: *mut PyObject,
        low: Py_ssize_t,
        high: Py_ssize_t,
        itemlist: *mut PyObject,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyList_Sort")]
    pub fn PyList_Sort(list: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyList_Reverse")]
    pub fn PyList_Reverse(list: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyList_AsTuple")]
    pub fn PyList_AsTuple(list: *mut PyObject) -> *mut PyObject;
//fn _PyList_Extend(arg1: *mut PyListObject, arg2: *mut PyObject)
//-> *mut PyObject;
}
