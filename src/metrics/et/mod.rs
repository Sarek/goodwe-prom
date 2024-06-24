use super::definitions::{Current, Energy, Frequency, Integer, LargeEnergy, LargePower, MetricSet, Percentage, Power, Temperature, Voltage};

pub fn base_metrics() -> MetricSet {
  let metrics = vec![
    // ignore the first three words, they're some timestamp

    Voltage::easy(35103, "string", "PV1"),
    Current::easy(35104, "string", "PV1"),
    LargePower::easy(35105, "string", "PV1"),
    Voltage::easy(35107, "string", "PV2"),
    Current::easy(35108, "string", "PV2"),
    LargePower::easy(35109, "string", "PV2"),
    Voltage::easy(35111, "string", "PV3"),
    Current::easy(35112, "string", "PV3"),
    LargePower::easy(35113, "string", "PV3"),
    Voltage::easy(35115, "string", "PV4"),
    Current::easy(35116, "string", "PV4"),
    LargePower::easy(35117, "string", "PV4"),

    Voltage::easy(35121, "string", "On-Grid L1"),
    Current::easy(35122, "string", "On-Grid L1"),
    Frequency::easy(35123, "string", "On-Grid L1"),
    Power::easy(35125, "string", "On-Grid L1"),

    Voltage::easy(35126, "string", "On-Grid L2"),
    Current::easy(35127, "string", "On-Grid L2"),
    Frequency::easy(35128, "string", "On-Grid L2"),
    Power::easy(35130, "string", "On-Grid L2"),

    Voltage::easy(35131, "string", "On-Grid L3"),
    Current::easy(35132, "string", "On-Grid L3"),
    Frequency::easy(35133, "string", "On-Grid L3"),
    Power::easy(35135, "string", "On-Grid L3"),

    Power::easy(35138, "string", "Total Inverter Power"),

    Power::easy(35140, "string", "Active Power"),

    Voltage::easy(35145, "string", "Backup L1 Voltage"),
    Current::easy(35146, "string", "Backup L1 Current"),
    Frequency::easy(35147, "string", "Backup L1 Frequency"),
    Power::easy(35150, "string", "Backup L1 Power"),

    Voltage::easy(35151, "string", "Backup L2 Voltage"),
    Current::easy(35152, "string", "Backup L2 Current"),
    Frequency::easy(35153, "string", "Backup L2 Frequency"),
    Power::easy(35156, "string", "Backup L2 Power"),

    Voltage::easy(35157, "string", "Backup L3 Voltage"),
    Current::easy(35158, "string", "Backup L3 Current"),
    Frequency::easy(35159, "string", "Backup L3 Frequency"),
    Power::easy(35162, "string", "Backup L3 Power"),

    Power::easy(35164, "string", "Load L1"),
    Power::easy(35166, "string", "Load L2"),
    Power::easy(35168, "string", "Load L3"),
    Power::easy(35170, "string", "Backup Load"),
    Power::easy(35172, "string", "Load Total"),

    Percentage::easy(35173, "string", "Backup Utilization"),

    Temperature::easy(35174, "string", "Inverter Temperature (Air)"),
    Temperature::easy(35175, "string", "Inverter Temperature (Module)"),
    Temperature::easy(35176, "string", "Inverter Temperature (Radiator)"),

    Voltage::easy(35178, "string", "Bus Voltage"),
    Voltage::easy(35179, "string", "NBus Voltage"),

    Voltage::easy(35180, "string", "Battery"),
    Current::easy(35181, "string", "Battery"),
    // two-word value
    LargePower::easy(35182, "string", "Battery"),

    // It seems those counters consist of two words
    LargeEnergy::easy(35191, "string", "Total PV Generation"),
    LargeEnergy::easy(35193, "string", "Today's PV Generation"),
    LargeEnergy::easy(35195, "string", "Total Energy Export"),

    // 35197: Total hours

    Energy::easy(35199, "string", "Today's Energy Export"),
    // Two-word counter
    Energy::easy(35200, "string", "Total Energy Import"),
    Energy::easy(35202, "string", "Today's Energy Import"),
  ];

  MetricSet{
    base: 35100,
    metrics,
  }
}

pub fn battery_metrics() -> MetricSet {
  let metrics = vec![
    Integer::easy(37000, "string", "Battery BMS"),
    Integer::easy(37001, "string", "Battery Index"),
    Integer::easy(37002, "string", "Battery Status"),
    Temperature::easy(37003, "string", "Battery Temperature"),
    Integer::easy(37004, "string", "Battery Charge Limit"),
    Integer::easy(37005, "string", "Battery Discharge Limit"),
    Integer::easy(37006, "string", "Battery Error L"),
    Percentage::easy(37007, "string", "Battery State of Charge"),
    Percentage::easy(37008, "string", "Battery State of Health"),
    Integer::easy(37009, "string", "Battery Modules"),
    Integer::easy(37010, "string", "Battery Warning L"),
    Integer::easy(37011, "string", "Battery Error H"),
    Integer::easy(37013, "string", "Battery Warning H"),
    Integer::easy(37014, "string", "Battery SW Version"),
    Integer::easy(37015, "string", "Battery HW Version"),
    Integer::easy(37016, "string", "Battery Max Cell Temp ID"),
    Integer::easy(37017, "string", "Battery Min Cell Temp ID"),
    Integer::easy(37018, "string", "Battery Max Cell Voltage ID"),
    Integer::easy(37019, "string", "Battery Min Cell Voltage ID"),
    Temperature::easy(37020, "string", "Battery Max Cell Temperature"),
    Temperature::easy(37021, "string", "Battery Max Cell Temperature"),
    Voltage::easy(37022, "string", "Battery Max Cell Voltage"),
    Voltage::easy(37023, "string", "Battery Min Cell Voltage")
  ];

  MetricSet {
    base: 37000,
    metrics
  }
}
