## Get user

- **URL**

  `/v1/users`

- **Method:**

  `GET`

- **URL Params**

  **Required:**

  `id=[integer]`

- **Data Params**

  None

- **Success Response:**

  - **Code:** 200 <br />
    **Content:**
    ```json
    {
      "id": "1",
      "firstname": "bob",
      "lastname": "smith",
      "credit_limit": "2000"
    }
    ```

- **Error Response:**

  - **Code:** `503` <br />
    **Content:**

    ```json
    upstream connect error or disconnect/reset before headers. reset reason: remote reset
    ```

- **Sample Call:**

  `curl -v 'localhost:50000/v1/users?id=1'`

## Get orders

- **URL**

  `/v1/orders`

- **Method:**

  `GET`

- **URL Params**

  **Required:**

  `userid=[integer]`

- **Data Params**

  None

- **Success Response:**

  - **Code:** 200 <br />
    **Content:**
    ```json
    {
      "orders": [
        {
          "id": "1",
          "userid": "1",
          "product": "Laptop",
          "total": 429.99
        },
        {
          "id": "2",
          "userid": "1",
          "product": "Dryer",
          "total": 680
        }
      ]
    }
    ```

- **Error Response:**

  - **Code:** `503` <br />
    **Content:**

    ```json
    upstream connect error or disconnect/reset before headers. reset reason: remote reset
    ```

- **Sample Call:**

  `curl -v 'localhost:50000/v1/orders?userid=1'`

## Place order

- **URL**

  `/v1/orders`

- **Method:**

  `POST`

- **Data Params**

  ```json
  {
    "userid": 1,
    "product": "Fridge",
    "total": 500
  }
  ```

- **Success Response:**

  - **Code:** 200 <br />
    **Content:**

    ```json
    {
      "status": "SUCCESS"
    }
    ```

- **Error Response:**

  - **Code:** `400` <br />
    **Content:**

    ```json

    ```

    **Headers:**

    ```json
    {
      "content-type": "application/grpc",
      "grpc-status": 9,
      "grpc-message": "Insufficient%20credit"
    }
    ```

    OR

  - **Code:** `503` <br />
    **Content:**

    ```json
    upstream connect error or disconnect/reset before headers. reset reason: remote reset
    ```

- **Sample Call:**

  `curl -v -d '{"userid":1, "product":"Fridge", "total":500}' -H "Content-Type: application/json" -X POST 'localhost:50000/v1/orders'`
