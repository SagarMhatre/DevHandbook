## Run locally for MCP capability

```sh
git clone https://github.com/FlowiseAI/Flowise.git
cd Flowise 
npm install -g flowise
npx flowise start

```


https://hub.docker.com/r/flowiseai/flowise


https://raw.githubusercontent.com/FlowiseAI/Flowise/refs/heads/main/docker/.env.example = .env
https://raw.githubusercontent.com/FlowiseAI/Flowise/refs/heads/main/docker/docker-compose.yml


```sh

# image: flowiseai/flowise:2.2.7
clear
cd /Users/sagarmhatre/Local-Documents/src/DevHandbook/docs/flowise
docker compose up -d
open http://localhost:3000
# You can bring the containers down by 
# docker compose stop

```

