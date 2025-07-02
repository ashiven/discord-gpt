<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Release](https://img.shields.io/github/v/release/ashiven/discord-gpt)](https://github.com/ashiven/discord-gpt/releases)
[![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/ashiven/discord-gpt)](https://github.com/ashiven/discord-gpt/issues)
[![GitHub Issues or Pull Requests](https://img.shields.io/github/issues-pr/ashiven/discord-gpt)](https://github.com/ashiven/discord-gpt/pulls)
![GitHub Repo stars](https://img.shields.io/github/stars/ashiven/discord-gpt)

</div>

## About

This is a **Discord** bot that was written with [Poise](https://github.com/serenity-rs/poise) to enable users to have simple interactions with **ChatGPT** without having to use the website.

## Getting Started

### Prerequisites

-  Download and install the latest version of [Rust](https://www.rust-lang.org/tools/install).
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

3. Set the **OpenAI** API key and **Discord** bot token environment variables or add them to a `.env` file.

   ```bash
   export OPENAI_API_KEY="your api key" DISCORD_TOKEN="your discord token"
   ```

### Usage

-  Enter the following command to start the bot:

   ```bash
   cargo run --release
   ```

### Interactions

-  `~chat` to have a regular conversation with the bot that maintains its context.

   ```
   User: ~chat My favorite color is blue.

   Bot: That is very interesting!

   User: ~chat What is my favorite color?

   Bot: Your favorite color is blue.
   ```

-  `~summarize` to get a brief summary of your message.

   ```
   User: ~summarize *Long and complicated text*

   Bot: The key points of the text are the following: ...
   ```
- `~session 10 [goals]` to start a 10 minute pomodoro working session, optionally telling **ChatGPT** about your goals for this session. 

  ```
  User: ~session 10 I would like to use this fruitful time in order to build an http server in python.

  Bot: Your 10 minute session has been running for: 

       0m : 39s

  Bot: 🍅🎉 Time's up! Your 1-minute pomodoro session has come to an end. It's break time! 🌟

       Did you manage to build the HTTP server in Python during this short session?
       If not, would you like some help to achieve it in the next one?

       Here's a cute animal video to brighten your break: Funny Animal Video
  ```

---

> GitHub [@ashiven](https://github.com/Ashiven) &nbsp;&middot;&nbsp;
> Twitter [ashiven\_](https://twitter.com/ashiven_)
