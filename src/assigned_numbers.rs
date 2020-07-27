#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum AssignedRfcommNumbers {
    Dun = 1,
    Spp = 3,
    HspHs = 6,
    HfpHf = 7,
    Opp = 9,
    Ftp = 10,
    Bip = 11,
    HspAg = 12,
    HfpAg = 13,
    Synch = 14,
    Pbap = 15,
    MapMas = 16,
    MapMns = 17,
    SyncEvolution = 19,
    PcOviSuite = 24,
    SyncMLClient = 25,
    SyncMLServer = 26,
}
