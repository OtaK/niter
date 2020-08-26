#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
)]
#[cfg_attr(target_os = "linux", derive(zvariant_derive::Type))]
#[repr(u16)]
pub enum DescriptorNumber {
    CharacteristicAggregateFormat = 0x2905,
    CharacteristicExtenderProperties = 0x2900,
    CharacteristicPresentationFormat = 0x2904,
    CharacteristicUserDescription = 0x2901,
    ClientCharacteristicConfiguration = 0x2902,
    EnvironmentalSensingConfiguration = 0x290B,
    EnvironmentalSensingMeasurement = 0x290C,
    EnvironmentalSensingTriggerSetting = 0x290D,
    ExternalReportReference = 0x2907,
    NumberOfDigitals = 0x2909,
    ReportReference = 0x2908,
    ServerCharacteristicConfiguration = 0x2903,
    TimeTriggerSetting = 0x290E,
    ValidRange = 0x2906,
    ValueTriggerSetting = 0x290A,
}
