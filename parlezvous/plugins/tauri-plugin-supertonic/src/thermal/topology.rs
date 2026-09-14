#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SocClass {
    Flagship,    // SD 8 Gen 2/3, Dimensity 9200+
    HighEnd,     // SD 7+ Gen 2, Dimensity 8200
    MidRange,    // SD 695, Dimensity 700
    LowEnd,      // SD 680, entry-level
}

#[derive(Debug, Clone)]
pub struct CpuTopology {
    pub prime_core: Option<usize>,
    pub perf_cores: Vec<usize>,
    pub little_cores: Vec<usize>,
    pub soc_class: SocClass,
}

impl CpuTopology {
    pub fn detect() -> Self {
        let mut cores_with_freq: Vec<(usize, u64)> = Vec::new();

        for cpu in 0..16 {
            let freq_path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/cpuinfo_max_freq", cpu);
            if let Ok(freq_str) = std::fs::read_to_string(&freq_path) {
                if let Ok(freq) = freq_str.trim().parse::<u64>() {
                    cores_with_freq.push((cpu, freq));
                }
            }
        }

        if cores_with_freq.is_empty() {
            return Self::fallback();
        }

        cores_with_freq.sort_by(|a, b| b.1.cmp(&a.1));

        let max_freq = cores_with_freq[0].1;
        let num_cores = cores_with_freq.len();

        // Classify SoC based on topology
        let soc_class = Self::classify_soc(max_freq, num_cores);

        log::info!("Detected SoC class: {:?}, max freq: {} MHz, cores: {}",
            soc_class, max_freq / 1000, num_cores);

        // Parse topology based on frequency tiers
        let (prime_core, perf_cores, little_cores) =
            Self::parse_topology(&cores_with_freq, soc_class);

        Self {
            prime_core,
            perf_cores,
            little_cores,
            soc_class,
        }
    }

    fn classify_soc(max_freq: u64, num_cores: usize) -> SocClass {
        // Flagship: Prime core > 3.0 GHz, 8+ cores
        if max_freq > 3_000_000 && num_cores >= 8 {
            SocClass::Flagship
        }
        // High-end: Max freq > 2.5 GHz, 8 cores
        else if max_freq > 2_500_000 && num_cores >= 8 {
            SocClass::HighEnd
        }
        // Mid-range: Max freq > 2.0 GHz
        else if max_freq > 2_000_000 {
            SocClass::MidRange
        }
        // Low-end: Everything else
        else {
            SocClass::LowEnd
        }
    }

    fn parse_topology(
        cores_with_freq: &[(usize, u64)],
        soc_class: SocClass,
    ) -> (Option<usize>, Vec<usize>, Vec<usize>) {
        let max_freq = cores_with_freq[0].1;
        let min_freq = cores_with_freq.last().unwrap().1;
        let freq_range = max_freq - min_freq;

        match soc_class {
            SocClass::Flagship => {
                // 1 Prime + 3-4 Perf + 3-4 Little
                let prime_threshold = max_freq - (freq_range / 20); // Top 5%
                let perf_threshold = max_freq - (freq_range / 3);   // Middle tier

                let mut prime = None;
                let mut perf = Vec::new();
                let mut little = Vec::new();

                for &(cpu, freq) in cores_with_freq {
                    if prime.is_none() && freq >= prime_threshold {
                        prime = Some(cpu);
                    } else if freq >= perf_threshold {
                        perf.push(cpu);
                    } else {
                        little.push(cpu);
                    }
                }

                (prime, perf, little)
            }

            SocClass::HighEnd | SocClass::MidRange => {
                // 2-4 Big + 4-6 Little (no prime)
                let perf_threshold = max_freq - (freq_range / 4);

                let mut perf = Vec::new();
                let mut little = Vec::new();

                for &(cpu, freq) in cores_with_freq {
                    if freq >= perf_threshold {
                        perf.push(cpu);
                    } else {
                        little.push(cpu);
                    }
                }

                (None, perf, little)
            }

            SocClass::LowEnd => {
                // Often homogeneous or 4+4
                if freq_range < 200_000 {
                    // Homogeneous - treat all as "perf"
                    let all_cores: Vec<_> = cores_with_freq.iter().map(|&(cpu, _)| cpu).collect();
                    (None, all_cores, vec![])
                } else {
                    let mid = cores_with_freq.len() / 2;
                    let perf: Vec<_> = cores_with_freq[..mid].iter().map(|&(cpu, _)| cpu).collect();
                    let little: Vec<_> = cores_with_freq[mid..].iter().map(|&(cpu, _)| cpu).collect();
                    (None, perf, little)
                }
            }
        }
    }

    fn fallback() -> Self {
        log::warn!("Could not detect CPU topology, using fallback");
        Self {
            prime_core: None,
            perf_cores: (0..4).collect(),
            little_cores: (4..8).collect(),
            soc_class: SocClass::MidRange,
        }
    }
}
