# Rust Backend with Warp and Docker Compose

This repository contains a backend application written in Rust using the Warp framework and a Docker Compose configuration to start a PostgreSQL database. The application provides an example of building a RESTful API server with database integration.

## Prerequisites

Before running the application, make sure you have the following dependencies installed on your system:

- Rust (1.66 currently)
- Docker (24.0.2 currently)
- Docker Compose (1.29.2 currently)
- Diesel CLI (1.4.1 currently)

The version of the docker and docker-compose is not as important is the rust and diesel versions.
The version 2.X of diesel has a very large breaking change, making it not compatible with 1.X versions.

## Getting Started

To get started with the backend application and PostgreSQL database, follow these steps:

1. Ensure you have the prerequisites installed on your system.

2. Clone this repository to your local machine and then go the back folder:

3. Open the project in your preferred Rust development environment (VS Code is the best).

4. export the .env file into your environment variables
   ```bash
   export $(cat .env | xargs)

5. Start the docker-compose and run the migrations:
    ```bash
    docker-compose up -d
    diesel migration run
    ```

6. Run the following command in the terminal to build, install the dependecies and start the application:

   ```bash
   cargo run
   ```
Please note that the port used (3030) is defined in the .env file

## Customization

You can customize the Card Manager application by making changes to the codebase:

- Add new routes and handlers: Open `filters.rs` and add new routes and handlers to support the desired functionality.
- To update the schema.rs file, run the following command:
  ```bash
  diesel migration run
  ```
- To add a new struct to parse data, put it in the `domain.rs` file.

- To add a database call, put it in the `database.rs` file, you can also expand it by implementing `DBAccessManager` in another file.

- To add database migrations add a script in the migration folders with the command
  ```bash
  diesel migration generate <migration_name>
  ```
  Then edit the new migration in the `migrations` folder.
  Then to update the database run the command
  ```bash
  diesel migration run
  ```
  This will update the database and the `schema.rs` file.

## Contributions

Contributions to the Card Manager application backend are welcome! If you find any issues or have ideas for enhancements, feel free to open an issue or submit a pull request on the GitHub repository.

## License

The Card Manager application is open-source and released under the [MIT License](LICENSE). You are free to use, modify, and distribute the code as per the terms of the license.

## Credits

The Card Manager application was developed by Gaspard W. (Extragornax) as a demonstration of backend development in rust for the drivenspark exam.
