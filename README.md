# Amplitude

This is amplitude! A (currently WIP) website for creating and assigning coding
exercises.

## Images

[media/preview-code.png]
[media/preview-login.png]
[media/preview-rich.png]

## Setup for Development (Linux)

### Backend

To run the backend rust server requires the rust toolchain, postgresql, and
docker.

First create a `.env` file in the root of the project. Here you will need to
provide the connection string for the database and the name for your database 
which should look something like:

```ini
# .env
DATABASE_URL = "postgres://user:pass@127.0.0.1:5432/amplitude_development"
```

Note that this will require postgres to be configured properly. For fedora,
instructions on doing so are available [here](https://docs.fedoraproject.org/en-US/quick-docs/postgresql/).
If you are getting ident errors, a fix is provided there.

```sh
# or however you start the docker daemon
sudo systemctl start docker

sudo systemctl start postgresql

cargo r # create the database, it will tell you to run the next command:
cargo r -p migration -- fresh # ...which will initialize the database

cargo r # and you should be good to go!
```

### Frontend

To run the frontend svelte server you require the npm toolchain and pnpm.

```sh
cd frontend

pnpm i
pnpm run dev # start the frontend server
```