// Interacting with the hos system

use comfy_table::Table;
use sysinfo::{Components, Disks, Networks, System};

fn get_sys() -> System {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys
}

fn get_nets() -> Networks {
    Networks::new_with_refreshed_list()
}

fn get_disks() -> Disks {
    Disks::new_with_refreshed_list()
}

fn get_components() -> Components {
    Components::new_with_refreshed_list()
}

pub fn get_sys_summary() {
    let track_sys = get_sys();
    let track_nets = get_nets();
    let _track_disks = get_disks();
    let _track_components = get_components();

    let mut table = Table::new();

    table.set_header(vec!["Attribute", "State"]);
    table.add_row(vec![
        "Total memory".to_string(),
        track_sys.total_memory().to_string(),
    ]);
    for (interface_name, _data) in &track_nets {
        table.add_row(vec!["Network", interface_name]);
    }

    // TODO: Handle disks and components

    println!("{table}");
}
