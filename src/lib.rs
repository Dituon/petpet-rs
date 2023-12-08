extern crate alloc;
#[cfg(feature = "java")]
extern crate jni;

#[cfg(feature = "java")]
use jni::JNIEnv;
#[cfg(feature = "java")]
use jni::objects::{JByteArray, JClass, JString};
#[cfg(feature = "java")]
use once_cell::sync::Lazy;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::prelude::Python;
#[cfg(feature = "python")]
use pyo3::types::{PyBytes, PyString, PyTuple};
#[cfg(feature = "python")]
use pythonize::depythonize;
#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use std::sync::Arc;

use crate::core::builder::petpet_builder::PetpetBuilder;
use crate::core::encoder::encoder::{EncodeFormat, IMAGE_ENCODER};
use crate::core::errors::Error;
use crate::core::http::avatar_data_factory::create_avatar_data;
use crate::core::http::template_data::PetpetData;
use crate::core::template::petpet_template::PetpetTemplate;

mod core;

#[cfg(feature = "python")]
#[pyclass]
struct PyPetpetBuilder {
    builder: Arc<PetpetBuilder>,
}

#[cfg(feature = "python")]
#[pyclass]
enum PyOutputFormat {
    GIF,
    PNG,
}

#[cfg(feature = "python")]
impl From<EncodeFormat> for PyOutputFormat {
    fn from(value: EncodeFormat) -> Self {
        match value {
            EncodeFormat::PNG => Self::PNG,
            EncodeFormat::GIF => Self::GIF,
        }
    }
}

#[cfg(feature = "python")]
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

#[cfg(feature = "python")]
#[pymodule]
fn petpet(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPetpetBuilder>()?;
    m.add_class::<PyOutputFormat>()?;
    Ok(())
}


#[cfg(feature = "java")]
static RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(||
    tokio::runtime::Runtime::new().unwrap()
);

#[cfg(feature = "java")]
#[no_mangle]
pub extern "C" fn Java_moe_d2n_petpetrs_PetpetRsBuilder_createBuilder<'local>(
    mut env: JNIEnv<'local>, _class: JClass<'local>,
    template: JString<'local>, path: JString<'local>,
) -> *const PetpetBuilder {
    let template_raw: String = env.get_string(&template).expect("Couldn't get java string!").into();
    let path: String = env.get_string(&path).expect("Couldn't get java string!").into();

    match create_builder(template_raw, path) {
        Ok(builder) => {
            Box::into_raw(Box::new(builder))
        }
        Err(e) => {
            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", e));
            std::ptr::null()
        }
    }
}

#[cfg(feature = "java")]
fn create_builder(template: String, path: String) -> Result<PetpetBuilder, Error>{
    let template: PetpetTemplate = serde_json::from_str(&template)?;
    PetpetBuilder::new(template, path)
}

#[cfg(feature = "java")]
#[no_mangle]
pub unsafe extern "C" fn Java_moe_d2n_petpetrs_PetpetRsBuilder_builderBuild<'local>(
    mut env: JNIEnv<'local>, class: JClass<'local>,
    ptr: *mut PetpetBuilder,
    data: JString<'local>,
) -> JByteArray<'local> {
    if ptr.is_null() {
        let _ = env.throw_new("java/lang/NullPointerException", "");
        return JByteArray::default();
    }

    match builder_build(&mut env, class, ptr, data) {
        Ok(arr) => arr,
        Err(e) => {
            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", e));
            JByteArray::default()
        }
    }
}

#[cfg(feature = "java")]
unsafe fn builder_build<'local>(
    env: &mut JNIEnv<'local>, _class: JClass<'local>,
    ptr: *mut PetpetBuilder,
    data: JString<'local>,
) -> Result<JByteArray<'local>, Error> {
    let data_raw: String = env.get_string(&data).expect("Couldn't get java string!").into();
    let data: PetpetData = serde_json::from_str(&data_raw)?;
    let builder = Box::from_raw(ptr);
    let avatar_data = create_avatar_data(&data.avatar)?;
    let (images, delay) = RUNTIME.block_on(
        builder.build(avatar_data, data.text)
    )?;
    let (blob, _format) = IMAGE_ENCODER.encode(&images, delay)?;
    Ok(env.byte_array_from_slice(&blob).unwrap())
}

#[cfg(feature = "java")]
#[no_mangle]
pub unsafe extern "C" fn Java_moe_d2n_petpetrs_PetpetRsBuilder_closeBuilder(_env: JNIEnv, _class: JClass, ptr: *mut PetpetBuilder) {
    if ptr.is_null() {
        return;
    }
    let builder = Box::from_raw(ptr);
    std::mem::drop(builder)
}

