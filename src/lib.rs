use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::PyOSError;

#[pyfunction]
fn decrypt(mut bytes: Vec<u8>, key: &[u8]) -> PyResult<Vec<u8>> {
    sqlcrypto::decrypt(bytes.as_mut_slice(), key).map_err(|_|PyOSError::new_err("decryption failure"))?;
    Ok(bytes)
}

#[pyfunction]
fn encrypt(mut bytes: Vec<u8>, key: &[u8]) -> PyResult<Vec<u8>> {
    sqlcrypto::encrypt(bytes.as_mut_slice(), key).map_err(|_|PyOSError::new_err("encryption failure"))?;
    Ok(bytes)
}

#[pymodule]
fn pysqlcrypto(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decrypt, m)?)?;
    m.add_function(wrap_pyfunction!(encrypt, m)?)?;
    Ok(())
}