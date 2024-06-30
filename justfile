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

