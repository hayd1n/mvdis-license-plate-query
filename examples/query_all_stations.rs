use std::{collections::HashMap, sync::Arc, vec};

use mvdis_license_plate_query::{ocr::Ocr, options, Client, PlateInfo, QueryBuilder};
use strum::IntoEnumIterator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let retry_times = 3;
    let plate_ver = options::PlateVer::New;
    let plate_type = options::PlateType::Motorcycle550ccBelow;

    let mut results: HashMap<String, HashMap<String, Vec<PlateInfo>>> = HashMap::new();

    let client = Arc::new(Client::new());
    let ocr = Arc::new(Ocr::new()?);

    for region in options::Region::iter() {
        let region_entry = results
            .entry(region.as_name().to_string())
            .or_insert_with(HashMap::new);
        for station in options::get_station(region) {
            let station_entry = region_entry
                .entry(station.as_name().to_string())
                .or_insert_with(Vec::new);
            for window_no in options::WindowNo::iter() {
                let options = mvdis_license_plate_query::QueryOptions {
                    region,
                    station,
                    window_no,
                    plate_ver,
                    plate_type,
                };

                println!(
                    "Querying {}({}, {}) license plates in {} {} {}...",
                    options.plate_type.as_name(),
                    options.plate_type.energy_type().as_name(),
                    options.plate_type.vehicle_type().as_name(),
                    options.region.as_name(),
                    options.station.as_name(),
                    options.window_no.as_str()
                );

                let mut query_results = vec![];

                // Send the query and iterate through the results
                let mut attempts = 0;
                let mut query = loop {
                    let client = Arc::clone(&client);
                    let ocr = Arc::clone(&ocr);
                    match QueryBuilder::new(client, options).send(ocr).await {
                        Ok(query) => break query,
                        Err(mvdis_license_plate_query::Error::CaptchaError)
                            if attempts < retry_times =>
                        {
                            attempts += 1;
                            println!(
                                "Captcha error encountered. Retrying... (attempt {}/{})",
                                attempts, retry_times
                            );
                        }
                        Err(e) => return Err(e.into()),
                    }
                };
                loop {
                    let result = query.results()?.unwrap();
                    query_results.extend(result);

                    let have_next_page = query.next_page().await?;
                    if !have_next_page {
                        break;
                    }
                    println!("Fetching next page...");
                }

                station_entry.extend(query_results);
            }
        }
    }

    // Calculate the total number of plates
    let total_plates: usize = results
        .values()
        .map(|stations| stations.values().map(|plates| plates.len()).sum::<usize>())
        .sum();

    // Print the results
    for (region, stations) in &results {
        for (station, plates) in stations {
            println!("{} - {}", region, station);
            for plate in plates {
                println!("{}\t${}", plate.plate_no, plate.price);
            }
        }
    }

    println!("Total plates: {}", total_plates);

    Ok(())
}
