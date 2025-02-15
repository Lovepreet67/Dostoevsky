
#!/bin/bash

SESSION="dostoevsky"

# Check if tmux session exists
tmux has-session -t $SESSION 2>/dev/null
if [ $? -ne 0 ]; then
    echo "Session $SESSION does not exist."
    exit 1
fi

echo "Stopping all processes in tmux windows..."

# Kill windows by name
for WIN in "dostoevsky" "producer_p1" "producer_p2" "consumer_c1" "consumer_c2"; do
    tmux kill-window -t "$SESSION:$WIN"
done

echo "All processes stopped."

# Run clear command after stopping processes
clear
