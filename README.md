# 2tarAPI - Discord Bot in Rust 🦀

My first Remaking of "StarAPI" a simple API / Discord Bot with some quirks and gismos then and there.

## Features

- **🖥️ Server Info**: Comprehensive system monitoring including:
  - RAM usage and availability
  - CPU information and usage
  - Kernel and OS information  
  - Public and local IP addresses
  - System uptime

- **⚡ API Health**: Real-time health monitoring with:
  - Response time measurement
  - Discord latency checking
  - Memory usage tracking
  - Disk health status
  - Overall system health scoring

- **🎉 Fun Stuff**: 
  - Random tech facts
  - System-themed jokes
  - Bot uptime tracking
  - Command usage statistics

## 🚀 Deployment Options

### How to run the API / Discord Bot

Setup Rust in your environment (this one is for linux):

1. **Clone repository**:
   ```bash
   git clone https://github.com/O4bit/2tarAPI.git
   cd 2tarAPI
   ```

2. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Create your Discord application:**
   - Go to [Discord Developer Portal](https://discord.com/developers/applications)
   - Create a new application
   - Go to the "Bot" section
   - Create a bot and copy the token

4. **Set up environment variables:**
   ```bash
   cp .env.example .env
   # Edit .env and add your Discord bot token
   ```

5. **Build and run:**
   ```bash
   cargo run
   ```

## Commands

### Slash Commands
- `/server_info` - Get comprehensive server information
- `/health` - Check API health and performance metrics
- `/random_fact` - Get a random tech fact
- `/system_joke` - Get a system-themed joke
- `/uptime` - Show bot uptime
- `/stats` - Show command usage statistics

### Prefix Commands (!)
All slash commands are also available with the `!` prefix:
- `!server_info`
- `!health` 
- `!random_fact`
- `!system_joke`
- `!uptime`
- `!stats`

## Contributing

Any contributions are appreciated

## License

This project is licensed under the GNU General public V3 license

---

*Built by [O4bit](https://github.com/O4bit) with ❤️ and 🦀 Rust*
