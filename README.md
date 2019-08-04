# tigum-api

Backend for tigum - built with Rust

## V1 Routes

## Data Types:

Topic - A topic is an object for storing all of the notes a user has created for that topic.

```json
[
  {
    "topic_id": "u64",
    "title": "string",
    "date_created": "string"
  }
]
```

Note - A note is an object that stores resources in order of how the user has created or updated them.

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

Resource - A resource can be user generated or created by Sciob (Tigum extension for web clipping). It holds a content content_type and the content itself in parsed html.

```json
{
  "resource_id": "u64",
  "content_type": "TEXT",
  "content": "string (parsed html)"
}
```

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
    "date_created": "string"
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
