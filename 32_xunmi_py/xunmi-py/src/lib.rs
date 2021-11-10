use pyo3::{exceptions, prelude::*};
use std::{fs, str::FromStr};
use xunmi::{self as x};

pub(crate) fn to_pyerr<E: ToString>(err: E) -> PyErr {
    exceptions::PyValueError::new_err(err.to_string())
}

#[pyclass]
pub struct Indexer(x::Indexer);

#[pyclass]
pub struct InputConfig(x::InputConfig);

#[pyclass]
pub struct IndexUpdater(x::IndexUpdater);

#[pymethods]
impl Indexer {
    #[new]
    pub fn open_or_create(filename: &str) -> PyResult<Indexer> {
        let content = fs::read_to_string(filename).map_err(to_pyerr)?;
        let config = x::IndexConfig::from_str(&content).map_err(to_pyerr)?;
        let indexer = x::Indexer::open_or_create(config).map_err(to_pyerr)?;
        Ok(Indexer(indexer))
    }

    pub fn get_updater(&self) -> IndexUpdater {
        IndexUpdater(self.0.get_updater())
    }

    pub fn search(
        &self,
        query: String,
        fields: Vec<String>,
        limit: usize,
        offset: Option<usize>,
    ) -> PyResult<Vec<(f32, String)>> {
        let default_fields: Vec<_> = fields.iter().map(|s| s.as_str()).collect();
        let data: Vec<_> = self
            .0
            .search(&query, &default_fields, limit, offset.unwrap_or(0))
            .map_err(to_pyerr)?
            .into_iter()
            .map(|(score, doc)| (score, serde_json::to_string(&doc).unwrap()))
            .collect();

        Ok(data)
    }

    pub fn reload(&self) -> PyResult<()> {
        self.0.reload().map_err(to_pyerr)
    }
}

#[pymethods]
impl IndexUpdater {
    pub fn add(&mut self, input: &str, config: &InputConfig) -> PyResult<()> {
        self.0.add(input, &config.0).map_err(to_pyerr)
    }

    pub fn update(&mut self, input: &str, config: &InputConfig) -> PyResult<()> {
        self.0.update(input, &config.0).map_err(to_pyerr)
    }

    pub fn commit(&mut self) -> PyResult<()> {
        self.0.commit().map_err(to_pyerr)
    }

    pub fn clear(&self) -> PyResult<()> {
        self.0.clear().map_err(to_pyerr)
    }
}

#[pymethods]
impl InputConfig {
    #[new]
    fn new(
        input_type: String,
        mapping: Option<Vec<(String, String)>>,
        conversion: Option<Vec<(String, (String, String))>>,
    ) -> PyResult<Self> {
        let input_type = match input_type.as_ref() {
            "yaml" | "yml" => x::InputType::Yaml,
            "json" => x::InputType::Json,
            "xml" => x::InputType::Xml,
            _ => return Err(exceptions::PyValueError::new_err("Invalid input type")),
        };
        let conversion = conversion
            .unwrap_or_default()
            .into_iter()
            .filter_map(|(k, (t1, t2))| {
                let t = match (t1.as_ref(), t2.as_ref()) {
                    ("string", "number") => (x::ValueType::String, x::ValueType::Number),
                    ("number", "string") => (x::ValueType::Number, x::ValueType::String),
                    _ => return None,
                };
                Some((k, t))
            })
            .collect::<Vec<_>>();

        Ok(Self(x::InputConfig::new(
            input_type,
            mapping.unwrap_or_default(),
            conversion,
        )))
    }
}

#[pymodule]
fn xunmi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Indexer>()?;
    m.add_class::<InputConfig>()?;
    m.add_class::<IndexUpdater>()?;
    Ok(())
}
