use std::{
    thread,
    time::{Duration, Instant},
};
use sysinfo::{ProcessorExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    let mut frame_count = 0;
    let mut start_time = Instant::now();
    let measure_interval = Duration::from_secs(5);
    let mut last_measure_time = Instant::now();

    loop {
        // Rafraîchit les informations du système toutes les 5 secondes
        if last_measure_time.elapsed() >= measure_interval {
            sys.refresh_all();

            // Affiche l'utilisation globale du CPU
            let global_cpu_usage = sys.global_processor_info().cpu_usage();
            println!("Utilisation globale du CPU : {:.2}%", global_cpu_usage);

            // Affiche l'utilisation du CPU par cœur
            for (i, processor) in sys.processors().iter().enumerate() {
                println!("Processeur {}: {:.2}%", i, processor.cpu_usage());
            }

            // Affiche l'utilisation de la mémoire
            let total_memory = sys.total_memory();
            let used_memory = sys.used_memory();
            println!("Mémoire utilisée : {} KB", used_memory);
            println!("Mémoire totale : {} KB", total_memory);

            // Affiche l'utilisation de la mémoire swap
            let total_swap = sys.total_swap();
            let used_swap = sys.used_swap();
            println!("Mémoire swap utilisée : {} KB", used_swap);
            println!("Mémoire swap totale : {} KB", total_swap);

            last_measure_time = Instant::now();
        }

        // Incrémente le compteur de frames
        frame_count += 1;

        // Calcule et affiche les FPS toutes les secondes
        let elapsed_time = start_time.elapsed();
        if elapsed_time >= Duration::from_secs(1) {
            let fps = frame_count as f64 / elapsed_time.as_secs_f64();
            println!("FPS : {:.2}", fps);

            // Réinitialise le compteur de frames et l'heure de départ
            frame_count = 0;
            start_time = Instant::now();
        }

        // Pause courte pour éviter une utilisation excessive du CPU
        thread::sleep(Duration::from_millis(100));
    }
}
