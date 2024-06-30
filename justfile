set dotenv-load

alias r:=run

default: fmt run 

fmt:
    cargo fmt

run:
    cargo r 

# !danger 
refresh-db:
    sea-orm-cli migrate refresh

entiry:
    sea-orm-cli generate entity  -u $DATABASE_URL -o src/entities

docker-build:
    docker build -t zero2prod .

docker-run:
    docker run --network host -d --name zero2prod-server zero2prod