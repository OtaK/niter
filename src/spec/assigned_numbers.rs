#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
)]
#[cfg_attr(target_os = "linux", derive(zvariant_derive::Type))]
#[repr(u16)]
pub enum AssignedRfcommNumber {
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
