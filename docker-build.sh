
docker build -t alert-listener .
docker run --rm --env-file .env -p 8888:8888 alert-listener