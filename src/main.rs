use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub struct Data {
    pub start_time: DateTime<Utc>,
    pub command_count: std::sync::Arc<std::sync::Mutex<HashMap<String, u64>>>,
} 

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

mod commands;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load environment variables
    dotenv::dotenv().ok();
    
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Expected DISCORD_TOKEN in environment");
    
    let intents = serenity::GatewayIntents::non_privileged();
    
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::info::server_info(),
                commands::health::health(),
                commands::fun::random_fact(),
                commands::fun::system_joke(),
                commands::fun::roll(),
                commands::fun::coinflip(),
                commands::fun::magic8ball(),
                commands::admin::uptime(),
                commands::admin::stats(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                edit_tracker: Some(std::sync::Arc::new(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                ))),
                ..Default::default()
            },
            on_error: |error| {
                Box::pin(async move {
                    match error {
                        poise::FrameworkError::Command { error, ctx, .. } => {
                            tracing::error!("Error in command `{}`: {:?}", ctx.command().name, error);
                            let _ = ctx.say("An error occurred while running the command.").await;
                        }
                        error => {
                            if let Err(e) = poise::builtins::on_error(error).await {
                                tracing::error!("Error while handling error: {}", e);
                            }
                        }
                    }
                })
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    start_time: Utc::now(),
                    command_count: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
    
    Ok(())
}

async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            tracing::info!("2tarAPI is ready! Logged in as {}", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}
