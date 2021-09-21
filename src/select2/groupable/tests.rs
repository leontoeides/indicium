#[test]

fn selectable() {

    use crate::select2::{Request, Groupable, GroupRecord, groupable_results};
    use std::clone::Clone;
    use std::cmp::{Eq, PartialEq};
    use std::fmt::Debug;
    use std::hash::Hash;
    use std::string::ToString;

    enum Category {
        Province,
        Terrority,
    } // Category

    impl std::fmt::Display for Category {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Category::Province => write!(f, "Provinces"),
                Category::Terrority => write!(f, "Territories"),
            } // match
        } // fn
    } // impl

    struct Province {
        id: String,
        category: Category,
        text: String,
    } // Province

    let provinces = vec![
        Province { id: "AB".to_string(), category: Category::Province, text: "Alberta".to_string() },
        Province { id: "BC".to_string(), category: Category::Province, text: "British Columbia".to_string() },
        Province { id: "MB".to_string(), category: Category::Province, text: "Manitoba".to_string() },
        Province { id: "NB".to_string(), category: Category::Province, text: "New Brunswick".to_string() },
        Province { id: "NL".to_string(), category: Category::Province, text: "Newfoundland and Labrador".to_string() },
        Province { id: "NS".to_string(), category: Category::Province, text: "Nova Scotia".to_string() },
        Province { id: "NT".to_string(), category: Category::Terrority, text: "Northwest Territories".to_string() },
        Province { id: "NU".to_string(), category: Category::Province, text: "Nunavut".to_string() },
        Province { id: "ON".to_string(), category: Category::Province, text: "Ontario".to_string() },
        Province { id: "PE".to_string(), category: Category::Province, text: "Prince Edward Island".to_string() },
        Province { id: "QC".to_string(), category: Category::Province, text: "Québec".to_string() },
        Province { id: "SK".to_string(), category: Category::Province, text: "Saskatchewan".to_string() },
        Province { id: "YT".to_string(), category: Category::Terrority, text: "Yukon".to_string() },
    ]; // vec!

    impl<K: Clone + Debug + Eq + Hash + PartialEq + ToString> Groupable<K> for Province {
        fn select2_grouped_record(&self) -> GroupRecord {
            GroupRecord {
                id: self.id.clone(),
                group: self.category.to_string(),
                text: self.text.clone(),
                selected: false,
                disabled: false,
            } // GroupRecord
        } // fn
    } // impl

    // Test returned records, `selected` field, and pagination. Results for
    // Page #2 with two items per page, and "MB" Manitoba selected:

    let select2_results = groupable_results::<String, Province>(
        &Request {
            term: None,
            q: None,
            request_type: None,
            page: Some(2),
        }, // Request
        &None,                      // Pagination items per page
        &provinces,                 // Vec<Groupable>
        &Some("MB".to_string()),    // `id` of selected record (if any)
    ); // groupable_results

    println!("Results: {:#?}", select2_results);

    assert_eq!(select2_results.results[0].text, "Provinces".to_string());



    //assert_eq!(true, false);



/*
    assert_eq!(select2_results.results[0], Record {
            id: "MB".to_string(),
            text: "Manitoba".to_string(),
            selected: true,
            disabled: false,
        }
    ); // assert_eq!

    assert!(select2_results.pagination.more);

    // Test pagination. Results for Page #7 with two items per page:

    let select2_results = groupable_results::<String, Province>(
        &Request {
            term: None,
            q: None,
            request_type: None,
            page: Some(7),
        }, // Request
        &Some(2),                   // Pagination items per page
        &provinces,                 // Vec<Groupable>
        &None,                      // `id` of selected record (if any)
    ); // groupable_results

    assert!(!select2_results.pagination.more); */

} // fn