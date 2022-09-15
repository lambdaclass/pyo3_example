use pyo3::prelude::*;
use pyo3::types::PyDict;
use inner::Inner;

#[pyclass]
struct Outer {
    inner: &mut Inner,
}

impl Outer {
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
            locals.set_item("outer", cell);
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
