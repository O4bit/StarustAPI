use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use std::time::Instant;

/// Check API health and response time
#[poise::command(slash_command, prefix_command)]
pub async fn health(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();
    
    // Defer the response to measure actual response time
    ctx.defer().await?;
    
    let response_time = start.elapsed();
    
    // Test various system components
    let discord_latency = ctx.ping().await;
    let memory_usage = get_memory_usage();
    let disk_health = check_disk_health().await;
    
    let health_status = if response_time.as_millis() < 500 && memory_usage < 80.0 {
        ("🟢 Healthy", 0x00ff00)
    } else if response_time.as_millis() < 1000 && memory_usage < 90.0 {
        ("🟡 Degraded", 0xffa500)
    } else {
        ("🔴 Unhealthy", 0xff0000)
    };
    
    let embed = serenity::CreateEmbed::new()
        .title("⚡ API Health Check")
        .color(health_status.1)
        .field("Status", health_status.0, true)
        .field("Response Time", format!("{}ms", response_time.as_millis()), true)
        .field("Discord Latency", format!("{}ms", discord_latency.as_millis()), true)
        .field("Memory Usage", format!("{:.1}%", memory_usage), true)
        .field("Disk Status", disk_health, true)
        .field("API Version", "v2.0.0", true)
        .timestamp(chrono::Utc::now())
        .footer(serenity::CreateEmbedFooter::new("2tarAPI Health Monitor"));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    Ok(())
}

fn get_memory_usage() -> f64 {
    use sysinfo::{System, SystemExt};
    let mut sys = System::new_all();
    sys.refresh_memory();
    
    (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0
}

async fn check_disk_health() -> String {
    use sysinfo::{System, SystemExt, DiskExt};
    let mut sys = System::new_all();
    sys.refresh_disks();
    
    let disks = sys.disks();
    if disks.is_empty() {
        return "❓ Unknown".to_string();
    }
    
    let total_space: u64 = disks.iter().map(|d| d.total_space()).sum();
    let available_space: u64 = disks.iter().map(|d| d.available_space()).sum();
    let usage_percent = ((total_space - available_space) as f64 / total_space as f64) * 100.0;
    
    if usage_percent < 80.0 {
        "🟢 Good"
    } else if usage_percent < 90.0 {
        "🟡 Warning"
    } else {
        "🔴 Critical"
    }.to_string()
}
