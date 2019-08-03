# tigum-api

Backend for tigum - built with Rust

## V1 Routes

### Headers?

### POST /sign-in

### GET /topics

Request Body:

```json
{
  "user_id": "string"
}
```

Retreives a list of topics created by that user

```json

{
    "topic_id": "u64",
    "title": "string",
    "date_created": "string",
    "notes": "Array<note_id>",
    "videos": "Array<video_id>",
    "article_snippets": "Array<article_snippet_id>",
    "code": "Array<code_id>",
    "images": "Array<image_id>",
    "documents": "Array<document_id>",
    "excercises": "Array<exercise_id>"
}

`
```

### GET /notes

Request Body:

```json

```
