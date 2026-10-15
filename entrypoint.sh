#!/bin/sh

# 1. Read Home Assistant configuration while running as root
if [ -f /data/options.json ]; then
  # Parse keys and fallback to empty string if missing or null
  PORT=$(jq --raw-output '.port // ""' /data/options.json)
  DATABASE_URL=$(jq --raw-output '.database_url // ""' /data/options.json)

  [ "$PORT" = "null" ] && PORT=""
  [ "$DATABASE_URL" = "null" ] && DATABASE_URL=""

  # Explicitly export them so they are readable by std::env::var in Rust
  export PORT
  export DATABASE_URL
fi

# 2. Fix local SQLite file permissions if you use it
case "$DATABASE_URL" in
  *sqlite://*)
    # Extract the raw file path if formatted as sqlite:///app/resource_management.db
    # Otherwise defaults to creating it directly in /app/
    DB_PATH=$(echo "$DATABASE_URL" | sed 's|^sqlite://||' | sed 's|^/||')
    [ -z "$DB_PATH" ] && DB_PATH="app/resource_management.db"
    
    touch "/$DB_PATH"
    chown appuser:appuser "/$DB_PATH"
    ;;
esac

# 3. CRUCIAL: Drop root privileges and execute your Rust application
# runuser preserves the exported variables for your Rust binary
exec runuser -u appuser -g appuser -- "$@"
