
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 [init|drop|truncate]"
    exit 1
fi

case "$1" in
    "init")
        echo "init db..."
        psql -h localhost -p 5434 -U postgres -d postgres < db_init.sql
        ;;
    "drop")
        echo "drop table..."
        psql -h localhost -p 5434 -U postgres -d postgres < db_drop.sql
        ;;
    "truncate")
        echo "truncate..."
        psql -h localhost -p 5434 -U postgres -d postgres < db_truncate.sql
        ;;
    *)
        echo "unknown params: $1"
        echo "Usage: $0 [init|drop|truncate]"
        exit 1
        ;;
esac