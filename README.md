# Rust web app demo

## Table of contents

- Goal of this project
- What the app is about
- Technologies used
  - Backend
  - Frontend
- Screenshots
- How to run the app

This project is a small project to test out web technologies in Rust. I wanted to try out some backend frameworks and some WASM-based frontend frameworks, to create a web app all in Rust!

## Goal of this project

The goal of this project was to make a small-scale web application to try out Rust frameworks. This allowed me to understand better the state of Rust frameworks, to better my skills in Rust, and to have previous experience with these technologies if I want to work on another, fuller web application project in Rust.

The goal of this project _was not_ to make a polished, useful application. For instance, the UI has a theme toggle, but the styling is very rough, the intention was merely to make a functional toggle.

## What the app is about

The actual application I chose to make (which is more of a pretext than anything else), is a discography database. It allows users to add artists, albums and songs, and browse these objects. As the project is small-scale, many features that would be essential if the project's goal was to be useful are not present, such as a search engine.

## Technologies used

### Backend

- [Rocket](https://rocket.rs/)
- [SeaORM](https://www.sea-ql.org/SeaORM/)
- PostgreSQL

### Frontend

- [Yew](https://yew.rs/)
- [Stylist](https://github.com/futursolo/stylist-rs)

## Screenshots

### Songs page

View all the songs in the database.
![Songs page](https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/screenshots/songs_page.png)

### Albums page

View all the albums in the database.
![Albums page](https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/screenshots/albums_page.png)

### Artists page

View all the artists in the database.
![Artists page](https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/screenshots/artists_page.png)

### Create artist

Page to create a new artist.
![Create artist](https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/screenshots/create_artist.png)

### Edit artist

Page to edit the attributes of an artist currently in the database.
![Edit artist](https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/screenshots/edit_artist.png)

### Artist details

Page that shows all the details related to an artist, including its attributes and its discography.
![Artist details](https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/screenshots/artist_details.png)

## How to run the app

The app can be run locally or in Docker containers with Docker Compose.

### Locally

#### Requirements

You will need to have the following installed:

- Rust toolchain (cargo)
- [Trunk](https://trunkrs.dev/)
- PostgreSQL

#### Steps

1. Clone this repository:

```
git clone https://github.com/SpacewaIker/rust-webapp-demo.git
```

2. Create an empty database and put its URL in the `.env` file (should be `postgres://{username}:{password}@localhost:5432/{database_name}`)
3. Run the backend server with `rust-webapp-demo/backend$ cargo run --release`
4. Run the frontend server with `rust-webapp-demo/frontend$ trunk serve --release --open`
5. (Optional) Populate the database by running the `populate_db.sql` script (warning: this will first clear the database):

```
psql -U {username} -f populate_db.sql {database_name}
```

### With Docker Compose

Disclaimer: This method wasn't thoroughly tested and might not work.

#### Requirements

Docker needs to be installed and the daemon running.

#### Steps

1. Clone this repository:

```
git clone https://github.com/SpacewaIker/rust-webapp-demo.git
```

2. Run the containers with:

```
rust-webapp-demo$ docker compose up
```

Note: Downloading the Rust and PostgreSQL images used in the containers and building the code might take a while.
