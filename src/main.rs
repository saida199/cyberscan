mod system;
mod analysis;
mod display;

use system::collect_system_data;
use analysis::analyze;
use display::display_scan;

fn main() {
    let data = collect_system_data();
    let analysis = analyze(&data);

    display_scan(&data, &analysis);
}
