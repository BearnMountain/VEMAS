
# docker build -t vem-tui .
# docker run -it -v $Project vivado-image

docker compose up -d
docker compose attach vem
