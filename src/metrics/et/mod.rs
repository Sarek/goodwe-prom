use super::definitions::{
    Current, Energy, Frequency, Integer, LargeEnergy, LargePower, MetricSet, Percentage, Power,
    Temperature, Voltage,
};

const METRIC_VOLTAGE_PV: &str = "voltage_pv_volts";
const METRIC_CURRENT_PV: &str = "current_pv_amperes";
const METRIC_POWER_PV: &str = "power_pv_watts";

const METRIC_VOLTAGE_GRID: &str = "voltage_grid_volts";
const METRIC_CURRENT_GRID: &str = "current_grid_amperes";
const METRIC_POWER_GRID: &str = "power_grid_watts";
const METRIC_FREQUENCY_GRID: &str = "frequency_grid_hertz";

const METRIC_VOLTAGE_BACKUP: &str = "voltage_backup_volts";
const METRIC_CURRENT_BACKUP: &str = "current_backup_amperes";
const METRIC_POWER_BACKUP: &str = "power_backup_watts";
const METRIC_FREQUENCY_BACKUP: &str = "frequency_backup_hertz";

const METRIC_LOAD: &str = "load_watts";

const METRIC_TEMP: &str = "temperature_celsius";

const METRIC_INT_VOLTAGE: &str = "voltage_internal_volts";

pub fn base_metrics() -> MetricSet {
    let metrics = vec![
        // ignore the first three words, they're some timestamp
        Voltage::easy(35103, METRIC_VOLTAGE_PV, "mppt", "pv1"),
        Current::easy(35104, METRIC_CURRENT_PV, "mppt", "pv1"),
        LargePower::easy(35105, METRIC_POWER_PV, "mppt", "pv1"),
        Voltage::easy(35107, METRIC_VOLTAGE_PV, "mppt", "pv2"),
        Current::easy(35108, METRIC_CURRENT_PV, "mppt", "pv2"),
        LargePower::easy(35109, METRIC_POWER_PV, "mppt", "pv2"),
        Voltage::easy(35111, METRIC_VOLTAGE_PV, "mppt", "pv3"),
        Current::easy(35112, METRIC_CURRENT_PV, "mppt", "pv3"),
        LargePower::easy(35113, METRIC_POWER_PV, "mppt", "pv3"),
        Voltage::easy(35115, METRIC_VOLTAGE_PV, "mppt", "pv4"),
        Current::easy(35116, METRIC_CURRENT_PV, "mppt", "pv4"),
        LargePower::easy(35117, METRIC_POWER_PV, "mppt", "pv4"),
        Voltage::easy(35121, METRIC_VOLTAGE_GRID, "phase", "L1"),
        Current::easy(35122, METRIC_CURRENT_GRID, "phase", "L1"),
        Frequency::easy(35123, METRIC_FREQUENCY_GRID, "phase", "L1"),
        Power::easy(35125, METRIC_POWER_GRID, "phase", "L1"),
        Voltage::easy(35126, METRIC_VOLTAGE_GRID, "phase", "L2"),
        Current::easy(35127, METRIC_CURRENT_GRID, "phase", "L2"),
        Frequency::easy(35128, METRIC_FREQUENCY_GRID, "phase", "L2"),
        Power::easy(35130, METRIC_POWER_GRID, "phase", "L2"),
        Voltage::easy(35131, METRIC_VOLTAGE_GRID, "phase", "L3"),
        Current::easy(35132, METRIC_CURRENT_GRID, "phase", "L3"),
        Frequency::easy(35133, METRIC_FREQUENCY_GRID, "phase", "L3"),
        Power::easy(35135, METRIC_POWER_GRID, "phase", "L3"),
        // TODO: Remove the redundant labels
        Power::easy(35138, "inverter_power_total_watts", "none", "none"),
        Power::easy(35140, "active_power_total_watts", "none", "none"),
        Voltage::easy(35145, METRIC_VOLTAGE_BACKUP, "phase", "L1"),
        Current::easy(35146, METRIC_CURRENT_BACKUP, "phase", "L1"),
        Frequency::easy(35147, METRIC_FREQUENCY_BACKUP, "phase", "L1"),
        Power::easy(35150, METRIC_POWER_BACKUP, "phase", "L1"),
        Voltage::easy(35151, METRIC_VOLTAGE_BACKUP, "phase", "L2"),
        Current::easy(35152, METRIC_CURRENT_BACKUP, "phase", "L2"),
        Frequency::easy(35153, METRIC_FREQUENCY_BACKUP, "phase", "L2"),
        Power::easy(35156, METRIC_POWER_BACKUP, "phase", "L2"),
        Voltage::easy(35157, METRIC_VOLTAGE_BACKUP, "phase", "L3"),
        Current::easy(35158, METRIC_CURRENT_BACKUP, "phase", "L3"),
        Frequency::easy(35159, METRIC_FREQUENCY_BACKUP, "phase", "L3"),
        Power::easy(35162, METRIC_POWER_BACKUP, "phase", "L3"),
        Power::easy(35164, METRIC_LOAD, "phase", "L1"),
        Power::easy(35166, METRIC_LOAD, "phase", "L2"),
        Power::easy(35168, METRIC_LOAD, "phase", "L3"),
        Power::easy(35170, METRIC_LOAD, "type", "Backup"),
        Power::easy(35172, METRIC_LOAD, "type", "Total"),
        Percentage::easy(35173, "backup_utilization_ratio", "none", "none"),
        Temperature::easy(35174, METRIC_TEMP, "sensor", "Air"),
        Temperature::easy(35175, METRIC_TEMP, "sensor", "Module"),
        Temperature::easy(35176, METRIC_TEMP, "sensor", "Radiator"),
        Voltage::easy(35178, METRIC_INT_VOLTAGE, "sensor", "Bus"),
        Voltage::easy(35179, METRIC_INT_VOLTAGE, "sensor", "NBus"),
        Voltage::easy(35180, "voltage_battery_volts", "none", "none"),
        Current::easy(35181, "current_battery_volts", "string", "Battery"),
        // two-word value
        LargePower::easy(35182, "power_battery_watts", "none", "none"),
        // It seems those counters consist of two words
        LargeEnergy::easy(35191, "pv_generation_total", "timeframe", "all"),
        LargeEnergy::easy(35193, "pv_generation_total", "timeframe", "today"),
        LargeEnergy::easy(35195, "pv_export_total", "timeframe", "all"),
        // 35197: Total hours
        Energy::easy(35199, "pv_export_total", "timeframe", "today"),
        // Two-word counter
        Energy::easy(35200, "energy_import_total", "timeframe", "all"),
        Energy::easy(35202, "energy_import_total", "timeframe", "today"),
    ];

    MetricSet {
        base: 35100,
        metrics,
    }
}

