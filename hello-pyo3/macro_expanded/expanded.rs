#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use pyo3::prelude::*;
/// Formats the sum of two numbers as string.
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
#[doc(hidden)]
mod sum_as_string {
    pub(crate) struct MakeDef;
    pub const DEF: ::pyo3::impl_::pyfunction::PyMethodDef = MakeDef::DEF;
}
const _: () = {
    use ::pyo3 as _pyo3;
    impl sum_as_string::MakeDef {
        const DEF: ::pyo3::impl_::pyfunction::PyMethodDef = _pyo3::impl_::pymethods::PyMethodDef::fastcall_cfunction_with_keywords(
            "sum_as_string\0",
            _pyo3::impl_::pymethods::PyCFunctionFastWithKeywords({
                unsafe extern "C" fn trampoline(
                    _slf: *mut _pyo3::ffi::PyObject,
                    _args: *const *mut _pyo3::ffi::PyObject,
                    _nargs: _pyo3::ffi::Py_ssize_t,
                    _kwnames: *mut _pyo3::ffi::PyObject,
                ) -> *mut _pyo3::ffi::PyObject {
                    _pyo3::impl_::trampoline::fastcall_with_keywords(
                        _slf,
                        _args,
                        _nargs,
                        _kwnames,
                        __pyfunction_sum_as_string,
                    )
                }
                trampoline
            }),
            "sum_as_string(a, b)\n--\n\nFormats the sum of two numbers as string.\0",
        );
    }
    #[allow(non_snake_case)]
    unsafe fn __pyfunction_sum_as_string<'py>(
        _py: _pyo3::Python<'py>,
        _slf: *mut _pyo3::ffi::PyObject,
        _args: *const *mut _pyo3::ffi::PyObject,
        _nargs: _pyo3::ffi::Py_ssize_t,
        _kwnames: *mut _pyo3::ffi::PyObject,
    ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
        let function = sum_as_string;
        const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription = _pyo3::impl_::extract_argument::FunctionDescription {
            cls_name: ::std::option::Option::None,
            func_name: "sum_as_string",
            positional_parameter_names: &["a", "b"],
            positional_only_parameters: 0usize,
            required_positional_parameters: 2usize,
            keyword_only_parameters: &[],
        };
        let mut output = [::std::option::Option::None; 2usize];
        let (_args, _kwargs) = DESCRIPTION
            .extract_arguments_fastcall::<
                _pyo3::impl_::extract_argument::NoVarargs,
                _pyo3::impl_::extract_argument::NoVarkeywords,
            >(_py, _args, _nargs, _kwnames, &mut output)?;
        let mut ret = function(
            _pyo3::impl_::extract_argument::extract_argument(
                _pyo3::impl_::extract_argument::unwrap_required_argument(output[0usize]),
                &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                "a",
            )?,
            _pyo3::impl_::extract_argument::extract_argument(
                _pyo3::impl_::extract_argument::unwrap_required_argument(output[1usize]),
                &mut { _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT },
                "b",
            )?,
        );
        let owned = _pyo3::impl_::pymethods::OkWrap::wrap(ret, _py);
        owned
            .map(|obj| _pyo3::conversion::IntoPyPointer::into_ptr(obj))
            .map_err(::core::convert::Into::into)
    }
};
/// A Python module implemented in Rust.
fn hello_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(
        {
            use sum_as_string as wrapped_pyfunction;
            ::pyo3::impl_::pyfunction::wrap_pyfunction_impl(&wrapped_pyfunction::DEF, m)
        }?,
    )?;
    Ok(())
}
#[doc(hidden)]
mod hello_pyo3 {
    pub(crate) struct MakeDef;
    pub static DEF: ::pyo3::impl_::pymodule::ModuleDef = MakeDef::make_def();
    pub const NAME: &'static str = "hello_pyo3\u{0}";
    /// This autogenerated function is called by the python interpreter when importing
    /// the module.
    #[export_name = "PyInit_hello_pyo3"]
    pub unsafe extern "C" fn init() -> *mut ::pyo3::ffi::PyObject {
        ::pyo3::impl_::trampoline::module_init(|py| DEF.make_module(py))
    }
}
const _: () = {
    use ::pyo3::impl_::pymodule as impl_;
    impl hello_pyo3::MakeDef {
        const fn make_def() -> impl_::ModuleDef {
            const INITIALIZER: impl_::ModuleInitializer = impl_::ModuleInitializer(
                hello_pyo3,
            );
            unsafe {
                impl_::ModuleDef::new(
                    hello_pyo3::NAME,
                    "A Python module implemented in Rust.\0",
                    INITIALIZER,
                )
            }
        }
    }
};