use crate::{Context, Error};
use crate::utils::{get_system_info, get_public_ip, get_local_ip, format_bytes, format_uptime};
use poise::serenity_prelude as serenity;

/// Get comprehensive server information
#[poise::command(slash_command, prefix_command)]
pub async fn server_info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    
    let system_info = get_system_info();
    let public_ip = get_public_ip().await.unwrap_or_else(|_| "Unknown".to_string());
    let local_ip = get_local_ip().unwrap_or_else(|| "Unknown".to_string());
    
    let embed = serenity::CreateEmbed::new()
        .title("🖥️ Server Information")
        .color(0x00ff00)
        .field(
            "💾 Memory",
            format!(
                "**Total:** {}\n**Used:** {}\n**Available:** {}\n**Usage:** {:.1}%",
                format_bytes(system_info.memory.total),
                format_bytes(system_info.memory.used),
                format_bytes(system_info.memory.available),
                (system_info.memory.used as f64 / system_info.memory.total as f64) * 100.0
            ),
            true,
        )
        .field(
            "🔧 CPU",
            format!(
                "**Model:** {}\n**Frequency:** {} MHz\n**Usage:** {:.1}%",
                system_info.cpu.brand,
                system_info.cpu.frequency,
                system_info.cpu.usage
            ),
            true,
        )
        .field(
            "🌐 Network",
            format!(
                "**Public IP:** {}\n**Local IP:** {}",
                public_ip,
                local_ip
            ),
            true,
        )
        .field(
            "⚙️ System",
            format!(
                "**OS:** {}\n**Kernel:** {}\n**Uptime:** {}",
                system_info.os_version,
                system_info.kernel_version,
                format_uptime(system_info.uptime)
            ),
            false,
        )
        .thumbnail("https://cdn.discordapp.com/emojis/1234567890123456789.png") // You can replace with your bot's icon
        .timestamp(chrono::Utc::now())
        .footer(serenity::CreateEmbedFooter::new("2tarAPI | System Monitor"));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    Ok(())
}
