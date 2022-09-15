use std::cell::RefCell;
use std::rc::Rc;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use inner::Inner;

#[pyclass(unsendable)]
pub struct Outer {
    inner: &mut Inner,
}

#[pymethods]
impl Outer {
    #[new]
    fn new() -> Outer {
        Outer {
            inner: &mut Inner::new(),
        }
    }
}

#[pyclass]
pub struct PyPrinter;

impl PyPrinter {
    pub fn run(self, outer: Outer) {
        Python::with_gil(|py| {
            let cell = PyCell::new(py, outer).unwrap();
            let locals = PyDict::new(py);
            locals.set_item("outer", cell).unwrap();
            py.run("print(outer)", None, Some(locals)).unwrap();  
        });
    }
}

#[pymodule]
fn ffi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPrinter>()?;
    m.add_class::<Outer>()?;
    Ok(())
}
