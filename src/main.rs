use pyo3::prelude::*;

fn main() -> PyResult<()> {
    let codepath = include_str!("pymodule.py");

    Python::with_gil(|py| {
        let pymod = PyModule::from_code_bound(py, codepath, "pymodule.py", "pymodule")?;
        let args = (3, 4);
        let rresult = pymod.call_method1("add", args)?;
        let result = rresult.extract::<i32>()?;
        println!("Result is {}", result);
        Ok(())
    })
}