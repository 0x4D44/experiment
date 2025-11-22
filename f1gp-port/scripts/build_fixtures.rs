use f1gp_port::data::parser::tests::{build_test_track_bytes, SECTION_COUNT, TRACK_DATA_OFFSET};

fn main() {
    let (data, _) = build_test_track_bytes(SECTION_COUNT, TRACK_DATA_OFFSET);
    std::fs::create_dir_all("data/fixtures").unwrap();
    std::fs::write("data/fixtures/track_stub.bin", &data).unwrap();
    std::fs::copy("data/samples/driver_db.json", "data/fixtures/driver_db.json").unwrap();
    println!("Fixtures generated under data/fixtures");
}
