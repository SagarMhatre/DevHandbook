Ref: https://lablab.ai/t/whisper-api-flask-docker

```sh
docker build -t whisper-api:1.3 .
docker run -p 5000:5000 --name my-whisper-api-13 --rm whisper-api:1.3
```

```rest
curl -F "file=@englishTestAudio.mp3" http://localhost:5000/whisper
```

src - https://audio-lingua.ac-versailles.fr/spip.php?rubrique1&lang=en

```rest
curl -F "file=@french test audio - Elisa - le film WONKA.mp3" http://localhost:5000/whisper-translate
```