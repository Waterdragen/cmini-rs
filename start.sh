while true; do
    cargo run -r -- -y
    exit_code=$?
    if [ $exit_code -ne 0 ]; then
        break
    fi
done