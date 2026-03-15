/******************************************************************************
 * Project: system_inspector_tui
 * File: metrics.rs
 * Date: 05.03.2026
 * Author: Korawit Utai
 *
 * 
 *
******************************************************************************/













use sysinfo::{Components, CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct Metrics {
    sys: System,
    components: Components,
}

impl Metrics {
    pub fn new() -> Self {
        let refresh = RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything());

        let sys = System::new_with_specifics(refresh);
        let components = Components::new_with_refreshed_list();

        Self { sys, components }
    }

    pub fn refresh_all(&mut self) {
        self.sys.refresh_all();
        self.components.refresh();
    }

    pub fn cpu_text(&self) -> String {
        let cpus = self.sys.cpus();
        if cpus.is_empty() {
            return "CPU: no data".into();
        }

        let avg = cpus
            .iter()
            .map(|c| c.cpu_usage() as f64)
            .sum::<f64>()
            / cpus.len() as f64;

        let mut s = String::new();
        s.push_str(&format!("CPU avg usage: {:.1}%\n", avg));
        /*s.push_str(&format!("Cores: {}\n", cpus.len()));*/
        for (i, c) in cpus.iter().enumerate().take(32) {
            s.push_str(&format!(
                "  Core {:>2}: {:>5.1}% ({})\n",
                i,
                c.cpu_usage(),
                c.name()
            ));
        }
        if cpus.len() > 32 {
            s.push_str("  ...\n");
        }
        s
    }

    pub fn mem_text(&self) -> String {
      
        let total = self.sys.total_memory();
        let used = self.sys.used_memory();

        let total_gib = total as f64 / 1024.0 / 1024.0;
        let used_gib = used as f64 / 1024.0 / 1024.0;
        let pct = if total == 0 {
            0.0
        } else {
            used as f64 / total as f64 * 100.0
        };

        format!(
            "Memory:\n  Used:  {:.2} GiB\n  Total: {:.2} GiB\n  Usage: {:.1}%",
            used_gib, total_gib, pct
        )
    }

    pub fn temp_text(&self) -> String {
        if self.components.is_empty() {
            return "Temperature:\n  ".into();
        }

        let mut s = String::from("Temperature sensors:\n");
        for c in self.components.iter().take(8) {
            s.push_str(&format!("  {}: {:.1}°C\n", c.label(), c.temperature()));
        }
        s
    }
}