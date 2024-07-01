use std::{collections::HashMap, fmt::Display};

use super::modbus;

const METRIC_NAME_PREFIX: &str = "goodwe_";

pub struct MetricSet {
    pub(crate) base: u16,
    pub(crate) metrics: Vec<Box<dyn Metric>>,
}

impl MetricSet {
    pub fn read_data(&mut self, data: &[u8]) -> Result<(), MetricReadError> {
        let metric_len = self.metrics.len();
        for metric in &mut self.metrics[0..metric_len] {
            let metric: &mut dyn Metric = metric.as_mut();
            metric.read_data(self.base, &data)?
        }

        Ok(())
    }

    pub fn get_modbus_command(&self, addr: u8) -> Vec<u8> {
        let data_len = self
            .metrics
            .iter()
            .max_by_key(|x| x.get_register())
            .unwrap()
            .get_register()
            - self.base
            + 1; // add one to also get the second byte of the 16 bit word
        modbus::create_command(modbus::Command::ReadMulti, addr, self.base, data_len)
    }

    fn gen_types_list(&self) -> HashMap<String, MetricType> {
        let mut retval = HashMap::new();

        for metric in &self.metrics {
            retval.insert(metric.get_name(), metric.get_type());
        }

        retval
    }
}

impl Display for MetricSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in &self.gen_types_list() {
            writeln!(f, "# TYPE {} {}", entry.0, entry.1)?;
        }
        for metric in &self.metrics {
            writeln!(f, "{}", metric)?;
        }

        Ok(())
    }
}

pub struct BaseMetric {
    #[allow(dead_code)]
    metric_type: MetricType,
    metric_name: String,
    labels: Vec<KV<String, String>>,
    register: u16,
}

impl BaseMetric {
    pub fn new(
        metric_type: MetricType,
        metric_name: String,
        labels: Vec<KV<String, String>>,
        register: u16,
    ) -> Self {
        Self {
            metric_type,
            metric_name,
            labels,
            register,
        }
    }

    pub fn get_register(&self) -> u16 {
        self.register
    }
}

impl Display for BaseMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{", self.metric_name)?;
        for kv in &self.labels {
            write!(f, "{}, ", kv)?;
        }
        write!(f, "}}")
    }
}

pub trait Metric: Display {
    #[allow(dead_code)]
    fn read_data(&mut self, base_register: u16, data: &[u8]) -> Result<(), MetricReadError>;
    fn get_register(&self) -> u16;
    fn get_name(&self) -> String;
    fn get_type(&self) -> MetricType;
}

#[derive(Clone, Copy)]
pub enum MetricType {
    Counter,
    Gauge,
}

impl Display for MetricType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricType::Counter => write!(f, "counter"),
            MetricType::Gauge => write!(f, "gauge"),
        }
    }
}

pub enum MetricReadError {
    OutOfBounds,
    #[allow(dead_code)]
    GeneralError,
}

impl Display for MetricReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricReadError::OutOfBounds => write!(f, "MetricReadError::OutOfBounds"),
            MetricReadError::GeneralError => write!(f, "MetricReadError::GeneralError"),
        }
    }
}

pub struct KV<K, V>
where
    K: Display,
    V: Display,
{
    key: K,
    value: V,
}

impl<K, V> Display for KV<K, V>
where
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=\"{}\"", self.key, self.value)
    }
}

impl<K, V> KV<K, V>
where
    K: Display,
    V: Display,
{
    #[allow(dead_code)]
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

fn get_register_bytes<const WIDTH: usize>(
    data: &[u8],
    base_register: usize,
    register: usize,
) -> Result<[u8; WIDTH], MetricReadError> {
    let offset = (base_register - register) * 2;
    if offset > (data.len() - WIDTH) {
        return Err(MetricReadError::OutOfBounds);
    }

    let mut retval: [u8; WIDTH] = [0_u8; WIDTH];
    retval.copy_from_slice(&data[offset..(offset + WIDTH)]);
    Ok(retval)
}

pub struct Voltage {
    base: BaseMetric,
    value: Option<f32>,
}

impl Voltage {
    pub fn new(register: u16, metric_name: &str, labels: Vec<KV<String, String>>) -> Self {
        let mut metric_name: String = metric_name.to_owned();
        metric_name.insert_str(0, METRIC_NAME_PREFIX);
        let base = BaseMetric::new(MetricType::Gauge, metric_name, labels, register);
        Self { base, value: None }
    }

    pub fn easy(register: u16, metric_name: &str, key: &str, value: &str) -> Box<dyn Metric> {
        Box::new(Self::new(
            register,
            metric_name,
            vec![KV {
                key: key.to_string(),
                value: value.to_string(),
            }],
        ))
    }
}

impl Metric for Voltage {
    fn read_data(&mut self, base_register: u16, data: &[u8]) -> Result<(), MetricReadError> {
        let value =
            get_register_bytes::<2>(&data, self.base.register as usize, base_register as usize)?;
        let value = i16::from_be_bytes(value);
        self.value = Some(value as f32 / 10.0);

        Ok(())
    }

    fn get_register(&self) -> u16 {
        self.base.get_register()
    }

    fn get_name(&self) -> String {
        self.base.metric_name.clone()
    }

    fn get_type(&self) -> MetricType {
        self.base.metric_type
    }
}

impl Display for Voltage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.base, self.value.unwrap_or(f32::NAN))
    }
}