pub fn battery_metrics() -> MetricSet {
    let metrics = vec![
        Integer::easy(37000, "battery_bms", "none", "none"),
        Integer::easy(37001, "battery_index", "none", "none"),
        Integer::easy(37002, "battery_status", "none", "none"),
        Temperature::easy(37003, METRIC_TEMP, "sensor", "Battery"),
        Integer::easy(37004, "battery_current_limit_amperes", "type", "Charge"),
        Integer::easy(37005, "battery_current_limit_amperes", "type", "Discharge"),
        Integer::easy(37006, "battery_error", "side", "L"),
        Percentage::easy(37007, "battery_state_ratio", "type", "State of Charge"),
        Percentage::easy(37008, "battery_state_ratio", "type", "State of Health"),
        Integer::easy(37009, "battery_modules", "none", "none"),
        Integer::easy(37010, "battery_warning", "side", "L"),
        Integer::easy(37011, "battery_error", "side", "H"),
        Integer::easy(37013, "battery_warning", "side", "H"),
        Integer::easy(37014, "battery_version", "part", "SW"),
        Integer::easy(37015, "battery_version", "part", "HW"),
        Integer::easy(37016, "battery_cell_temp_id", "type", "Max"),
        Integer::easy(37017, "battery_cell_temp_id", "type", "Min"),
        Integer::easy(37018, "battery_cell_voltage_id", "type", "Max"),
        Integer::easy(37019, "battery_cell_voltage_id", "type", "Min"),
        Temperature::easy(37020, "battery_cell_temp_celsius", "type", "Max"),
        Temperature::easy(37021, "battery_cell_temp_celsius", "type", "Min"),
        Voltage::easy(37022, "battery_cell_voltage_volts", "type", "Max"),
        Voltage::easy(37023, "battery_cell_voltage_volts", "type", "Min"),
    ];

    MetricSet {
        base: 37000,
        metrics,
    }
}
