// SPDX-License-Identifier: MIT
// SigmaOS - Contact Manager
// Inspired by GNOME Contacts and KDE Kontact
// Provides contact management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Phone number type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhoneType {
    Home,
    Work,
    Mobile,
    Other,
}

/// Email type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailType {
    Home,
    Work,
    Other,
}

/// Address type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    Home,
    Work,
    Other,
}

/// Phone number
#[derive(Debug, Clone)]
pub struct PhoneNumber {
    pub number: String,
    pub phone_type: PhoneType,
}

/// Email address
#[derive(Debug, Clone)]
pub struct EmailAddress {
    pub email: String,
    pub email_type: EmailType,
}

/// Address
#[derive(Debug, Clone)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub address_type: AddressType,
}

/// Contact
#[derive(Debug, Clone)]
pub struct Contact {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub company: String,
    pub job_title: String,
    pub phone_numbers: Vec<PhoneNumber>,
    pub email_addresses: Vec<EmailAddress>,
    pub addresses: Vec<Address>,
    pub notes: String,
    pub avatar: Option<String>,
    pub favorite: bool,
    pub created: String,
    pub modified: String,
}

/// Contact group
#[derive(Debug, Clone)]
pub struct ContactGroup {
    pub id: String,
    pub name: String,
    pub contact_ids: Vec<String>,
}

/// Contact manager
pub struct ContactManager {
    pub contacts: BTreeMap<String, Contact>,
    pub groups: BTreeMap<String, ContactGroup>,
    pub filter: String,
}

impl ContactManager {
    pub fn new() -> Self {
        Self {
            contacts: BTreeMap::new(),
            groups: BTreeMap::new(),
            filter: String::new(),
        }
    }

    /// Add contact
    pub fn add_contact(&mut self, contact: Contact) {
        self.contacts.insert(contact.id.clone(), contact);
    }

    /// Remove contact
    pub fn remove_contact(&mut self, id: String) -> Result<(), &'static str> {
        if !self.contacts.contains_key(&id) {
            return Err("Contact not found");
        }
        self.contacts.remove(&id);
        Ok(())
    }

    /// Update contact
    pub fn update_contact(&mut self, contact: Contact) -> Result<(), &'static str> {
        if !self.contacts.contains_key(&contact.id) {
            return Err("Contact not found");
        }
        self.contacts.insert(contact.id.clone(), contact);
        Ok(())
    }

    /// Get contact
    pub fn get_contact(&self, id: &str) -> Option<&Contact> {
        self.contacts.get(id)
    }

    /// Set filter
    pub fn set_filter(&mut self, filter: String) {
        self.filter = filter;
    }

    /// Get filtered contacts
    pub fn get_filtered_contacts(&self) -> Vec<&Contact> {
        if self.filter.is_empty() {
            self.contacts.values().collect()
        } else {
            let filter_lower = self.filter.to_lowercase();
            self.contacts
                .values()
                .filter(|c| {
                    c.first_name.to_lowercase().contains(&filter_lower)
                        || c.last_name.to_lowercase().contains(&filter_lower)
                        || c.email_addresses
                            .iter()
                            .any(|e| e.email.to_lowercase().contains(&filter_lower))
                        || c.phone_numbers
                            .iter()
                            .any(|p| p.number.contains(&filter_lower))
                })
                .collect()
        }
    }

    /// Toggle favorite
    pub fn toggle_favorite(&mut self, id: String) -> Result<(), &'static str> {
        let contact = self.contacts.get_mut(&id).ok_or("Contact not found")?;
        contact.favorite = !contact.favorite;
        Ok(())
    }

    /// Add group
    pub fn add_group(&mut self, group: ContactGroup) {
        self.groups.insert(group.id.clone(), group);
    }

    /// Remove group
    pub fn remove_group(&mut self, id: String) -> Result<(), &'static str> {
        if !self.groups.contains_key(&id) {
            return Err("Group not found");
        }
        self.groups.remove(&id);
        Ok(())
    }

    /// Add contact to group
    pub fn add_to_group(
        &mut self,
        group_id: String,
        contact_id: String,
    ) -> Result<(), &'static str> {
        let group = self.groups.get_mut(&group_id).ok_or("Group not found")?;
        if !group.contact_ids.contains(&contact_id) {
            group.contact_ids.push(contact_id);
        }
        Ok(())
    }

    /// Remove contact from group
    pub fn remove_from_group(
        &mut self,
        group_id: String,
        contact_id: String,
    ) -> Result<(), &'static str> {
        let group = self.groups.get_mut(&group_id).ok_or("Group not found")?;
        let index = group
            .contact_ids
            .iter()
            .position(|id| id == &contact_id)
            .ok_or("Contact not in group")?;
        group.contact_ids.remove(index);
        Ok(())
    }

    /// Get contacts in group
    pub fn get_contacts_in_group(&self, group_id: &str) -> Vec<&Contact> {
        self.groups
            .get(group_id)
            .map(|g| {
                g.contact_ids
                    .iter()
                    .filter_map(|id| self.contacts.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all contacts
    pub fn get_all_contacts(&self) -> Vec<&Contact> {
        self.contacts.values().collect()
    }

    /// Get all groups
    pub fn get_all_groups(&self) -> Vec<&ContactGroup> {
        self.groups.values().collect()
    }

    /// Contact count
    pub fn contact_count(&self) -> usize {
        self.contacts.len()
    }

    /// Group count
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }
}

/// Global contact manager
static mut CONTACT_MANAGER: Option<ContactManager> = None;

/// Initialize contact manager
pub fn init_contact_manager() {
    unsafe {
        CONTACT_MANAGER = Some(ContactManager::new());
    }
}

/// Get contact manager
pub fn get_contact_manager() -> &'static mut ContactManager {
    unsafe {
        CONTACT_MANAGER
            .as_mut()
            .expect("Contact manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_manager_creation() {
        let cm = ContactManager::new();
        assert_eq!(cm.contact_count(), 0);
    }

    #[test]
    fn test_add_contact() {
        let mut cm = ContactManager::new();
        let contact = Contact {
            id: "1".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            nickname: String::new(),
            company: String::new(),
            job_title: String::new(),
            phone_numbers: Vec::new(),
            email_addresses: Vec::new(),
            addresses: Vec::new(),
            notes: String::new(),
            avatar: None,
            favorite: false,
            created: String::new(),
            modified: String::new(),
        };
        cm.add_contact(contact);
        assert_eq!(cm.contact_count(), 1);
    }

    #[test]
    fn test_filter() {
        let mut cm = ContactManager::new();
        let contact = Contact {
            id: "1".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            nickname: String::new(),
            company: String::new(),
            job_title: String::new(),
            phone_numbers: Vec::new(),
            email_addresses: Vec::new(),
            addresses: Vec::new(),
            notes: String::new(),
            avatar: None,
            favorite: false,
            created: String::new(),
            modified: String::new(),
        };
        cm.add_contact(contact);
        cm.set_filter("john".to_string());
        let filtered = cm.get_filtered_contacts();
        assert_eq!(filtered.len(), 1);
    }
}
