use anyhow::Result;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

use oci_spec::runtime::Spec;
use once_cell::sync::OnceCell;

use crate::cache::{new_cache, Cache, CacheOption, WithAutoRefresh};

fn get_or_create_default_cache(_options: Vec<Arc<dyn CacheOption>>) -> Arc<Mutex<Cache>> {
    let mut cache: OnceCell<Arc<Mutex<Cache>>> = OnceCell::new();
    cache.get_or_init(|| {
        let options: Vec<Arc<dyn CacheOption>> = vec![Arc::new(WithAutoRefresh(true))];
        new_cache(options)
    });
    cache.take().unwrap()
}

pub fn get_default_cache() -> Arc<Mutex<Cache>> {
    get_or_create_default_cache(vec![])
}

pub fn configure(options: Vec<Arc<dyn CacheOption>>) -> Result<()> {
    let cache = get_or_create_default_cache(options.clone());
    let mut cache = cache.lock().unwrap();
    if options.is_empty() {
        return Ok(());
    }
    cache.configure(options);
    Ok(())
}

pub fn refresh() -> Result<(), Box<dyn Error>> {
    let cache = get_default_cache();
    let mut cache = cache.lock().unwrap();
    cache.refresh()
}

pub fn inject_devices(
    oci_spec: &mut Spec,
    devices: Vec<String>,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync + 'static>> {
    let cache = get_default_cache();
    let mut cache = cache.lock().unwrap();
    cache.inject_devices(Some(oci_spec), devices)
}

pub fn list_devices() -> Vec<String> {
    let cache = get_default_cache();
    let mut cache = cache.lock().unwrap();
    cache.list_devices()
}

pub fn get_errors() -> HashMap<String, Vec<anyhow::Error>> {
    let cache = get_default_cache();
    let cache = cache.lock().unwrap();
    cache.get_errors()
}
