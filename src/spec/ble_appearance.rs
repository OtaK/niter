#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
)]
#[cfg_attr(target_os = "linux", derive(zvariant_derive::Type))]
#[repr(u16)]
pub enum BLEAppearance {
    /// Unknown device
    Unknown = 0,
    /// Generic Phone
    GenericPhone = 64,
    /// Generic Computer
    GenericComputer = 128,
    /// Generic Watch
    GenericWatch = 192,
    /// Watch subtype
    WatchSportsWatch = 193,
    /// Generic category
    GenericClock = 256,
    /// Generic category
    GenericDisplay = 320,
    /// Generic category
    GenericRemoteControl = 384,
    /// Generic category
    GenericEyeGlasses = 448,
    /// Generic category
    GenericTag = 512,
    /// Generic category
    GenericKeyring = 576,
    /// Generic category
    GenericMediaPlayer = 640,
    /// Generic category
    GenericBarcodeScanner = 704,
    /// Generic category
    GenericThermometer = 768,
    /// Thermometer subtype
    ThermometerEar = 769,
    /// Generic category
    GenericHeartrateSensor = 832,
    /// Heart Rate Sensor subtype
    HeartRateSensorHeartRateBelt = 833,
    /// Generic category
    GenericBloodPressure = 896,
    /// Blood Pressure subtype
    BloodPressureArm = 897,
    /// Blood Pressure subtype
    BloodPressureWrist = 898,
    /// HID Generic
    HumanInterfaceDevice = 960,
    /// HID subtype
    Keyboard = 961,
    /// HID subtype
    Mouse = 962,
    /// HID subtype
    Joystick = 963,
    /// HID subtype
    Gamepad = 964,
    /// HID subtype
    DigitizerTablet = 965,
    /// HID subtype
    CardReader = 966,
    /// HID subtype
    DigitalPen = 967,
    /// HID subtype
    BarcodeScanner = 968,
    /// Generic category
    GenericGlucoseMeter = 1024,
    /// Generic category
    GenericRunningWalkingSensor = 1088,
    /// Running Walking Sensor subtype
    RunningWalkingSensorInShoe = 1089,
    /// Running Walking Sensor subtype
    RunningWalkingSensorOnShoe = 1090,
    /// Running Walking Sensor subtype
    RunningWalkingSensorOnHip = 1091,
    /// Generic category
    GenericCycling = 1152,
    /// Cycling subtype
    CyclingCyclingComputer = 1153,
    /// Cycling subtype
    CyclingSpeedSensor = 1154,
    /// Cycling subtype
    CyclingCadenceSensor = 1155,
    /// Cycling subtype
    CyclingPowerSensor = 1156,
    /// Cycling subtype
    CyclingSpeedCadenceSensor = 1157,
    /// Pulse Oximeter Generic Gategory
    GenericPulseOximeter = 3136,
    /// Pulse Oximeter subtype
    Fingertip = 3137,
    /// Pulse Oximeter subtype
    WristWorn = 3138,
    /// Weight Scale Generic Category
    GenericWeightScale = 3200,
    /// Outdoor Sports Activity Generic Category
    GenericOutdoorSportsActivity = 5184,
    /// Outdoor Sports Activity subtype
    LocationDisplayDevice = 5185,
    /// Outdoor Sports Activity subtype
    LocationNavigationDisplayDevice = 5186,
    /// Outdoor Sports Activity subtype
    LocationPod = 5187,
    /// Outdoor Sports Activity subtype
    LocationNavigationPod = 5188,
    /// Environmental Sensor Generic Category
    GenericEnvironmentalSensor = 5696,
}

impl std::convert::TryFrom<zvariant::OwnedValue> for BLEAppearance {
    type Error = crate::NiterError;
    fn try_from(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
        use std::convert::TryInto as _;
        let intermediate: u16 = value.try_into()?;
        Ok(intermediate.try_into().map_err(anyhow::Error::from)?)
    }
}

impl Into<zvariant::Structure<'_>> for BLEAppearance {
    fn into(self) -> zvariant::Structure<'static> {
        zvariant::StructureBuilder::new().append_field(zvariant::Value::U16(self as _)).build()
    }
}
