## Rust Axum Key-Value Store

This project is a simple, in-memory key-value store implemented in Rust, using the Axum framework. 

### Installation

1.  Clone the repository:
    
    bashCopy code
    
    `git clone https://github.com/yourusername/your-repository-name.git`
    
2.  Navigate to the project directory:
    
    bashCopy code
    
    `cd your-repository-name`
    
3.  Build the project:
    
    bashCopy code
    
    `cargo build`
    
4.  Run the server:
    
    bashCopy code
    
    `cargo run`


### Usage

The key-value store supports several operations, accessible through HTTP requests:

*   **Set a Key-Value Pair**: Send a POST request to `/set/{key}` with the value in the request body.
*   **Get a Value by Key**: Send a GET request to `/get/{key}`.
*   **Delete a Key-Value Pair**: Send a DELETE request to `/delete/{key}`.
*   **List All Keys**: Send a GET request to `/keys`.
*   **Flush the Store**: Send a GET request to `/flush` to remove all key-value pairs.
*   **Store Capacity**: Send a GET request to `/capacity` to get the number of stored key-value pairs.
