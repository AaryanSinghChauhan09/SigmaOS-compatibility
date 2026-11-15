//! SigmaOS-compatibility: compatibility component model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for compatibility component.
#[derive(Debug, Clone, PartialEq)]
pub struct CompatibilityItem {
    pub id: String,
    pub name: String,
    pub item_type: String,
    pub enabled: bool,
}

/// Secondary domain entity for compatibility component.
#[derive(Debug, Clone, PartialEq)]
pub struct CompatibilityConfig {
    pub id: String,
    pub value: String,
    pub description: String,
}

/// Errors for the compatibility component domain.
#[derive(Debug, PartialEq)]
pub enum CompatibilityError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for CompatibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for CompatibilityError {}

/// Registry and manager for compatibility component.
pub struct CompatibilityManager {
    pub name: String,
    items: BTreeMap<String, CompatibilityItem>,
    configs: BTreeMap<String, CompatibilityConfig>,
}

impl CompatibilityManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: CompatibilityItem) -> Result<(), CompatibilityError> {
        if item.name.is_empty() {
            return Err(CompatibilityError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(CompatibilityError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&CompatibilityItem> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: CompatibilityConfig) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), CompatibilityError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| CompatibilityError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_compatibility_item(id: &str, name: &str) -> CompatibilityItem {
        CompatibilityItem {
            id: id.to_string(),
            name: name.to_string(),
            item_type: "compatibility-default".to_string(),
            enabled: true,
        }
    }

    fn make_compatibility_config(id: &str, val: &str) -> CompatibilityConfig {
        CompatibilityConfig {
            id: id.to_string(),
            value: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = CompatibilityManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = CompatibilityManager::new("test");
        let item = make_compatibility_item("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = CompatibilityManager::new("test");
        let item = make_compatibility_item("item1", "");
        assert_eq!(mgr.add_item(item), Err(CompatibilityError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = CompatibilityManager::new("test");
        let item1 = make_compatibility_item("item1", "First");
        let item2 = make_compatibility_item("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(CompatibilityError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = CompatibilityManager::new("test");
        let cfg = make_compatibility_config("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = CompatibilityManager::new("test");
        let item = make_compatibility_item("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = CompatibilityManager::new("test");
        let item = make_compatibility_item("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(CompatibilityError::NotFound("none".to_string())));
    }
}
