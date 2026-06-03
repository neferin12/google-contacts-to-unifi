use std::collections::HashMap;
use crate::models::UnifiContact;

pub fn map_google_to_unifi(record: &HashMap<String, String>) -> Option<UnifiContact> {
    let last_name = record.get("Last Name").cloned().unwrap_or_default();
    
    if last_name.trim().is_empty() {
        let first_name = record.get("First Name").cloned().unwrap_or_default();
        let name_to_print = if first_name.trim().is_empty() { "Unknown" } else { &first_name };
        eprintln!("Warning: Skipping contact '{}' because they have no last name.", name_to_print);
        return None;
    }

    let mut contact = UnifiContact {
        first_name: record.get("First Name").cloned().unwrap_or_default(),
        last_name,
        company: record.get("Organization Name").cloned().unwrap_or_default(),
        job_title: record.get("Organization Title").cloned().unwrap_or_default(),
        email: record.get("E-mail 1 - Value").cloned().unwrap_or_default(),
        ..Default::default()
    };

    // Find phone numbers
    for i in 1..=10 { // Usually there are a few phone numbers, up to maybe 5 or 10
        let type_key = format!("Phone {} - Label", i);
        let val_key = format!("Phone {} - Value", i);
        
        if let (Some(t), Some(v)) = (record.get(&type_key), record.get(&val_key)) {
            if v.is_empty() {
                continue;
            }
            
            let t_lower = t.to_lowercase();
            
            for part in v.split(":::") {
                let clean_v = part.trim().to_string();
                if clean_v.is_empty() {
                    continue;
                }

                if t_lower.contains("mobile") && contact.mobile_number.is_empty() {
                    contact.mobile_number = clean_v;
                } else if t_lower.contains("home") && contact.home_number.is_empty() {
                    contact.home_number = clean_v;
                } else if (t_lower.contains("work") || t_lower.contains("company")) && contact.work_number.is_empty() {
                    contact.work_number = clean_v;
                } else if t_lower.contains("fax") && contact.fax_number.is_empty() {
                    contact.fax_number = clean_v;
                } else if contact.other_number.is_empty() {
                    contact.other_number = clean_v;
                }
            }
        }
    }

    Some(contact)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_map_google_to_unifi() {
        let mut record = HashMap::new();
        record.insert("First Name".to_string(), "John".to_string());
        record.insert("Last Name".to_string(), "Doe".to_string());
        record.insert("Organization Name".to_string(), "Acme Corp".to_string());
        record.insert("Organization Title".to_string(), "Engineer".to_string());
        record.insert("E-mail 1 - Value".to_string(), "john@example.com".to_string());
        
        record.insert("Phone 1 - Label".to_string(), "Mobile".to_string());
        record.insert("Phone 1 - Value".to_string(), "+123456789:::+111222333".to_string());
        
        record.insert("Phone 2 - Label".to_string(), "Work".to_string());
        record.insert("Phone 2 - Value".to_string(), "+987654321".to_string());
        
        let contact = map_google_to_unifi(&record).expect("Contact should not be None");
        
        assert_eq!(contact.first_name, "John");
        assert_eq!(contact.last_name, "Doe");
        assert_eq!(contact.company, "Acme Corp");
        assert_eq!(contact.job_title, "Engineer");
        assert_eq!(contact.email, "john@example.com");
        assert_eq!(contact.mobile_number, "+123456789");
        assert_eq!(contact.work_number, "+987654321");
        assert_eq!(contact.home_number, "");
        assert_eq!(contact.fax_number, "");
        assert_eq!(contact.other_number, "+111222333");
    }
    
    #[test]
    fn test_empty_record() {
        let record = HashMap::new();
        let contact_opt = map_google_to_unifi(&record);
        assert!(contact_opt.is_none());
    }
}
