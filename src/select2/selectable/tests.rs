#[test]

fn selectable() {

    use crate::select2::{Record, Request, Selectable, selectable_results};
    use std::clone::Clone;
    use std::cmp::{Eq, PartialEq};
    use std::fmt::Debug;
    use std::hash::Hash;
    use std::string::ToString;

    struct Province {
        id: String,
        text: String,
    } // Province

    let provinces = vec![
        Province { id: "AB".to_string(), text: "Alberta".to_string() },
        Province { id: "BC".to_string(), text: "British Columbia".to_string() },
        Province { id: "MB".to_string(), text: "Manitoba".to_string() },
        Province { id: "NB".to_string(), text: "New Brunswick".to_string() },
        Province { id: "NL".to_string(), text: "Newfoundland and Labrador".to_string() },
        Province { id: "NS".to_string(), text: "Nova Scotia".to_string() },
        Province { id: "NT".to_string(), text: "Northwest Territories".to_string() },
        Province { id: "NU".to_string(), text: "Nunavut".to_string() },
        Province { id: "ON".to_string(), text: "Ontario".to_string() },
        Province { id: "PE".to_string(), text: "Prince Edward Island".to_string() },
        Province { id: "QC".to_string(), text: "Québec".to_string() },
        Province { id: "SK".to_string(), text: "Saskatchewan".to_string() },
        Province { id: "YT".to_string(), text: "Yukon".to_string() },
    ]; // vec!

    impl<K: Clone + Debug + Eq + Hash + PartialEq + ToString> Selectable<K> for Province {
        fn select2_record(&self) -> Record {
            Record {
                id: self.id.clone(),
                text: self.text.clone(),
                selected: false,
                disabled: false,
            } // Record
        } // fn
    } // impl

    // Test returned records, `selected` field, and pagination. Results for
    // Page #2 with two items per page, and "MB" Manitoba selected:

    let select2_results = selectable_results::<String, Province>(
        &Request {
            term: None,
            q: None,
            request_type: None,
            page: Some(2),
        }, // Request
        &Some(2),                   // Pagination items per page
        &provinces,                 // Vec<Selectable>
        &Some("MB".to_string()),    // `id` of selected record (if any)
    ); // selectable_results

    assert_eq!(select2_results.results[0], Record {
            id: "MB".to_string(),
            text: "Manitoba".to_string(),
            selected: true,
            disabled: false,
        }
    ); // assert_eq!

    assert!(select2_results.pagination.more);

    // Test pagination. Results for Page #7 with two items per page:

    let select2_results = selectable_results::<String, Province>(
        &Request {
            term: None,
            q: None,
            request_type: None,
            page: Some(7),
        }, // Request
        &Some(2),                   // Pagination items per page
        &provinces,                 // Vec<Selectable>
        &None,                      // `id` of selected record (if any)
    ); // selectable_results

    assert!(!select2_results.pagination.more);

} // fn