# tigum-api

Backend for tigum - built with Rust

## V1 Routes

## Data Types:

Topic - A topic is an object for storing all of the notes a user has created for that topic.
Note - A note is an object that stores resources in order of how the user has created or updated them.
Resource - A resource can be user generated or created by Sciob (Tigum extension for web clipping). It holds

### GET /topics

Retreives a list of topics created by that user

Request Body:

```json
{
  "user_id": "string"
}
```

Response Body:

```json
[{
  "topic_id": "u64",
  "title": "string",
  "date_created": "string"
}}
```

### GET /topics/?topic_id=1234

Retreives a single topic

Request Body:

```json
{
  "user_id": "string"
}
```

Response Body:

```json
{
  "topic_id": "u64",
  "title": "string",
  "date_created": "string"
}
```

### GET /notes/?topic_id=1234

Retrieves a list of notes for a specific topic

Request Body:

```json
{
  "limit_from": 10,
  "limit_to": 20
}
```

Response Body:

```json
{
  "note_title": "string (optional)",
  "note_id": "u64",
  "note_content": [
    {
      "type": "TEXT",
      "id": "u64",
      "content": "string (parsed html)"
    },
    {
      "type": "VIDEO",
      "id": "u64",
      "content": "string (parsed html)"
    }
  ]
}
```
