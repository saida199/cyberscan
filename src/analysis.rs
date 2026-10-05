use rand::Rng;
use crate::system::SystemData;

pub struct Analysis {
    pub stability_score: u8,
    pub load_factor: f32,
}

pub fn analyze(data: &SystemData) -> Analysis {
    let mut rng = rand::thread_rng();

    let load_factor = (data.cpu_usage / 100.0)
        + (data.used_memory as f32 / data.total_memory as f32);

    let stability_score = (100.0 - (load_factor * 50.0)) as u8
        + rng.gen_range(0..5);

    Analysis {
        stability_score,
        load_factor,
    }
}
