use std::time::Duration;
use super::topology::SocClass;

/// SoC-specific thermal configuration
#[derive(Debug, Clone)]
pub struct ThermalConfig {
    // Buffer thresholds
    pub buffer_low_threshold: f32,
    pub buffer_high_threshold: f32,

    // Thermal budgets
    pub max_high_power_duration: Duration,
    pub cooldown_duration: Duration,

    // Core selection strategy
    #[allow(dead_code)]
    pub use_prime_core: bool,
    pub sustained_mode_cores: CoreSelection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoreSelection {
    AllPerf,           // Use all performance cores
    SinglePerf,        // Use 1 performance core
    HybridPerfLittle,  // Mix of perf + little
    #[allow(dead_code)]
    AllLittle,         // Use all little cores
    #[allow(dead_code)]
    MinimalLittle,     // Use minimal little cores
}

impl ThermalConfig {
    pub fn for_soc_class(soc_class: SocClass) -> Self {
        match soc_class {
            SocClass::Flagship => Self {
                // Flagship can sustain high power longer
                buffer_low_threshold: 3.0,      // Less aggressive buffering
                buffer_high_threshold: 12.0,
                max_high_power_duration: Duration::from_secs(180), // 3 minutes
                cooldown_duration: Duration::from_secs(45),
                use_prime_core: false,          // Skip prime (too hot)
                sustained_mode_cores: CoreSelection::AllPerf,
            },

            SocClass::HighEnd => Self {
                buffer_low_threshold: 2.5,
                buffer_high_threshold: 10.0,
                max_high_power_duration: Duration::from_secs(120), // 2 minutes
                cooldown_duration: Duration::from_secs(60),
                use_prime_core: false,
                sustained_mode_cores: CoreSelection::AllPerf,
            },

            SocClass::MidRange => Self {
                // Need aggressive thermal management (your SD 695 case)
                buffer_low_threshold: 2.0,
                buffer_high_threshold: 10.0,
                max_high_power_duration: Duration::from_secs(90), // 1.5 minutes
                cooldown_duration: Duration::from_secs(90),
                use_prime_core: false,
                sustained_mode_cores: CoreSelection::HybridPerfLittle,
            },

            SocClass::LowEnd => Self {
                // Very conservative
                buffer_low_threshold: 1.5,
                buffer_high_threshold: 8.0,
                max_high_power_duration: Duration::from_secs(60), // 1 minute
                cooldown_duration: Duration::from_secs(120),
                use_prime_core: false,
                sustained_mode_cores: CoreSelection::SinglePerf,
            },
        }
    }
}
