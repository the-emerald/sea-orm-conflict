https://discord.com/channels/873880840487206962/1071914216560267284

`./postgres.sh` (requires docker, run in another terminal)

`sea-orm-cli migrate refresh && sea-orm-cli generate entity -o entity/src/entities/ && cargo clippy`