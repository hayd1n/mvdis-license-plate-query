use std::{sync::Arc, vec};

use mvdis_license_plate_query::{ocr::Ocr, options, Client, QueryBuilder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Arc::new(Client::new());
    let ocr = Arc::new(Ocr::new()?);

    let options = mvdis_license_plate_query::QueryOptions {
        region: options::Region::Taipei,
        station: options::Station::TaipeiCity,
        window_no: options::WindowNo::One,
        plate_ver: options::PlateVer::New,
        plate_type: options::PlateType::Motorcycle550ccBelow,
    };

    println!(
        "Querying {}({}, {}) license plates in {} {} {}...",
        options.plate_type.as_name(),
        options.plate_type.energy_type().as_name(),
        options.plate_type.vehicle_type().as_name(),
        options.region.as_name(),
        options.station.as_name(),
        options.window_no.value()
    );

    let mut results = vec![];

    // Send the query and iterate through the results
    let mut query = QueryBuilder::new(client, options).send(ocr).await?;
    loop {
        let result = query.results()?.unwrap();
        results.extend(result);

        let have_next_page = query.next_page().await?;
        if !have_next_page {
            break;
        }
        println!("Fetching next page...");
    }

    for result in results {
        println!("{}\t${}", result.plate_no, result.price);
    }

    Ok(())
}
