use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::Rng;

/// Get a random tech fact
#[poise::command(slash_command, prefix_command)]
pub async fn random_fact(ctx: Context<'_>) -> Result<(), Error> {
    let facts = vec![
        "The first computer bug was an actual bug - a moth found trapped in a Harvard Mark II computer in 1947!",
        "The term 'debugging' was coined by Grace Hopper when she found the actual bug in the computer.",
        "Rust was originally a side project by Mozilla employee Graydon Hoare in 2010.",
        "Discord was originally called 'Discord' because it was meant to bring harmony to gaming communication.",
        "The first 1GB hard drive, IBM 3380, was released in 1980 and cost $40,000!",
        "Rust's mascot is called 'Ferris' and is a crab! 🦀",
        "The fastest supercomputer can perform over 1 exaflop (1,000,000,000,000,000,000 calculations per second)!",
        "There are more possible games of chess than atoms in the observable universe.",
        "The first computer programmer was Ada Lovelace in 1843, before computers even existed!",
        "WiFi was invented by accident while trying to detect black holes.",
        "The @ symbol was used in email for the first time in 1971 by Ray Tomlinson.",
        "Python was named after Monty Python's Flying Circus, not the snake!",
    ];
    
    let mut rng = rand::thread_rng();
    let fact = facts[rng.gen_range(0..facts.len())];
    
    let embed = serenity::CreateEmbed::new()
        .title("🤓 Random Tech Fact")
        .description(fact)
        .color(0x7289da)
        .timestamp(chrono::Utc::now())
        .footer(serenity::CreateEmbedFooter::new("2tarAPI | Fun Facts"));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    Ok(())
}

/// Generate a system-themed joke
#[poise::command(slash_command, prefix_command)]
pub async fn system_joke(ctx: Context<'_>) -> Result<(), Error> {
    let jokes = vec![
        ("Why do programmers prefer dark mode?", "Because light attracts bugs! 🐛"),
        ("How many programmers does it take to change a light bulb?", "None. That's a hardware problem! 💡"),
        ("Why did the developer go broke?", "Because they used up all their cache! 💰"),
        ("What's a computer's favorite beat?", "An algo-rhythm! 🎵"),
        ("Why don't programmers like nature?", "It has too many bugs! 🌿"),
        ("How do you comfort a JavaScript bug?", "You console it! 🤗"),
        ("Why did the CPU break up with the RAM?", "Because it had commitment issues! 💔"),
        ("What do you call a programmer from Finland?", "Nerdic! 🇫🇮"),
        ("Why did the database administrator leave his wife?", "She had one-to-many relationships! 💍"),
        ("What's the object-oriented way to become wealthy?", "Inheritance! 💎"),
    ];
    
    let mut rng = rand::thread_rng();
    let (setup, punchline) = &jokes[rng.gen_range(0..jokes.len())];
    
    let embed = serenity::CreateEmbed::new()
        .title("😂 System Joke")
        .field("Setup", setup, false)
        .field("Punchline", punchline, false)
        .color(0xffd700)
        .timestamp(chrono::Utc::now())
        .footer(serenity::CreateEmbedFooter::new("2tarAPI | Comedy Central"));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    Ok(())
}

/// 🎲 Roll a dice with custom sides (surprise feature!)
#[poise::command(slash_command, prefix_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "Number of sides on the dice (default: 6)"] sides: Option<u32>,
    #[description = "Number of dice to roll (default: 1)"] count: Option<u32>,
) -> Result<(), Error> {
    let sides = sides.unwrap_or(6).max(2).min(1000); // Between 2-1000 sides
    let count = count.unwrap_or(1).max(1).min(10);   // Between 1-10 dice
    
    let mut rng = rand::thread_rng();
    let mut results = Vec::new();
    let mut total = 0;
    
    for _ in 0..count {
        let roll = rng.gen_range(1..=sides);
        results.push(roll);
        total += roll;
    }
    
    let embed = serenity::CreateEmbed::new()
        .title("🎲 Dice Roll Results")
        .field("Dice", format!("{}d{}", count, sides), true)
        .field("Results", results.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(", "), true)
        .field("Total", total.to_string(), true)
        .color(0xff6b6b)
        .timestamp(chrono::Utc::now())
        .footer(serenity::CreateEmbedFooter::new("2tarAPI | Random Generator"));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    Ok(())
}

/// 🪙 Flip a coin (or multiple coins!)
#[poise::command(slash_command, prefix_command)]
pub async fn coinflip(
    ctx: Context<'_>,
    #[description = "Number of coins to flip (default: 1)"] count: Option<u32>,
) -> Result<(), Error> {
    let count = count.unwrap_or(1).max(1).min(50); // Between 1-50 coins
    
    let mut rng = rand::thread_rng();
    let mut heads = 0;
    let mut tails = 0;
    let mut results = Vec::new();
    
    for _ in 0..count {
        let flip = rng.gen_bool(0.5);
        if flip {
            heads += 1;
            results.push("🟡 Heads");
        } else {
            tails += 1;
            results.push("⚫ Tails");
        }
    }
    
    let result_text = if count <= 10 {
        results.join(", ")
    } else {
        format!("🟡 {} Heads, ⚫ {} Tails", heads, tails)
    };
    
    let embed = serenity::CreateEmbed::new()
        .title("🪙 Coin Flip Results")
        .field("Coins Flipped", count.to_string(), true)
        .field("Results", result_text, false)
        .color(0xffd700)
        .timestamp(chrono::Utc::now())
        .footer(serenity::CreateEmbedFooter::new("2tarAPI | Random Generator"));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    Ok(())
}

/// 🎱 Ask the magic 8-ball a question (surprise interactive feature!)
#[poise::command(slash_command, prefix_command)]
pub async fn magic8ball(
    ctx: Context<'_>,
    #[description = "Your question for the magic 8-ball"] question: String,
) -> Result<(), Error> {
    let responses = vec![
        // Positive responses
        ("It is certain", 0x00ff00),
        ("It is decidedly so", 0x00ff00),
        ("Without a doubt", 0x00ff00),
        ("Yes definitely", 0x00ff00),
        ("You may rely on it", 0x00ff00),
        ("As I see it, yes", 0x00ff00),
        ("Most likely", 0x00ff00),
        ("Outlook good", 0x00ff00),
        ("Yes", 0x00ff00),
        ("Signs point to yes", 0x00ff00),
        
        // Neutral responses
        ("Reply hazy, try again", 0xffff00),
        ("Ask again later", 0xffff00),
        ("Better not tell you now", 0xffff00),
        ("Cannot predict now", 0xffff00),
        ("Concentrate and ask again", 0xffff00),
        
        // Negative responses
        ("Don't count on it", 0xff0000),
        ("My reply is no", 0xff0000),
        ("My sources say no", 0xff0000),
        ("Outlook not so good", 0xff0000),
        ("Very doubtful", 0xff0000),
    ];
    
    let mut rng = rand::thread_rng();
    let (response, color) = &responses[rng.gen_range(0..responses.len())];
    
    let embed = serenity::CreateEmbed::new()
        .title("🎱 Magic 8-Ball")
        .field("Your Question", &question, false)
        .field("The Magic 8-Ball Says...", response, false)
        .color(*color)
        .timestamp(chrono::Utc::now())
        .footer(serenity::CreateEmbedFooter::new("2tarAPI | Fortune Teller"));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    Ok(())
}
