use std::collections::HashMap;

use errors::Error;

/// Pagination struct extracted from the query parameters
#[derive(Debug)]
pub struct Pagination {
    /// The index where the page starts
    pub start: usize,
    /// The index where the page ends
    pub end: usize,
}

fn cap_number(max: usize) -> impl Fn(usize) -> usize {
    move |x| {
        if x > max {
            max
        } else {
            x
        }
    }
}

/// Extracts the pagination parameters from the query parameters
pub fn extract_pagination(params: HashMap<String, String>, total_length: usize) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        let start = params
            .get("start")
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseError)?;
        let end = params
            .get("end")
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseError)?;
        if start > end {
            return Err(Error::StartGreaterThanEnd);
        }
        return Ok(Pagination {
            start: cap_number(total_length)(start),
            end: cap_number(total_length)(end),
        });
    }
    Err(Error::MissingParameters)
}
