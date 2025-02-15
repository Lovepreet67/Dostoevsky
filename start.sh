
#!/bin/bash

SESSION="dostoevsky"

# Check if tmux session exists
tmux has-session -t $SESSION 2>/dev/null
if [ $? -ne 0 ]; then
    echo "Session $SESSION does not exist. Please create it first."
    exit 1
fi

echo "Starting processes in new tmux windows..."

# Start Dostoevsky in a new window
tmux new-window -t "$SESSION" -n "dostoevsky" "cargo run --dostoevsky"

# Wait for 5 seconds before starting other processes
sleep 5

# Start producers in new windows
tmux new-window -t "$SESSION" -n "producer_p1" "cargo run --producer p1"
tmux new-window -t "$SESSION" -n "producer_p2" "cargo run --producer p2"

# Start consumers in new windows
tmux new-window -t "$SESSION" -n "consumer_c1" "cargo run --consumer c1"
tmux new-window -t "$SESSION" -n "consumer_c2" "cargo run --consumer c2"

echo "All processes started in separate tmux windows."

# Attach to the session
tmux attach-session -t $SESSION
