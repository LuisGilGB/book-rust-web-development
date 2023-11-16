use std::collections::HashMap;

use errors::Error;

/// Pagination struct extracted from the query parameters
#[derive(Default, Debug)]
pub struct Pagination {
    /// The index where the page starts
    pub offset: Option<u32>,
    /// The size of the page
    pub limit: Option<u32>,
}

fn cap_number(max: u32) -> impl Fn(u32) -> u32 {
    move |x| {
        if x > max {
            max
        } else {
            x
        }
    }
}

/// Extracts the pagination parameters from the query parameters
///
/// # Example usage
/// ```rust
/// use std::collections::HashMap;
/// use qa_web_app::infrastructure::pagination::{extract_pagination, Pagination};
///
/// let mut params = HashMap::new();
/// params.insert("offset".to_string(), "0".to_string());
/// params.insert("limit".to_string(), "10".to_string());
/// let pagination = extract_pagination(params, 100).unwrap();
/// assert_eq!(pagination.offset, Some(0));
/// assert_eq!(pagination.limit, Some(10));
/// ```
pub fn extract_pagination(
    params: HashMap<String, String>
) -> Result<Pagination, Error> {
    if params.contains_key("offset") && params.contains_key("limit") {
        let offset = params
            .get("offset")
            .unwrap()
            .parse::<u32>()
            .map_err(Error::ParseError)?;
        let limit = params
            .get("limit")
            .unwrap()
            .parse::<u32>()
            .map_err(Error::ParseError)?;
        if offset > limit {
            return Err(Error::StartGreaterThanEnd);
        }
        return Ok(Pagination {
            offset: Some(offset),
            limit: Some(limit),
        });
    }
    Err(Error::MissingParameters)
}
