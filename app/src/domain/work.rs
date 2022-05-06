use crate::csv_loader::{parse, CSVParser};
use crate::AppResult;
use csv::StringRecord;
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::Display as StrumDisplay;
use strum_macros::EnumString;

#[derive(PartialEq, Clone, Debug, StrumDisplay, EnumString)]
pub enum WorkStatus {
    Prepare,
    PublishNFT,
    SellOrder,
}

impl WorkStatus {
    pub fn from(str: String) -> Self {
        WorkStatus::from_str(&str).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Work {
    pub id: String,
    pub video_path: String,
    pub status: WorkStatus,
}

impl Work {
    pub fn update_status(&mut self, status: WorkStatus) -> AppResult<()> {
        self.status = status;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Thumbnail {
    pub id: String,
    pub work_id: String,
    pub image_path: String,
    pub order: i32,
}

impl CSVParser for Work {
    fn from(header: HashMap<&str, usize>, record: StringRecord) -> AppResult<Box<Self>> {
        let id = parse(&header, &record, "ID")?;
        let video_path = parse(&header, &record, "VideoPath")?;

        let data = Self {
            id,
            video_path,
            status: WorkStatus::Prepare,
        };
        return Ok(Box::new(data));
    }
}

impl CSVParser for Thumbnail {
    fn from(header: HashMap<&str, usize>, record: StringRecord) -> AppResult<Box<Self>> {
        let id = parse(&header, &record, "ID")?;
        let work_id = parse(&header, &record, "WorkID")?;
        let image_path = parse(&header, &record, "ImagePath")?;
        let order = parse(&header, &record, "Order")?;

        let data = Self {
            id,
            work_id,
            image_path,
            order: order.parse().unwrap(),
        };
        return Ok(Box::new(data));
    }
}
