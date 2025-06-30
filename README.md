<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Release](https://img.shields.io/github/v/release/ashiven/discord-gpt)](https://github.com/ashiven/discord-gpt/releases)
[![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/ashiven/discord-gpt)](https://github.com/ashiven/discord-gpt/issues)
[![GitHub Issues or Pull Requests](https://img.shields.io/github/issues-pr/ashiven/discord-gpt)](https://github.com/ashiven/discord-gpt/pulls)
![GitHub Repo stars](https://img.shields.io/github/stars/ashiven/discord-gpt)

</div>

## About

This is a discord bot that was written with [poise](https://docs.rs/poise/latest/poise/index.html) to enable users to have simple conversations with **ChatGPT** without having to interact with the website.

## Getting Started

### Prerequisites

-  Download and install the latest version of [Rust](https://www.python.org/downloads/).
-  Register for an [OpenAI](https://platform.openai.com/api-keys) API key and retrieve it.
-  Create a new [Discord bot](https://www.writebots.com/discord-bot-token/) and retrieve a token for it.

### Setup

1. Clone the repository to your local machine as follows:
   ```bash
   git clone https://github.com/ashiven/discord-gpt.git
   ```
   
2. Navigate to the **discord-gpt** directory.

   ```bash
   cd ./discord-gpt
   ```

3. Set the **Discord** bot token and **OpenAI** API key environment variables or add them to a `.env` file

   ```bash
   export OPENAI_API_KEY="your api key" DISCORD_TOKEN="your discord token" 
   ```

### Usage

-  Enter the following command to start the bot:

   ```bash
   cargo run --release
   ```

---

> GitHub [@ashiven](https://github.com/Ashiven) &nbsp;&middot;&nbsp;
> Twitter [ashiven\_](https://twitter.com/ashiven_)
