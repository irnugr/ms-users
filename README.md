# ms-users

`ms-users` is a microservice built using Rust that handles user management features such as registering users, resetting passwords, activating/inactivating users, and more. The service runs on port 8010.

## Features

- **Register User**: Create a new user account.
- **Reset User Password**: Reset the password for an existing user.
- **Activate User**: Activate a user account.
- **Inactivate User**: Inactivate a user account.

## Requirements

- Rust (latest stable version)
- Cargo (Rust package manager)

## Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/yourusername/ms-users.git
    cd ms-users
    ```

2. **Build the project**:
    ```sh
    cargo build --release
    ```

## Usage

1. **Run the service**:
    ```sh
    cargo run --release
    ```

    The service will start and listen on port 8010.

## API Endpoints

### Register User

- **Endpoint**: `/register`
- **Method**: `POST`
- **Request Body**:
    ```json
    {
        "username": "string",
        "password": "string",
        "email": "string"
    }
    ```
- **Response**:
    ```json
    {
        "message": "User registered successfully",
        "user_id": "string"
    }
    ```

### Reset User Password

- **Endpoint**: `/reset-password`
- **Method**: `POST`
- **Request Body**:
    ```json
    {
        "email": "string",
        "new_password": "string"
    }
    ```
- **Response**:
    ```json
    {
        "message": "Password reset successfully"
    }
    ```

### Activate User

- **Endpoint**: `/activate`
- **Method**: `POST`
- **Request Body**:
    ```json
    {
        "user_id": "string"
    }
    ```
- **Response**:
    ```json
    {
        "message": "User activated successfully"
    }
    ```

### Inactivate User

- **Endpoint**: `/inactivate`
- **Method**: `POST`
- **Request Body**:
    ```json
    {
        "user_id": "string"
    }
    ```
- **Response**:
    ```json
    {
        "message": "User inactivated successfully"
    }
    ```

## Configuration

- The service runs on port 8010 by default. You can change the port by modifying the `main.rs` file.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
