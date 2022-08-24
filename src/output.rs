use crate::table::ToTableRows;
use cli_table::{CellStruct, Table};
use pi_hole_api::errors::APIError;
use serde::Serialize;
use std::collections::HashMap;

pub fn display<I, H, R>(results: I, title: Vec<CellStruct>, hosts: &[H], json: bool)
where
    I: Iterator<Item = Result<R, APIError>>,
    R: std::fmt::Debug + Serialize + ToTableRows,
    H: AsRef<str>,
{
    if json {
        let map: HashMap<String, Result<R, String>> = hosts
            .iter()
            .zip(results)
            .map(|(host, result)| {
                (
                    host.as_ref().to_owned(),
                    result.map_err(|e| format!("{:?}", e)),
                )
            })
            .collect();
        let serialised_json =
            serde_json::to_string_pretty(&map).expect("Unable to serialise results to JSON");
        println!("{}", serialised_json);

        // let serialised_yaml =
        //     serde_yaml::to_string(&map).expect("Unable to serialise results to YAML");
        // println!("{}", serialised_yaml);
    } else {
        let results: Vec<Result<R, APIError>> = results.collect();
        // Separate out the errors
        let errors = hosts
            .iter()
            .zip(&results)
            .filter_map(|(host, result)| match result {
                Ok(_) => None,
                Err(e) => Some((host, e)),
            })
            .map(|(host, error)| format!("{}: {:?}", host.as_ref(), error))
            .collect::<Vec<String>>();

        // Construct table rows from hosts and associated results
        let table_rows: Vec<Vec<CellStruct>> = hosts
            .iter()
            .zip(results)
            .filter_map(|(host, result)| match result {
                Ok(ok) => Some((host, ok)),
                Err(_) => None,
            })
            .flat_map(|(host, response_data)| response_data.to_table_rows(host.as_ref()))
            .collect();
        let table = table_rows.table().title(title);
        println!("{}", table.display().unwrap());

        if !errors.is_empty() {
            println!("Errors:");
            for error in errors {
                println!("{}", error);
            }
        }
    }
}
