# actix-tokio-todo
Simple TODO list API made in rust

## Requirements
- Rust
- psql

## Usage
```
# clone this repo

# Create a .env file
assign host ADDRESS,PORT,DATABASE_URL

# Run the server (Add --release for an optimized build)
cargo run 
```
```
curl -s http://ADDRESS:PORT/
```

# Routes

- `GET` `/api/` -> Status

  **Response:**
  ```
  {
    "status": "Up"
  }
  ```

- `GET` `/api/todos` -> Get todos

  **Response:**
  ```
  [
    {
      id": 1,
     "title": "play",
     "description": "play football tomorrow   evening"
    },
    ...
  ]
  ```

- `POST` `/api/todos` -> Create todo

  **Request Header:**
  ```
  Content-Type: application/json
  ```
  **Request Body:**
  ```
 {
  "title":"shopping",
  "description":"shop some vegetables"
 }
  ```
  **Response:**
  ```
  {
  "id": 8,
  "title": "shopping",
  "description": "shop some vegetables",
  "checked": false
 }
  ```

- `PUT` `/api/todos/{id}` -> Check todo

  **Response:**
  ```
  {
    "result": true
  }
  ```
  Result:
  - `true` -> Checked
  - `false` -> Already checked. Nothing to do.

- `DELETE` `/api/todos/{id}`
    
