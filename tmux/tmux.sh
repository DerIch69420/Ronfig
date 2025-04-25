#!/usr/bin/env bash

SESSION="ronfig"
PROJECT="$HOME/Projects/ronfig"

# Start a new detached tmux session with a window named 'code'
tmux new-session -d -s "$SESSION" -n code -c "$PROJECT"

# Send 'vi .' to the 'code' window
tmux send-keys -t "$SESSION:code" "vi ." C-m

# Create a second window named 'cargo'
tmux new-window -t "$SESSION:" -n cargo -c "$PROJECT"

# Create a third window named 'git'
tmux new-window -t "$SESSION:" -n git -c "$PROJECT"

# Select the 'code' window
tmux select-window -t "$SESSION:code"

# Attach to the session
tmux attach-session -t "$SESSION"

