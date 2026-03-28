use pure_mbr::GenericMbr;
use zerocopy::transmute_ref;

fn main() {
    let mbr: &GenericMbr = transmute_ref!(include_bytes!("../example2.bin"));
    for partition in mbr
        .partition_entries
        .iter()
        .filter(|entry| !entry.is_empty())
    {
        println!("partition: {partition:#?}");
    }
}