pub struct Current {
    base: BaseMetric,
    value: Option<f32>,
}

impl Current {
    pub fn new(register: u16, metric_name: &str, labels: Vec<KV<String, String>>) -> Self {
        let mut metric_name: String = metric_name.to_owned();
        metric_name.insert_str(0, METRIC_NAME_PREFIX);
        let base = BaseMetric::new(MetricType::Gauge, metric_name, labels, register);
        Self { base, value: None }
    }

    pub fn easy(register: u16, metric_name: &str, key: &str, value: &str) -> Box<dyn Metric> {
        Box::new(Self::new(
            register,
            metric_name,
            vec![KV {
                key: key.to_string(),
                value: value.to_string(),
            }],
        ))
    }
}

impl Metric for Current {
    fn read_data(&mut self, base_register: u16, data: &[u8]) -> Result<(), MetricReadError> {
        let value =
            get_register_bytes::<2>(&data, self.base.register as usize, base_register as usize)?;
        let value = i16::from_be_bytes(value);
        self.value = Some(value as f32 / 10.0);

        Ok(())
    }

    fn get_register(&self) -> u16 {
        self.base.get_register()
    }

    fn get_name(&self) -> String {
        self.base.metric_name.clone()
    }

    fn get_type(&self) -> MetricType {
        self.base.metric_type
    }
}

impl Display for Current {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.base, self.value.unwrap_or(f32::NAN))
    }
}

pub struct Power {
    base: BaseMetric,
    value: Option<i16>,
}

impl Power {
    pub fn new(register: u16, metric_name: &str, labels: Vec<KV<String, String>>) -> Self {
        let mut metric_name: String = metric_name.to_owned();
        metric_name.insert_str(0, METRIC_NAME_PREFIX);
        let base = BaseMetric::new(MetricType::Gauge, metric_name, labels, register);
        Self { base, value: None }
    }

    pub fn easy(register: u16, metric_name: &str, key: &str, value: &str) -> Box<dyn Metric> {
        Box::new(Self::new(
            register,
            metric_name,
            vec![KV {
                key: key.to_string(),
                value: value.to_string(),
            }],
        ))
    }
}

impl Metric for Power {
    fn read_data(&mut self, base_register: u16, data: &[u8]) -> Result<(), MetricReadError> {
        let value =
            get_register_bytes::<2>(&data, self.base.register as usize, base_register as usize)?;
        self.value = Some(i16::from_be_bytes(value));

        Ok(())
    }

    fn get_register(&self) -> u16 {
        self.base.get_register()
    }

    fn get_name(&self) -> String {
        self.base.metric_name.clone()
    }

    fn get_type(&self) -> MetricType {
        self.base.metric_type
    }
}

impl Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.base, self.value.unwrap_or(0))
    }
}

pub struct LargePower {
    base: BaseMetric,
    value: Option<i32>,
}

impl LargePower {
    pub fn new(register: u16, metric_name: &str, labels: Vec<KV<String, String>>) -> Self {
        let mut metric_name: String = metric_name.to_owned();
        metric_name.insert_str(0, METRIC_NAME_PREFIX);
        let base = BaseMetric::new(MetricType::Gauge, metric_name, labels, register);
        Self { base, value: None }
    }

    pub fn easy(register: u16, metric_name: &str, key: &str, value: &str) -> Box<dyn Metric> {
        Box::new(Self::new(
            register,
            metric_name,
            vec![KV {
                key: key.to_string(),
                value: value.to_string(),
            }],
        ))
    }
}

