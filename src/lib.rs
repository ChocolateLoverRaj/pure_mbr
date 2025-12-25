#![cfg_attr(not(feature = "std"), no_std)]

use zerocopy::{FromBytes, Immutable, IntoBytes, little_endian::U32};

#[repr(C)]
#[derive(Debug, FromBytes, IntoBytes, Immutable)]
pub struct GenericMbr {
    unused: [u8; 446],
    pub partition_entries: [PartitionEntry; 4],
    boot_signature: [u8; 2],
}

#[repr(C)]
#[derive(Debug, FromBytes, IntoBytes, Immutable)]
pub struct PartitionEntry {
    status: u8,
    chs_address_first: [u8; 3],
    partition_type: u8,
    chs_address_last: [u8; 3],
    lba: U32,
    n_sectors: U32,
}

impl PartitionEntry {
    /// If this partition entry is empty, it means that no partition exists for this entry, and this entry is basically a free slot
    pub fn is_empty(&self) -> bool {
        self.partition_type() == 0x00
    }

    /// See https://en.wikipedia.org/wiki/Partition_type#List_of_partition_IDs for the meaning of this
    pub fn partition_type(&self) -> u8 {
        self.partition_type
    }

    /// Returns the start sector of this partition (multiply by 512 bytes to get the start position in bytes)
    pub fn start_sector(&self) -> u32 {
        self.lba.into()
    }

    /// Returns the number of sectors (1 sector = 512 bytes) of this partition
    pub fn sector_len(&self) -> u32 {
        self.n_sectors.into()
    }
}

#[cfg(test)]
mod test {
    use zerocopy::transmute_ref;

    use crate::GenericMbr;

    #[test]
    fn example() {
        let mbr: &GenericMbr = transmute_ref!(include_bytes!("../example.bin"));
        for partition in mbr
            .partition_entries
            .iter()
            .filter(|entry| !entry.is_empty())
        {
            println!("partition: {partition:#?}");
        }
    }
}
