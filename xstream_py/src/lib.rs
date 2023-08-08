// use std::time::Duration;

use pyo3::prelude::*;

#[pyclass]
pub struct Server {
  s: xstream::Server,
}

pub fn server_host_port(host: String, port: u16) -> Server {
  Server(xstream::Server::host_port(host, port))
}

// #[pyfunction]
// fn sleep_for(py: Python<'_>) -> PyResult<&PyAny> {
//   pyo3_asyncio::tokio::future_into_py(py, async {
//     tokio::time::sleep(std::time::Duration::from_secs(3)).await;
//     Ok(Python::with_gil(|py| py.None()))
//   })
// }
//
// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//   Ok((a + b).to_string())
// }

/// A Python module implemented in Rust.
#[pymodule]
fn xstream_py(_py: Python, m: &PyModule) -> PyResult<()> {
  // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
  m.add_function(wrap_pyfunction!(server_host_port, m)?)?;
  m.add_class::<Server>()?;
  Ok(())
}
