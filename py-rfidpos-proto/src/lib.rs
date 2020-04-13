use pyo3::exceptions::TypeError;
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use rfidpos_proto::{ProtoUnion, PROTO_LENGTH};

#[pyclass]
struct RFIDPosProto {
    inner: ProtoUnion,
}

#[pymethods]
impl RFIDPosProto {
    #[new]
    fn new() -> RFIDPosProto {
        RFIDPosProto {
            inner: Default::default(),
        }
    }

    fn update_from_raw_bytes(&mut self, raw_bytes: &PyByteArray) -> PyResult<()> {
        if raw_bytes.len() != PROTO_LENGTH {
            return Err(TypeError::py_err("Invalid packet length!"));
        }

        let result = self.inner.update_from_raw_bytes(&raw_bytes.to_vec()[..]);

        if result.is_err() {
            return Err(TypeError::py_err(result.err().expect("Invalid execution path!")));
        }

        Ok(())
    }

    fn get_transport_buffer<'a>(&self, py: Python<'a>) -> PyResult<&'a PyByteArray> {
        let transport_buffer = PyByteArray::new(py, &self.inner.borrow_raw_frame()[..]);

        Ok(transport_buffer)
    }

    fn fill_with_empty_frame(&mut self, is_write_mode: bool, rank: i32, register: i32) -> PyResult<()> {
        self.inner.fill_with_empty_frame(is_write_mode, rank, register);
        Ok(())
    }

    fn fill(&mut self, is_write_mode: bool, rank: i32, register: i32, data: i32) -> PyResult<()> {
        self.inner.fill(is_write_mode, rank, register, data);
        Ok(())
    }

    fn parse_info(&self) -> PyResult<(bool, bool, i32, i32, i32)> {
        Ok(self.inner.parse_info())
    }
}

#[pymodule]
fn py_rfidpos_proto(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RFIDPosProto>()?;

    Ok(())
}
