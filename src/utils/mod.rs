use sysinfo::{System, SystemExt, CpuExt, NetworkExt};
use std::process::Command;

pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let cpu_info = sys.cpus().first().map(|cpu| CpuInfo {
        brand: cpu.brand().to_string(),
        frequency: cpu.frequency(),
        usage: cpu.cpu_usage(),
    }).unwrap_or_default();
    
    let memory = MemoryInfo {
        total: sys.total_memory(),
        available: sys.available_memory(),
        used: sys.used_memory(),
    };
    
    let kernel_version = sys.kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let os_version = sys.long_os_version().unwrap_or_else(|| "Unknown".to_string());
    
    SystemInfo {
        cpu: cpu_info,
        memory,
        kernel_version,
        os_version,
        uptime: sys.uptime(),
    }
}

pub async fn get_public_ip() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::get("https://api.ipify.org").await?;
    let ip = response.text().await?;
    Ok(ip)
}

pub fn get_local_ip() -> Option<String> {
    local_ip_address::local_ip().ok().map(|ip| ip.to_string())
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub kernel_version: String,
    pub os_version: String,
    pub uptime: u64,
}

#[derive(Debug, Clone, Default)]
pub struct CpuInfo {
    pub brand: String,
    pub frequency: u64,
    pub usage: f32,
}

#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub available: u64,
    pub used: u64,
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}

pub fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, minutes, secs)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}
