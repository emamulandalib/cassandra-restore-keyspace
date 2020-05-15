# Restore Cassandra KeySpaces

This repository contains code to restore data to cassandra after taking the snapshot using **nodetool**.

#### Usage

- Download the latest release from [here](https://github.com/emamulandalib/cassandra-restore-keyspace/releases)
- `chmod +x cassandra_restore_keyspace`
- `touch config.json`
- Place the code into `config.json`. Modify with proper values.
```$xslt
{
  "cassandra_data_dir": "/path/to/cassandra/data",
  "tag": "example",
  "destination": "/path/to/restore",
  "key_space": "key_space_name"
}
```
- Run: `./cassandra_restore_keyspace --config config.json`
- You will get structure like below into the destination folder.
```$xslt
.
└── keyspace
    ├── table1
    │   ├── manifest.json
    │   ├── md-2-big-CompressionInfo.db
    │   ├── md-2-big-Data.db
    │   ├── md-2-big-Digest.crc32
    │   ├── md-2-big-Filter.db
    │   ├── md-2-big-Index.db
    │   ├── md-2-big-Statistics.db
    │   ├── md-2-big-Summary.db
    │   ├── md-2-big-TOC.txt
    │   └── schema.cql
    ├── table2
    │   ├── manifest.json
    │   ├── md-1-big-CompressionInfo.db
    │   ├── md-1-big-Data.db
    │   ├── md-1-big-Digest.crc32
    │   ├── md-1-big-Filter.db
    │   ├── md-1-big-Index.db
    │   ├── md-1-big-Statistics.db
    │   ├── md-1-big-Summary.db
    │   ├── md-1-big-TOC.txt
    │   └── schema.cql
```
- For any kind of errors check message properly.
- For possible options: `./cassandra_restore_keyspace -h`