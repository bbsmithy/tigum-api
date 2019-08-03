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
[
  {
    "topic_id": "u64",
    "title": "string",
    "date_created": "string",
    "notes": [1232, 1262, 1236]
  }
]
```

### GET /topics/<topic_id>

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

### GET /notes/<topic_id>

Retrieves a list of notes

Request Body:

```json
{}
```

Response Body:

```json
[
  {
    "note_title": "string (optional)",
    "note_id": "u64",
    "note_content": [
      {
        "type": "TEXT",
        "resource_id": "u64",
        "content": "string (parsed html)"
      },
      {
        "type": "VIDEO",
        "resource_id": "u64",
        "content": "string (parsed html)"
      },
      {
        "type": "ARTICLE_SNIPPET",
        "resource_id": "u64",
        "content": "string (parsed html)"
      },
      {
        "type": "CODE",
        "resource_id": "u64",
        "content": "string (parsed html)"
      },
      {
        "type": "IMAGES",
        "resource_id": "u64",
        "content": "string(parsed html)"
      },
      {
        "type": "DOCUMENTS",
        "resource_id": "u64",
        "content": "string (parsed html)"
      },
      {
        "type": "EXCERCISES",
        "resource_id": "u64",
        "content": "string (parsed html)"
      }
    ]
  }
]
```

### GET /note/<note_id>

Request Body:

```json
{}
```

Response Body:

```json
{
  "note_title": "string (optional)",
  "note_id": "u64",
  "note_content": [
    {
      "type": "TEXT",
      "resource_id": "u64",
      "content": "string (parsed html)"
    },
    {
      "type": "VIDEO",
      "resource_id": "u64",
      "content": "string (parsed html)"
    },
    {
      "type": "ARTICLE_SNIPPET",
      "resource_id": "u64",
      "content": "string (parsed html)"
    },
    {
      "type": "CODE",
      "resource_id": "u64",
      "content": "string (parsed html)"
    },
    {
      "type": "IMAGES",
      "resource_id": "u64",
      "content": "string(parsed html)"
    },
    {
      "type": "DOCUMENTS",
      "resource_id": "u64",
      "content": "string (parsed html)"
    },
    {
      "type": "EXCERCISES",
      "resource_id": "u64",
      "content": "string (parsed html)"
    }
  ]
}
```
