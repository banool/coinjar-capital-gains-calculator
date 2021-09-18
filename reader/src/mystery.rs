use crate::traits::Reader;
use anyhow::Result;
use chrono::Utc;
use std::path::PathBuf;
use types::Transaction;

pub struct MysteryReader {}

impl Reader for MysteryReader {
    fn read_transactions(&self, path: &PathBuf) -> Result<Vec<Transaction>> {
        Ok(Vec::new())
    }
}