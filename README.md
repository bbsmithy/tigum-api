# tigum-api

Backend for tigum - built with Rust

## Data Types:

Topic - A topic is an object for storing all of the notes a user has created for that topic.

```json
[
  {
    "topic_id": "i32",
    "title": "string",
    "date_created": "string"
    "topic_content": Array<note_id(i32)>
  }
]
```

Note - A note is an object that stores resources in order of how the user has created or updated them.

```json
{
  "note_title": "string (optional)",
  "note_id": "i32",
  "date_created": "string",
  "note_content": Array<resource_id(i32)>
}
```

Resource - A resource can be user generated or created by Sciob (Tigum extension for web clipping). It holds a content content_type and the content itself in parsed html.

```json
{
  "resource_id": "i32",
  "content_type": "TEXT",
  "date_created": "string",
  "content": "string (parsed html)"
}
```

## V1 Routes

### GET /topics

Retreives a list of topics created by that user

Headers:

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
    "date_created": "string"
  }
]
```

### GET /topics/<topic_id>

Retreives a single topic

Headers:

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

### POST /notes

Retrieves a list of notes on a topic

Headers:

```json
{
  "user_id": "string"
}
```

Request Body:

```json
{
  "topic_id": "string"
}
```

Response Body:

```json
[
  {
    "note_title": "string (optional)",
    "note_id": "u64",
    "note_content": [
      {
        "content_type": "TEXT",
        "resource_id": "u64",
        "content": "string (parsed html)"
      },
      {
        "content_type": "VIDEO",
        "resource_id": "u64",
        "content": "string (parsed html)"
      },
      {
        "content_type": "ARTICLE_SNIPPET",
        "resource_id": "u64",
        "content": "string (parsed html)"
      },
      {
        "content_type": "CODE",
        "resource_id": "u64",
        "content": "string (parsed html)"
      },
      {
        "content_type": "IMAGES",
        "resource_id": "u64",
        "content": "string(parsed html)"
      },
      {
        "content_type": "DOCUMENTS",
        "resource_id": "u64",
        "content": "string (parsed html)"
      },
      {
        "content_type": "EXCERCISES",
        "resource_id": "u64",
        "content": "string (parsed html)"
      }
    ]
  }
]
```

### GET /notes/<note_id>

Headers:

```json
{
  "user_id": "string"
}
```

Response Body:

```json
{
  "note_title": "string (optional)",
  "note_id": "u64",
  "note_content": [
    {
      "content_type": "TEXT",
      "resource_id": "u64",
      "content": "string (parsed html)"
    },
    {
      "content_type": "VIDEO",
      "resource_id": "u64",
      "content": "string (parsed html)"
    },
    {
      "content_type": "ARTICLE_SNIPPET",
      "resource_id": "u64",
      "content": "string (parsed html)"
    },
    {
      "content_type": "CODE",
      "resource_id": "u64",
      "content": "string (parsed html)"
    },
    {
      "content_type": "IMAGES",
      "resource_id": "u64",
      "content": "string(parsed html)"
    },
    {
      "content_type": "DOCUMENTS",
      "resource_id": "u64",
      "content": "string (parsed html)"
    },
    {
      "content_type": "EXCERCISES",
      "resource_id": "u64",
      "content": "string (parsed html)"
    }
  ]
}
```
