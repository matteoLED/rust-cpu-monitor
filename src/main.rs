use std::{thread, time::Duration};
use sysinfo::{ProcessorExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();

    loop {
        // Rafraîchit les informations du système
        sys.refresh_cpu();

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

        // Pause de 5 secondes
        thread::sleep(Duration::from_secs(2));
    }
}
