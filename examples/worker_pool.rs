use mvdis_license_plate_query::{
    options::{PlateType, PlateVer},
    pool::{PoolClient, PoolClientOptions},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Initializing pool client...");

    let options = PoolClientOptions {
        plate_ver: PlateVer::New,
        plate_type: PlateType::Motorcycle550ccBelow,
        retry_times: 3,
        // Optional: specify regions, default is all
        // regions: Some(vec![mvdis_license_plate_query::options::Region::Taipei]),
        ..Default::default()
    };

    let client = PoolClient::new(options)?;

    println!(
        "Querying region(s)... This may take a while. Concurrency: {}",
        client.options().concurrency
    );

    let results = client.execute().await?;

    let mut total_plates = 0;

    for (region, stations) in results.iter() {
        for (station, plates) in stations {
            println!("{} {}", region.as_name(), station.as_name());
            for plate in plates {
                println!("{} ${}", plate.plate_no, plate.price);
            }
            total_plates += plates.len();
        }
    }

    println!("Total plates: {}", total_plates);

    Ok(())
}
