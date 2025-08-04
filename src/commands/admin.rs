use crate::{Context, Error, Data};
use poise::serenity_prelude as serenity;
use crate::utils::format_uptime;

/// Show bot uptime
#[poise::command(slash_command, prefix_command)]
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let uptime_duration = chrono::Utc::now() - data.start_time;
    let uptime_seconds = uptime_duration.num_seconds() as u64;
    
    let embed = serenity::CreateEmbed::new()
        .title("⏰ Bot Uptime")
        .description(format!(
            "**2tarAPI has been running for:**\n```{}```\n**Started at:** {}",
            format_uptime(uptime_seconds),
            data.start_time.format("%Y-%m-%d %H:%M:%S UTC")
        ))
        .color(0x9932cc)
        .timestamp(chrono::Utc::now())
        .footer(serenity::CreateEmbedFooter::new("2tarAPI | System Stats"));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    Ok(())
}

/// Show command usage statistics
#[poise::command(slash_command, prefix_command)]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let command_count = data.command_count.lock().unwrap();
    
    let total_commands: u64 = command_count.values().sum();
    let uptime_duration = chrono::Utc::now() - data.start_time;
    let uptime_hours = uptime_duration.num_hours() as f64;
    let commands_per_hour = if uptime_hours > 0.0 {
        total_commands as f64 / uptime_hours
    } else {
        0.0
    };
    
    let mut stats_text = String::new();
    if command_count.is_empty() {
        stats_text.push_str("No commands have been used yet!");
    } else {
        // Sort commands by usage count
        let mut sorted_commands: Vec<_> = command_count.iter().collect();
        sorted_commands.sort_by(|a, b| b.1.cmp(a.1));
        
        for (command, count) in sorted_commands.iter().take(10) {
            stats_text.push_str(&format!("**{}:** {} uses\n", command, count));
        }
    }
    
    let embed = serenity::CreateEmbed::new()
        .title("📊 Bot Statistics")
        .field("Total Commands", total_commands.to_string(), true)
        .field("Commands/Hour", format!("{:.1}", commands_per_hour), true)
        .field("Uptime", format_uptime(uptime_duration.num_seconds() as u64), true)
        .field("Command Usage", if stats_text.is_empty() { "No data yet".to_string() } else { stats_text }, false)
        .color(0x00ced1)
        .timestamp(chrono::Utc::now())
        .footer(serenity::CreateEmbedFooter::new("2tarAPI | Analytics"));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    // Update command count
    let command_name = ctx.command().name.clone();
    let mut command_count = data.command_count.lock().unwrap();
    *command_count.entry(command_name).or_insert(0) += 1;
    
    Ok(())
}
