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
![Songs page](https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/songs_page.png)

### Albums page

View all the albums in the database.
![Albums page](https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/albums_page.png)

### Artists page

View all the artists in the database.
![Artists page](https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/artists_page.png)

### Create artist

Page to create a new artist.
![Create artist](https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/create_artist.png)

### Edit artist

Page to edit the attributes of an artist currently in the database.
![Edit artist](https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/edit_artist.png)

### Artist details

Page that shows all the details related to an artist, including its attributes and its discography.
![Artist details](https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/artist_details.png)

## How to run the app

### Requirements

You will need to have the following installed:

- Rust toolchain (cargo)
- [Trunk](https://trunkrs.dev/)
- PostgreSQL

1. Clone this repository
1. Create an empty database and put its URL in the `.env` file (should be `postgres://username:password@localhost:5432/database_name`)
1. Run the backend server with `cargo run`
1. Run the frontend server with `trunk serve --open`
