use crate::core::builder::petpet_builder::PetpetBuilder;
use crate::core::encoder::encoder::{EncodeFormat, IMAGE_ENCODER};
use crate::core::errors::Error;
use crate::core::http::avatar_data_factory::create_avatar_data;
use crate::core::http::template_data::{AvatarDataURL, PetpetData};
use crate::core::template::petpet_template::PetpetTemplate;
use crate::core::template::text_template::TextData;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString, PyTuple};
use pythonize::depythonize;

#[pyclass]
struct PyPetpetBuilder {
    builder: Arc<PetpetBuilder>,
}

#[pyclass]
enum PyOutputFormat {
    GIF,
    PNG,
}

impl From<EncodeFormat> for PyOutputFormat {
    fn from(value: EncodeFormat) -> Self {
        match value {
            EncodeFormat::PNG => Self::PNG,
            EncodeFormat::GIF => Self::GIF,
        }
    }
}

#[pymethods]
impl PyPetpetBuilder {
    #[new]
    fn new(py_template: &PyAny, path: &PyString) -> PyResult<Self> {
        let template: PetpetTemplate = depythonize(
            py_template
        ).unwrap();
        match PetpetBuilder::new(
            template, path.to_string(),
        ) {
            Ok(builder) => Ok(PyPetpetBuilder {
                builder: Arc::new(builder)
            }),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }

    fn build<'p>(&self, py: Python<'p>, py_data: &'p PyAny) -> PyResult<&'p PyAny> {
        let data: PetpetData = depythonize(
            py_data
        ).unwrap();
        let builder = self.builder.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let data = data.clone();
            let avatar_data = create_avatar_data(&data.avatar).unwrap();
            let (images, delay) = builder.build(
                avatar_data,
                data.text,
            ).await.unwrap();
            let (blob, format) = IMAGE_ENCODER.encode(&images, delay).unwrap();

            let bytes: Py<PyTuple> = Python::with_gil(|py|
            PyTuple::new(
                py, vec![
                    PyBytes::new(py, &blob).into_py(py),
                    PyOutputFormat::from(format).into_py(py),
                ],
            ).into_py(py)
            );
            Ok(bytes)
        })
    }
}

#[pymodule]
fn petpet(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPetpetBuilder>()?;
    m.add_class::<PyOutputFormat>()?;
    Ok(())
}