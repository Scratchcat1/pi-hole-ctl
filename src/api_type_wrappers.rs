// Wrappers for API responses which return a "generic" datatype e.g. u64 or HashMap<String, u64>
// Allows traits such as ToTable to target the types correctly
use crate::table::{ToTableRows, ToTableTitle};
use cli_table::{Cell, CellStruct};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct OverTimeDataClientsWrapper(pub HashMap<String, Vec<u64>>);

impl ToTableRows for OverTimeDataClientsWrapper {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        self.0
            .into_iter()
            .flat_map(|(time, counts)| {
                counts
                    .into_iter()
                    .map(|count| vec![host.cell(), time.to_owned().cell(), count.cell()])
                    .collect::<Vec<Vec<CellStruct>>>()
            })
            .collect()
    }
}

impl ToTableTitle for OverTimeDataClientsWrapper {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "time".cell(), "count".cell()]
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct QueriesCountWrapper(pub u64);

impl ToTableRows for QueriesCountWrapper {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![host.cell(), self.0.cell()]]
    }
}

impl ToTableTitle for QueriesCountWrapper {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "count".cell()]
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct VersionWrapper(pub u32);

impl ToTableRows for VersionWrapper {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![host.cell(), self.0.cell()]]
    }
}

impl ToTableTitle for VersionWrapper {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "version".cell()]
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LogageWrapper(pub f32);

impl ToTableRows for LogageWrapper {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![host.cell(), self.0.cell()]]
    }
}

impl ToTableTitle for LogageWrapper {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "Logage".cell()]
    }
}
