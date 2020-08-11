#!/usr/bin/bash

user='user'
password='password'
host=localhost
tables=("table1" "table2")
key_space='keyspace'
 
backup_to_csv() {
    if [ ! -d $key_space ]; then
        mkdir -p "$key_space"
    fi

    echo "Dumping '$key_space' schema"
    cqlsh -u $user -p $password -e "DESCRIBE $key_space" $host > $key_space/schema.cql

    for table in ${tables[@]}; do
        echo "Processing KeySpace: $key_space, Table: $table"
        cqlsh -u $user -p $password -e "COPY $key_space.$table TO STDOUT WITH HEADER = true" $host > $key_space/$table.csv
        echo "CSV created for KeySpace: $key_space, Table: $table"
        echo -e "====================================================\n"
    done
}

backup_to_csv
