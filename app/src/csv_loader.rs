use crate::{AppError, AppResult};
use bytes::{Buf, Bytes};
use csv::StringRecord;
use std::collections::HashMap;

pub fn load_from_csv<T: CSVParser>(
    data: Bytes,
    limit: Option<i32>,
) -> AppResult<Vec<T>> {
    let mut rdr = csv::ReaderBuilder::new()
        // .delimiter(b'\t')
        .from_reader(data.reader());

    let mut header_map: HashMap<&str, usize> = HashMap::new();
    let header = rdr.headers()?.clone();
    for i in 0..header.len() {
        let key = &header[i];
        header_map.insert(key, i);
    }

    let mut results: Vec<T> = vec![];
    for record in rdr.records() {
        let record = record?;

        let r = T::from(header_map.to_owned(), record)?;
        results.push(*r);

        if let Some(limit) = limit {
            if results.len() as i32 >= limit {
                break;
            }
        }
    }

    Ok(results)
}

pub trait CSVParser {
    fn from(
        header: HashMap<&str, usize>,
        record: StringRecord,
    ) -> AppResult<Box<Self>>;
}

pub fn parse(header: &HashMap<&str, usize>, record: &StringRecord, key: &str) -> AppResult<String> {
    let res = header.get(key).ok_or(AppError::BadRequest(format!(
        "CSVのパースに失敗しました。{}が存在しません。",
        key
    )))?;
    Ok(String::from(record[res.clone()].trim()))
}
