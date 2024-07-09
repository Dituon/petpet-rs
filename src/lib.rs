extern crate alloc;
#[cfg(feature = "java")]
extern crate jni;


#[cfg(feature = "java")]
use jni::JNIEnv;
#[cfg(feature = "java")]
use jni::objects::{JByteArray, JClass, JObject, JString, AsJArrayRaw, JObjectArray};
#[cfg(feature = "java")]
use once_cell::sync::Lazy;
#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::{PyBytes, PyString, PyTuple};
#[cfg(feature = "python")]
use pythonize::depythonize;









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
pub extern "C" fn Java_PetpetRsBuilder_createBuilder<'local>(
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
fn create_builder(template: String, path: String) -> Result<PetpetBuilder, Error> {
    let template: PetpetTemplate = serde_json::from_str(&template)?;
    PetpetBuilder::new(template, path)
}

macro_rules! jni_string_option_prop {
    ($env:expr, $obj:expr, $field:ident) => {
        let $field: JString = $env.get_field(&$obj, stringify!($field), "Ljava/lang/String;").unwrap().l().unwrap().into();
        let $field: Option<String> = if $field.is_null() {
            None
        } else {
            Some($env.get_string(&$field).unwrap().into())
        };
    }
}

macro_rules! jni_string_prop {
    ($env:expr, $obj:expr, $field:ident) => {
        jni_string_option_prop!($env, $obj, $field);
        let $field = if let Some(str) = $field { str } else { "".to_string() };
    }
}

macro_rules! jni_string_array_option_prop {
    ($env:expr, $obj:expr, $field:ident) => {
        let jarr_raw: JObjectArray = JObjectArray::from($env.get_field(&$obj, stringify!($field), "[Ljava/lang/String;").unwrap().l().unwrap());
        let $field: Option<Vec<String>> = if !jarr_raw.is_null() {
            let jarr_len = $env.get_array_length(&jarr_raw).unwrap();
            let mut strs: Vec<String> = Vec::with_capacity(jarr_len as usize);
            for i in 0..jarr_len {
                let element: JString = $env.get_object_array_element(&jarr_raw, i).unwrap().into();
                let rust_string: String = $env.get_string(&element).unwrap().into();
                strs.push(rust_string);
            }
            Some(strs)
        } else {
            None
        };
    }
}

macro_rules! jni_string_array_prop {
    ($env:expr, $obj:expr, $field:ident) => {
        jni_string_array_option_prop!($env, $obj, $field);
        let $field = if let Some(strs) = $field { strs } else { Vec::new() };
    }
}

#[cfg(feature = "java")]
#[no_mangle]
pub unsafe extern "C" fn Java_PetpetRsBuilder_builderBuildByString<'local>(
    mut env: JNIEnv<'local>, class: JClass<'local>,
    ptr: *mut PetpetBuilder,
    data: JString<'local>,
) -> JByteArray<'local> {
    if ptr.is_null() {
        let _ = env.throw_new("java/lang/NullPointerException", "");
        return JByteArray::default();
    }

    match builder_build_by_string(&mut env, class, ptr, data) {
        Ok(arr) => arr,
        Err(e) => {
            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", e));
            JByteArray::default()
        }
    }
}

#[cfg(feature = "java")]
unsafe fn builder_build_by_string<'local>(
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
pub unsafe extern "C" fn Java_PetpetRsBuilder_builderBuildByObjects<'local>(
    mut env: JNIEnv<'local>, class: JClass<'local>,
    ptr: *mut PetpetBuilder,
    avatar_data: JObject<'local>, text_data: JObject<'local>,
) -> JByteArray<'local> {
    if ptr.is_null() {
        let _ = env.throw_new("java/lang/NullPointerException", "");
        return JByteArray::default();
    }

    match builder_build_by_objects(&mut env, class, ptr, avatar_data, text_data) {
        Ok(arr) => arr,
        Err(e) => {
            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", e));
            JByteArray::default()
        }
    }
}

#[cfg(feature = "java")]
unsafe fn builder_build_by_objects<'local>(
    env: &mut JNIEnv<'local>, _class: JClass<'local>,
    ptr: *mut PetpetBuilder,
    avatar_data: JObject<'local>, text_data: JObject<'local>,
) -> Result<JByteArray<'local>, Error> {
    let avatar_data_url = {
        jni_string_option_prop!(env, avatar_data, from);
        jni_string_option_prop!(env, avatar_data, to);
        jni_string_option_prop!(env, avatar_data, group);
        jni_string_option_prop!(env, avatar_data, bot);
        jni_string_array_option_prop!(env, avatar_data, random);
        AvatarDataURL { from, to, bot, group, random }
    };
    let text_data = {
        jni_string_prop!(env, text_data, from);
        jni_string_prop!(env, text_data, to);
        jni_string_prop!(env, text_data, group);
        jni_string_array_prop!(env, text_data, textList);
        TextData { from, to, group, text_list: textList }
    };

    let builder = Box::from_raw(ptr);
    let avatar_data = create_avatar_data(&avatar_data_url)?;
    let (images, delay) = RUNTIME.block_on(
        builder.build(avatar_data, text_data)
    )?;
    let (blob, _format) = IMAGE_ENCODER.encode(&images, delay)?;
    Ok(env.byte_array_from_slice(&blob).unwrap())
}

#[cfg(feature = "java")]
#[no_mangle]
pub unsafe extern "C" fn Java_PetpetRsBuilder_closeBuilder(_env: JNIEnv, _class: JClass, ptr: *mut PetpetBuilder) {
    if ptr.is_null() {
        return;
    }
    let builder = Box::from_raw(ptr);
    std::mem::drop(builder)
}

