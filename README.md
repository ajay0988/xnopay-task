# Task Architecture
![xnopay](https://user-images.githubusercontent.com/64467997/147384716-3a33ffe2-8477-4d14-86b1-0b02dfc327f4.png)

# Xnopay Task

I have created REST API in ACTIX-WEB framework (developed using RUST) and MongoDB ATLAS as database.


## Deployment of API's  locally
## Note 
1. System must have rust installed
   To install run  this cmd

  curl -sSf https://sh.rustup.rs | sh

2. System must have gcc


To deploy this project run

```bash
  git clone https://github.com/ajay0988/xnopay-task.git
```

```bash
   cd xnopay-task
```
```bash
   cd xnopay-task
```
```bash
   cd cargo run 
```

## API's for memories

#### Get all users details

```http
  GET /user
```

#### Get a single user detail

```http
  GET /user/:id
```
#### Create a user

```http
  POST /user


  Request body in JSON
  {
  "name": "Ajay",
  "uid": 2345,
  "salary": 4523.67,
  "gender": "Male"
  }


  Respnse body


  {
  "_id": "id provided by mongoDB"
   "name": "Ajay",
  "uid": 2345,
  "salary": 4523.67,
  "gender": "Male"
  }
```
#### Update a user detail

```http
  PUT /user/:id
```

#### Delete a user detail

```http
  DELETE /user/:id
```




## Authors

- [@ajaykumar](https://github.com/ajay0988/)

