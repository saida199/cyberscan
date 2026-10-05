use crate::{system::SystemData, analysis::Analysis};

pub fn display_scan(data: &SystemData, analysis: &Analysis) {
    println!("============================================");
    println!("                CYBERSCAN v1.0              ");
    println!("============================================\n");

    println!("SYSTEM OVERVIEW");
    println!("--------------------------------------------");
    println!("CPU Usage        : {:.2}%", data.cpu_usage);
    println!("Memory Used      : {} / {} MB",
        data.used_memory / 1024,
        data.total_memory / 1024
    );
    println!("Active Processes : {}", data.process_count);

    println!("\nANALYSIS");
    println!("--------------------------------------------");
    println!("Load Factor      : {:.3}", analysis.load_factor);
    println!("Stability Score  : {} / 100", analysis.stability_score);

    println!("\nSUMMARY");
    println!("--------------------------------------------");

    if analysis.stability_score > 80 {
        println!("System operating within optimal parameters.");
    } else if analysis.stability_score > 60 {
        println!("System stable. Minor load detected.");
    } else if analysis.stability_score > 40 {
        println!("System under moderate stress.");
    } else {
        println!("System instability detected. High load.");
    }

    println!("\nEND OF REPORT");
    println!("============================================");
}
