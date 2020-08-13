# Simple Bible API

Restful Bible API built using Rust and DynamoDB.

## Data Source

Json files sourced from [here](https://github.com/rkazakov/usfm2json/tree/master/json)

## DyanmoDB setup

I prefer using local DynamoDB for testing purposes. To setup your local DynamoDB, follow the [instructions](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DynamoDBLocal.DownloadingAndRunning.html)

## Preparing the environment

Create a `.env` in your project root and add `endpoint` for your Local DynamoDB

```text
endpoint=http://localhost:9090/
```

Build the project

```shell
cargo build
```

## Creating tables for translations and loading the text

The api is designed for one table per translation.

To create the table

```shell
cargo run --bin create <translation_id>

# Creating for KJV
# cargo run --bin create kjv
```

Load the data

```shell
cargo run --bin load <translation_id> <file_path>

# Loading KJV
# cargo run --bin load bibles/kjv.json
```

## Running the api

```shell
cargo run --bin bible_api
```

## Quering the API

### Get complete chapter

```shell
# curl http://127.0.0.1:9000/search/<translation_id>/<book_id>/<chapter>

curl http://127.0.0.1:9000/search/asv/Gen/1
```

```json
[
    {
        "chapter": 1,
        "verse": 1,
        "text": "In the beginning God created the heavens and the earth.",
        "translation_id": "ASV",
        "book_id": "Gen",
        "book_name": "Genesis"
    },
    {
        "chapter": 1,
        "verse": 2,
        "text": "And the earth was waste and void; and darkness was upon the face of the deep: and the Spirit of God moved upon the face of the waters",
        "translation_id": "ASV",
        "book_id": "Gen",
        "book_name": "Genesis"
    },
    {
        "chapter": 1,
        "verse": 3,
        "text": "And God said, Let there be light: and there was light.",
        "translation_id": "ASV",
        "book_id": "Gen",
        "book_name": "Genesis"
    },
    ...
]
```

### Filtering verses

#### All verses starting from

```shell
# curl http://127.0.0.1:9000/search/<translation_id>/<book_id>/<chapter>?from=starting_verse_number

curl http://127.0.0.1:9000/search/asv/Gen/1?from=2
```

#### All verses until

```shell
# curl http://127.0.0.1:9000/search/<translation_id>/<book_id>/<chapter>?to=ending_verse_number

curl http://127.0.0.1:9000/search/asv/Gen/1?to=4
```

#### All verses between

```shell
# curl http://127.0.0.1:9000/search/<translation_id>/<book_id>/<chapter>?from=starting_verse_number&to=ending_verse_number

curl http://127.0.0.1:9000/search/asv/Gen/1?from=4&to=8
```
