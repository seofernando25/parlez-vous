use std::time::{Duration, Instant};
use super::{config::{CoreSelection, ThermalConfig}, topology::{CpuTopology, SocClass}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThermalMode {
    ColdStart,
    Sustained,
    Efficiency,
    Emergency,
}

pub struct UnifiedThermalManager {
    topology: CpuTopology,
    config: ThermalConfig,
    current_mode: ThermalMode,
    
    // Thermal tracking
    high_power_usage_time: Duration,
    last_mode_switch: Instant,
    
    // Performance tracking
    baseline_rtf: Option<f32>,
    recent_rtf: Vec<f32>,
}

impl UnifiedThermalManager {
    pub fn new() -> Self {
        let topology = CpuTopology::detect();
        let config = ThermalConfig::for_soc_class(topology.soc_class);
        
        log::info!("Unified thermal manager initialized:");
        log::info!("  SoC: {:?}", topology.soc_class);
        log::info!("  Prime: {:?}", topology.prime_core);
        log::info!("  Perf: {:?}", topology.perf_cores);
        log::info!("  Little: {:?}", topology.little_cores);
        log::info!("  Config: buffer thresholds {:.1}s-{:.1}s, max power duration {:?}",
            config.buffer_low_threshold,
            config.buffer_high_threshold,
            config.max_high_power_duration
        );
        
        Self {
            topology,
            config,
            current_mode: ThermalMode::ColdStart,
            high_power_usage_time: Duration::ZERO,
            last_mode_switch: Instant::now(),
            baseline_rtf: None,
            recent_rtf: Vec::with_capacity(5),
        }
    }
    
    pub fn update(&mut self, buffer_seconds: f32, current_rtf: f32) -> ThermalMode {
        // Track RTF for throttling detection
        self.recent_rtf.push(current_rtf);
        if self.recent_rtf.len() > 5 {
            self.recent_rtf.remove(0);
        }
        
        if self.baseline_rtf.is_none() && self.recent_rtf.len() == 5 {
            self.baseline_rtf = Some(self.recent_rtf.iter().sum::<f32>() / 5.0);
            log::info!("Baseline RTF established: {:.2}x", self.baseline_rtf.unwrap());
        }
        
        let rtf_degradation = self.detect_throttling();
        let elapsed = self.last_mode_switch.elapsed();
        
        // Track high-power usage
        if matches!(self.current_mode, ThermalMode::ColdStart | ThermalMode::Sustained) {
            self.high_power_usage_time += elapsed;
        }
        
        let new_mode = self.decide_mode(buffer_seconds, rtf_degradation, elapsed);
        
        if new_mode != self.current_mode {
            log::info!(
                "Mode transition: {:?} -> {:?} (buffer: {:.1}s, RTF: {:.2}x, thermal budget: {:?})",
                self.current_mode,
                new_mode,
                buffer_seconds,
                current_rtf,
                self.high_power_usage_time
            );
            
            self.current_mode = new_mode;
            self.last_mode_switch = Instant::now();
            
            // Reset thermal budget when entering efficiency mode
            if matches!(new_mode, ThermalMode::Efficiency) {
                self.high_power_usage_time = Duration::ZERO;
            }
            
            self.apply_affinity().ok();
        }
        
        new_mode
    }
    
    fn decide_mode(&self, buffer_seconds: f32, rtf_degradation: bool, elapsed: Duration) -> ThermalMode {
        match self.current_mode {
            ThermalMode::ColdStart => {
                // Transition based on buffer fill or time limit
                if buffer_seconds > 5.0 || elapsed > Duration::from_secs(30) {
                    ThermalMode::Sustained
                } else {
                    ThermalMode::ColdStart
                }
            }
            
            ThermalMode::Sustained => {
                // Buffer critical - boost back to cold start
                if buffer_seconds < self.config.buffer_low_threshold {
                    ThermalMode::ColdStart
                }
                // Thermal budget exhausted or throttling detected
                else if self.high_power_usage_time > self.config.max_high_power_duration 
                    || rtf_degradation {
                    ThermalMode::Efficiency
                }
                // Buffer healthy - can afford to cool down
                else if buffer_seconds > self.config.buffer_high_threshold {
                    ThermalMode::Efficiency
                } else {
                    ThermalMode::Sustained
                }
            }
            
            ThermalMode::Efficiency => {
                // Buffer running low
                if buffer_seconds < self.config.buffer_low_threshold {
                    ThermalMode::Sustained
                }
                // Cooled down sufficiently
                else if elapsed > self.config.cooldown_duration 
                    && buffer_seconds < self.config.buffer_high_threshold {
                    ThermalMode::Sustained
                }
                // Severe throttling even on little cores
                else if rtf_degradation {
                    ThermalMode::Emergency
                } else {
                    ThermalMode::Efficiency
                }
            }
            
            ThermalMode::Emergency => {
                // Need significant cooling before returning
                if elapsed > Duration::from_secs(120) && !rtf_degradation {
                    ThermalMode::Efficiency
                } else {
                    ThermalMode::Emergency
                }
            }
        }
    }
    
    fn apply_affinity(&self) -> anyhow::Result<()> {
        let cores = self.get_cores_for_mode();
        set_cpu_affinity(&cores)?;
        
        log::info!("Applied {:?} mode on {:?} SoC: {} threads on cores {:?}",
            self.current_mode,
            self.topology.soc_class,
            cores.len(),
            cores
        );
        
        Ok(())
    }
    
    fn get_cores_for_mode(&self) -> Vec<usize> {
        match self.current_mode {
            ThermalMode::ColdStart => {
                // Max performance
                match self.topology.soc_class {
                    SocClass::Flagship | SocClass::HighEnd => {
                        // Use all perf cores (skip prime)
                        self.topology.perf_cores.clone()
                    }
                    SocClass::MidRange | SocClass::LowEnd => {
                        // Use all perf cores
                        self.topology.perf_cores.clone()
                    }
                }
            }
            
            ThermalMode::Sustained => {
                match self.config.sustained_mode_cores {
                    CoreSelection::AllPerf => {
                        self.topology.perf_cores.clone()
                    }
                    CoreSelection::SinglePerf => {
                        vec![self.topology.perf_cores[0]]
                    }
                    CoreSelection::HybridPerfLittle => {
                        // 1 big + 2 little (spreads heat)
                        let mut cores = vec![self.topology.perf_cores[0]];
                        let little_count = 2.min(self.topology.little_cores.len());
                        cores.extend(&self.topology.little_cores[..little_count]);
                        cores
                    }
                    _ => self.topology.perf_cores.clone(),
                }
            }
            
            ThermalMode::Efficiency => {
                // Use little cores based on SoC class
                let count = match self.topology.soc_class {
                    SocClass::Flagship => 4,
                    SocClass::HighEnd => 4,
                    SocClass::MidRange => 4,
                    SocClass::LowEnd => 2,
                };
                
                let available = count.min(self.topology.little_cores.len());
                self.topology.little_cores[..available].to_vec()
            }
            
            ThermalMode::Emergency => {
                // Minimal cores
                let count = 2.min(self.topology.little_cores.len());
                self.topology.little_cores[..count].to_vec()
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_thread_count(&self) -> usize {
        self.get_cores_for_mode().len().max(1)
    }

    fn detect_throttling(&self) -> bool {
        if let Some(baseline) = self.baseline_rtf {
            if self.recent_rtf.len() >= 3 {
                let current_avg = self.recent_rtf.iter().sum::<f32>() / self.recent_rtf.len() as f32;
                // If RTF dropped below 60% of baseline, we're throttling
                return current_avg < baseline * 0.6;
            }
        }
        false
    }
    
    pub fn get_soc_class(&self) -> SocClass {
        self.topology.soc_class
    }

    #[allow(dead_code)]
    pub fn get_current_mode(&self) -> ThermalMode {
        self.current_mode
    }
}

#[cfg(target_os = "android")]
fn set_cpu_affinity(cores: &[usize]) -> anyhow::Result<()> {
    use libc::{cpu_set_t, sched_setaffinity, CPU_SET, CPU_ZERO};
    
    unsafe {
        let mut cpuset: cpu_set_t = std::mem::zeroed();
        CPU_ZERO(&mut cpuset);
        
        for &cpu in cores {
            CPU_SET(cpu, &mut cpuset);
        }
        
        if sched_setaffinity(0, std::mem::size_of::<cpu_set_t>(), &cpuset) != 0 {
            return Err(anyhow::anyhow!("Failed to set CPU affinity"));
        }
    }
    
    Ok(())
}

#[cfg(not(target_os = "android"))]
fn set_cpu_affinity(_cores: &[usize]) -> anyhow::Result<()> {
    Ok(())
}
