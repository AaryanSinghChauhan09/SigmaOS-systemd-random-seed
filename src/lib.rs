// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-random-seed: systemd random seed component model.
//!
//! Real implementation: seeds, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seed {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemdRandomSeedError {
    EmptySeedName,
    DuplicateSeed(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdRandomSeedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySeedName => write!(f, "seed name must not be empty"),
            Self::DuplicateSeed(id) => write!(f, "duplicate seed {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdRandomSeedError {}

pub struct SystemdRandomSeed {
    pub name: String,
    seeds: BTreeMap<String, Seed>,
    events: BTreeMap<String, Event>,
}

impl SystemdRandomSeed {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            seeds: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_seed(&mut self, item: Seed) -> Result<(), SystemdRandomSeedError> {
        if item.name.trim().is_empty() {
            return Err(SystemdRandomSeedError::EmptySeedName);
        }
        let id = item.id.clone();
        if self.seeds.contains_key(&id) {
            return Err(SystemdRandomSeedError::DuplicateSeed(id.clone()));
        }
        self.seeds.insert(id, item);
        Ok(())
    }

    pub fn seed_count(&self) -> usize {
        self.seeds.len()
    }

    pub fn get_seed(&self, id: &str) -> Option<&Seed> {
        self.seeds.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdRandomSeedError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdRandomSeedError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_seed(id: &str, name: &str) -> Seed {
        Seed {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdRandomSeed::new("SystemdRandomSeed");
        assert_eq!(m.name, "SystemdRandomSeed");
        assert_eq!(m.seed_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_seed() {
        let mut m = SystemdRandomSeed::new("SystemdRandomSeed");
        m.add_seed(sample_seed("a1", "Alpha")).unwrap();
        assert_eq!(m.seed_count(), 1);
        assert!(m.get_seed("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdRandomSeed::new("SystemdRandomSeed");
        assert!(m.add_seed(sample_seed("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdRandomSeed::new("SystemdRandomSeed");
        m.add_seed(sample_seed("x", "One")).unwrap();
        assert!(m.add_seed(sample_seed("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdRandomSeed::new("SystemdRandomSeed");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
