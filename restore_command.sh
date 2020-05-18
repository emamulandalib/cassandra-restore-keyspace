#!/bin/bash

user='user'
password='password'
host=localhost

restore_from_csv() {
    key_space=$1
    tables=$2[@]
    tables=("${!tables}")

    echo "Creating Keyspace..."
    cqlsh -u $user -p $password -e "DROP KEYSPACE $key_space"
    cqlsh -u $user -p $password -f $key_space/schema.cql

    for table in "${tables[@]}"; do
        echo "Processing KeySpace: $key_space, Table: $table"
        cqlsh -u $user -p $password -e "COPY $key_space.$table FROM '$key_space/$table.csv' WITH HEADER = true AND NULL = ' '"
        echo "Import successful. KeySpace: $key_space, Table: $table"
        echo -e "====================================================\n"
    done
}

tables=("table1" "table2")
key_space='keyspace'
backup_to_csv "$key_space" tables