# Rollux

Rollux is a simple dice rolling bot for discord

# How to run

1. Make a discord bot [here](https://discordapp.com/developers/applications/).
   - Here is a [tutorial](https://www.digitaltrends.com/gaming/how-to-make-a-discord-bot/) on how to make a discord bot. 
2. Make a auth.json file and add the following to it:

```json
{
    "token": "{your token}"
}
```
3. Install node.js and npm, then run the following command: `npm install`
4. Run it with `node bot.js`

# Running with docker

1. Build docker image with a tag
   Example: `docker build -t rollux .`

2. Add authentication by either:
   1. Mount `/data` directory in container with your auth.json file
   2. Pass the environment variable `DISCORD_TOKEN` with your discord token

## Example docker-compose file:

For auth.json method:
```
version: '2'

services:
  rollux:
    image: 'rollux'
    volumes:
      - ./data:/data
```

For environment variable:
```
version: '2'

services:
  rollux:
    image: 'rollux'
    environment:
      - DISCORD_TOKEN={your token}
```
