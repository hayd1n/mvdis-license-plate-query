use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use mvdis_license_plate_query::{ocr::Ocr, options, Client, QueryBuilder};
use strum::IntoEnumIterator;
use tokio_task_pool::Pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let retry_times = 3;
    let plate_ver = options::PlateVer::New;
    let plate_type = options::PlateType::CarOwn;

    // Initialize results
    let mut initial_results = HashMap::new();
    for region in options::Region::iter() {
        let mut station_map = HashMap::new();
        for station in options::get_station(region) {
            station_map.insert(station.as_name().to_string(), Vec::new());
        }
        initial_results.insert(region.as_name().to_string(), station_map);
    }
    let results = Arc::new(Mutex::new(initial_results));

    let ocr = Arc::new(Ocr::new()?);

    let cpu_num = num_cpus::get();
    println!("CPU cores: {}", cpu_num);

    let pool = Pool::bounded(cpu_num);
    let mut handles = vec![];

    for region in options::Region::iter() {
        for station in options::get_station(region) {
            for window_no in options::WindowNo::iter() {
                let results = Arc::clone(&results);
                let ocr = Arc::clone(&ocr);
                let client = Arc::new(Client::new());

                handles.push(
                    pool.spawn(async move {
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
                        let mut attempts = 0;

                        // Send query and retry on captcha error
                        let mut query = loop {
                            match QueryBuilder::new(Arc::clone(&client), options)
                                .send(Arc::clone(&ocr))
                                .await
                            {
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
                                Err(e) => return Err(anyhow::Error::from(e)),
                            }
                        };

                        // Next page until no more pages
                        let mut current_page = 1;
                        loop {
                            let result = query.results()?.unwrap();
                            query_results.extend(result);

                            let have_next_page = query.next_page().await?;
                            if !have_next_page {
                                break;
                            }
                            current_page += 1;
                            println!(
                                "Fetching next page ({} {}, page {})...",
                                options.region.as_name(),
                                options.station.as_name(),
                                current_page
                            );
                        }

                        // Insert results into the results map
                        {
                            let mut results_lock = results.lock().unwrap();
                            if let Some(station_entry) =
                                results_lock.get_mut(&region.as_name().to_string())
                            {
                                if let Some(station_results) =
                                    station_entry.get_mut(&station.as_name().to_string())
                                {
                                    station_results.extend(query_results);
                                }
                            }
                        }

                        Ok::<(), anyhow::Error>(())
                    })
                    .await?,
                );
            }
        }
    }

    // Wait for all tasks to finish
    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Error: {:?}", e);
        }
    }

    let results = results.lock().unwrap();

    // Calculate total plates
    let total_plates: usize = results
        .values()
        .map(|stations| stations.values().map(|plates| plates.len()).sum::<usize>())
        .sum();

    // Print results
    for (region, stations) in results.iter() {
        for (station, plates) in stations {
            println!("{} {}", region, station);
            for plate in plates {
                println!("{} ${}", plate.plate_no, plate.price);
            }
        }
    }

    println!("Total plates: {}", total_plates);

    Ok(())
}
