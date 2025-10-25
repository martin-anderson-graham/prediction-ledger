# Welcome to `predection-ledger`, a TUI for tracking your prediction accuracy


# prediction-ledger is designed to be modular. The app features these general modules:
1. client (tui, website, cli, app, etc)
2. backend - what the client interacts with
3. storage - what the backend uses to persist the predictions

# Crates
1. prediction-ledge-core - domain types shared by all modules
2. client - defines abstract interface for clients
3. client-tui:  a TUI client for prediction-ledger
    a. TUI built using [ratatui](https://github.com/ratatui-org/ratatui)

TODO: 
- [ ] display predictions and details
    - [ ] keyboard shortcuts in main window to highlight
    - [ ] show highlighed prediction details
        - [ ] use non-fixed contraints/truncate as needed
- [ ] write predictions to disk
- [ ] popup component
- [ ] create new predictions
- [ ] manipulate predictions
    - [ ] mark resolved
    - [ ] edit
    - [ ] delete
- [ ] graph of prediction results
- [ ] group predictions
- [ ] filter predictions


Long term goals:
- [ ] sqlite storage
- [ ] write seperate backend with db/api
- [ ] run client over ssh (terminal.shop)
- [ ] browser with leptos