impl Metric for LargePower {
    fn read_data(&mut self, base_register: u16, data: &[u8]) -> Result<(), MetricReadError> {
        let value =
            get_register_bytes::<4>(&data, self.base.register as usize, base_register as usize)?;
        self.value = Some(i32::from_be_bytes(value));
        Ok(())
    }

    fn get_register(&self) -> u16 {
        self.base.get_register()
    }

    fn get_name(&self) -> String {
        self.base.metric_name.clone()
    }

    fn get_type(&self) -> MetricType {
        self.base.metric_type
    }
}

impl Display for LargePower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.base, self.value.unwrap_or(0))
    }
}

pub struct Frequency {
    base: BaseMetric,
    value: Option<f32>,
}

impl Frequency {
    pub fn new(register: u16, metric_name: &str, labels: Vec<KV<String, String>>) -> Self {
        let mut metric_name: String = metric_name.to_owned();
        metric_name.insert_str(0, METRIC_NAME_PREFIX);
        let base = BaseMetric::new(MetricType::Gauge, metric_name, labels, register);
        Self { base, value: None }
    }

    pub fn easy(register: u16, metric_name: &str, key: &str, value: &str) -> Box<dyn Metric> {
        Box::new(Self::new(
            register,
            metric_name,
            vec![KV {
                key: key.to_string(),
                value: value.to_string(),
            }],
        ))
    }
}

impl Metric for Frequency {
    fn read_data(&mut self, base_register: u16, data: &[u8]) -> Result<(), MetricReadError> {
        let value =
            get_register_bytes::<2>(&data, self.base.register as usize, base_register as usize)?;
        let value = i16::from_be_bytes(value);
        self.value = Some(value as f32 / 100.0);

        Ok(())
    }

    fn get_register(&self) -> u16 {
        self.base.get_register()
    }

    fn get_name(&self) -> String {
        self.base.metric_name.clone()
    }

    fn get_type(&self) -> MetricType {
        self.base.metric_type
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.base, self.value.unwrap_or(f32::NAN))
    }
}

pub struct Percentage {
    base: BaseMetric,
    value: Option<u16>,
}

impl Percentage {
    pub fn new(register: u16, metric_name: &str, labels: Vec<KV<String, String>>) -> Self {
        let mut metric_name = metric_name.to_owned();
        metric_name.insert_str(0, METRIC_NAME_PREFIX);
        let base = BaseMetric::new(MetricType::Gauge, metric_name, labels, register);
        Self { base, value: None }
    }

    pub fn easy(register: u16, metric_name: &str, key: &str, value: &str) -> Box<dyn Metric> {
        Box::new(Self::new(
            register,
            metric_name,
            vec![KV {
                key: key.to_string(),
                value: value.to_string(),
            }],
        ))
    }
}

impl Metric for Percentage {
    fn read_data(&mut self, base_register: u16, data: &[u8]) -> Result<(), MetricReadError> {
        let value =
            get_register_bytes::<2>(&data, self.base.register as usize, base_register as usize)?;
        self.value = Some(u16::from_be_bytes(value));

        Ok(())
    }

    fn get_register(&self) -> u16 {
        self.base.get_register()
    }

    fn get_name(&self) -> String {
        self.base.metric_name.clone()
    }

    fn get_type(&self) -> MetricType {
        self.base.metric_type
    }
}

impl Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.base, self.value.unwrap_or(u16::MIN))
    }
}

pub struct Temperature {
    base: BaseMetric,
    value: Option<f32>,
}

impl Temperature {
    pub fn new(register: u16, metric_name: &str, labels: Vec<KV<String, String>>) -> Self {
        let mut metric_name: String = metric_name.to_owned();
        metric_name.insert_str(0, METRIC_NAME_PREFIX);
        let base = BaseMetric::new(MetricType::Gauge, metric_name, labels, register);
        Self { base, value: None }
    }

    pub fn easy(register: u16, metric_name: &str, key: &str, value: &str) -> Box<dyn Metric> {
        Box::new(Self::new(
            register,
            metric_name,
            vec![KV {
                key: key.to_string(),
                value: value.to_string(),
            }],
        ))
    }
}

impl Metric for Temperature {
    fn read_data(&mut self, base_register: u16, data: &[u8]) -> Result<(), MetricReadError> {
        let value =
            get_register_bytes::<2>(&data, self.base.register as usize, base_register as usize)?;
        let value = i16::from_be_bytes(value);
        self.value = Some(value as f32 / 10.0);

        Ok(())
    }

