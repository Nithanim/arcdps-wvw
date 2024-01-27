//use csv;
//use once_cell::sync::Lazy;
#[derive(Debug)]
pub struct Record {
    pub map_id: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/*
static DATA: &[u8] = include_bytes!("../../../resources/o.csv");

pub static RECORDS: Lazy<Vec<Record>> = Lazy::new(|| {
    let mut rdr = csv::Reader::from_reader(Cursor::new(DATA));
    let mut records = Vec::new();

    for result in rdr.records() {
        let record: Record = match result {
            Ok(r) => Record {
                map_id: r[0].parse::<u32>().unwrap(),
                x: r[1].parse::<f32>().unwrap(),
                y: r[2].parse::<f32>().unwrap(),
                z: r[3].parse::<f32>().unwrap(),
            },
            Err(err) => panic!("Error reading CSV: {}", err),
        };
        records.push(record);
    }

    records
});
*/
