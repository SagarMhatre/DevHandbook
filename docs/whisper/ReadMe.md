Ref: https://lablab.ai/t/whisper-api-flask-docker

docker build -t whisper-api .

docker run -p 5000:5000 whisper-api

curl -F "file=@testaudio.mp3" http://localhost:5000/whisper