    fn get_register(&self) -> u16 {
        self.base.get_register()
    }

    fn get_name(&self) -> String {
        self.base.metric_name.clone()
    }

    fn get_type(&self) -> MetricType {
        self.base.metric_type
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.base, self.value.unwrap_or(f32::NAN))
    }
}

pub struct Energy {
    base: BaseMetric,
    value: Option<f32>,
}

impl Energy {
    pub fn new(register: u16, metric_name: &str, labels: Vec<KV<String, String>>) -> Self {
        let mut metric_name = metric_name.to_owned();
        metric_name.insert_str(0, METRIC_NAME_PREFIX);
        let base = BaseMetric::new(MetricType::Counter, metric_name, labels, register);
        Self { base, value: None }
    }

    pub fn easy(register: u16, metric_name: &str, key: &str, value: &str) -> Box<dyn Metric> {
        Box::new(Self::new(
            register,
            metric_name,
            vec![KV {
                key: key.to_string(),
                value: value.to_string(),
            }],
        ))
    }
}

impl Metric for Energy {
    fn read_data(&mut self, base_register: u16, data: &[u8]) -> Result<(), MetricReadError> {
        let value =
            get_register_bytes::<2>(&data, self.base.register as usize, base_register as usize)?;
        let value = i16::from_be_bytes(value);
        self.value = Some(value as f32 / 10.0);

        Ok(())
    }

    fn get_register(&self) -> u16 {
        self.base.get_register()
    }

    fn get_name(&self) -> String {
        self.base.metric_name.clone()
    }

    fn get_type(&self) -> MetricType {
        self.base.metric_type
    }
}

impl Display for Energy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.base, self.value.unwrap_or(f32::NAN))
    }
}

pub struct LargeEnergy {
    base: BaseMetric,
    value: Option<f32>,
}

impl LargeEnergy {
    pub fn new(register: u16, metric_name: &str, labels: Vec<KV<String, String>>) -> Self {
        let mut metric_name: String = metric_name.to_owned();
        metric_name.insert_str(0, METRIC_NAME_PREFIX);
        let base = BaseMetric::new(MetricType::Counter, metric_name, labels, register);
        Self { base, value: None }
    }

    pub fn easy(register: u16, metric_name: &str, key: &str, value: &str) -> Box<dyn Metric> {
        Box::new(Self::new(
            register,
            metric_name,
            vec![KV {
                key: key.to_string(),
                value: value.to_string(),
            }],
        ))
    }
}

impl Metric for LargeEnergy {
    fn read_data(&mut self, base_register: u16, data: &[u8]) -> Result<(), MetricReadError> {
        let value =
            get_register_bytes::<4>(&data, self.base.register as usize, base_register as usize)?;
        let value = i32::from_be_bytes(value);
        self.value = Some(value as f32 / 10.0);

        Ok(())
    }

    fn get_register(&self) -> u16 {
        self.base.get_register()
    }

    fn get_name(&self) -> String {
        self.base.metric_name.clone()
    }

    fn get_type(&self) -> MetricType {
        self.base.metric_type
    }
}

impl Display for LargeEnergy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.base, self.value.unwrap_or(f32::NAN))
    }
}

pub struct Integer {
    base: BaseMetric,
    value: Option<i16>,
}

impl Integer {
    pub fn new(register: u16, metric_name: &str, labels: Vec<KV<String, String>>) -> Self {
        let mut metric_name: String = metric_name.to_owned();
        metric_name.insert_str(0, METRIC_NAME_PREFIX);
        let base = BaseMetric::new(MetricType::Counter, metric_name, labels, register);
        Self { base, value: None }
    }

    pub fn easy(register: u16, metric_name: &str, key: &str, value: &str) -> Box<dyn Metric> {
        Box::new(Self::new(
            register,
            metric_name,
            vec![KV {
                key: key.to_string(),
                value: value.to_string(),
            }],
        ))
    }
}

impl Metric for Integer {
    fn read_data(&mut self, base_register: u16, data: &[u8]) -> Result<(), MetricReadError> {
        let value =
            get_register_bytes::<2>(&data, self.base.register as usize, base_register as usize)?;
        self.value = Some(i16::from_be_bytes(value));

        Ok(())
    }

    fn get_register(&self) -> u16 {
        self.base.get_register()
    }

    fn get_name(&self) -> String {
        self.base.metric_name.clone()
    }

    fn get_type(&self) -> MetricType {
        self.base.metric_type
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.base, self.value.unwrap_or(i16::MIN))
    }
}
