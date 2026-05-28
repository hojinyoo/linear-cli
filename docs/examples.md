# Usage Examples

## Common Tasks

```bash
linear-cli common
linear-cli tasks
linear-cli agent
```

## Projects

```bash
linear-cli p list                              # List all projects
linear-cli p list --archived                   # Include archived
linear-cli p get PROJECT_ID                    # View project details
linear-cli p create "Q1 Roadmap" -t Engineering
linear-cli p update PROJECT_ID --name "New Name"
linear-cli p update PROJECT_ID --name "New Name" --dry-run
linear-cli p delete PROJECT_ID --force
linear-cli p add-labels PROJECT_ID LABEL_ID
```

## Issues

```bash
linear-cli i list                              # List issues
linear-cli i list -t Engineering -s "In Progress"
linear-cli i list --output json                # Output as JSON
linear-cli i get LIN-123                       # View issue details
linear-cli i get LIN-123 --output json         # JSON output
linear-cli i create "Bug fix" -t Eng -p 1      # Priority: 1=urgent, 4=low
cat issue.json | linear-cli i create "Bug fix" -t Eng --data -
linear-cli i update LIN-123 -s Done
linear-cli i update LIN-123 -s Done --dry-run
linear-cli i delete LIN-123 --force
linear-cli i start LIN-123                     # Start working: assigns to you, sets In Progress, creates branch
linear-cli i stop LIN-123                      # Stop working: unassigns, resets status
```

## Labels

```bash
linear-cli l list                              # List project labels
linear-cli l list --type issue                 # List issue labels
linear-cli l create "Feature" --color "#10B981"
linear-cli l create "Bug" --type issue --color "#EF4444"
linear-cli l delete LABEL_ID --force
```

## Git Integration

```bash
linear-cli g checkout LIN-123                  # Create/checkout branch for issue
linear-cli g branch LIN-123                    # Show branch name for issue
linear-cli g create LIN-123                    # Create branch without checkout
linear-cli g checkout LIN-123 -b custom-branch # Use custom branch name
linear-cli g pr LIN-123                        # Create PR linked to issue
linear-cli g pr LIN-123 --draft                # Create draft PR
linear-cli g pr LIN-123 --base main            # Specify base branch
```

## jj (Jujutsu) Integration

The git subcommands auto-detect Jujutsu repositories. Pass `--vcs jj` to force it.

```bash
linear-cli g checkout LIN-123 --vcs jj         # Create bookmark for issue
linear-cli g branch LIN-123 --vcs jj           # Show bookmark name for issue
linear-cli g create LIN-123 --vcs jj           # Create bookmark without checkout
linear-cli g commits --vcs jj                  # Show commits with Linear trailers
linear-cli g pr LIN-123 --vcs jj               # Create PR using jj git push
```

## Sync Local Folders

```bash
linear-cli sy status                           # Compare local folders with Linear
linear-cli sy push -t Engineering              # Create Linear projects for local folders
linear-cli sy push -t Engineering --dry-run    # Preview without creating
```

## Search

```bash
linear-cli s issues "authentication bug"
linear-cli s projects "backend" --limit 10
```

## Uploads

Download attachments and images from Linear issues/comments:

```bash
# Download to file
linear-cli up fetch "https://uploads.linear.app/..." -f image.png

# Output to stdout (for piping to other tools)
linear-cli up fetch "https://uploads.linear.app/..." | base64

# Useful for AI agents that need to view images
linear-cli uploads fetch URL -f /tmp/screenshot.png
```

## Other Commands

```bash
# Teams
linear-cli t list
linear-cli t get TEAM_ID

# Users
linear-cli u list
linear-cli u get me

# Cycles
linear-cli c list -t Engineering
linear-cli c current -t Engineering

# Comments
linear-cli cm list ISSUE_ID
linear-cli cm list ISSUE_ID --output json      # JSON output for LLMs
linear-cli cm create ISSUE_ID -b "This is a comment"

# Documents
linear-cli d list
linear-cli d get DOC_ID
linear-cli d create "Doc Title" -p PROJECT_ID
linear-cli d update DOC_ID --title "New title" --dry-run
linear-cli d list --output json

# Templates
linear-cli tpl list
linear-cli tpl list --output json
linear-cli tpl show bug --output json

# Statuses
linear-cli st list -t Engineering
linear-cli st get "In Progress" -t Engineering

# Config
printf '%s\n' "$LINEAR_API_KEY" | linear-cli config set-key
linear-cli config show
```

## Interactive Mode

```bash
linear-cli interactive                         # Launch interactive TUI
linear-cli int --team ENG                      # Launch with preselected team (alias: int)
```

## Multiple Workspaces

Workspaces are managed through `config workspace-*` subcommands.

```bash
linear-cli config workspace-list               # List configured workspaces
printf '%s\n' "$LINEAR_API_KEY" | linear-cli config workspace-add personal  # Add a workspace
linear-cli config workspace-switch personal    # Switch active workspace
linear-cli config workspace-current            # Show current workspace
linear-cli config workspace-remove personal    # Remove a workspace
```

## Bulk Operations

Issues are passed with `-i` as a comma-separated list.

```bash
linear-cli b update-state Done -i LIN-1,LIN-2,LIN-3   # Update status for multiple issues
linear-cli b assign me -i LIN-1,LIN-2                 # Assign multiple issues
linear-cli b label bug -i LIN-1,LIN-2                 # Add label to multiple issues
linear-cli b unassign -i LIN-1,LIN-2                  # Unassign multiple issues
```

## JSON Output

```bash
# Use --output json with any list or get command
linear-cli i list --output json
linear-cli p list --output json | jq '.[] | .name'
linear-cli i get LIN-123 --output json
linear-cli t list --output json
linear-cli cm list ISSUE_ID --output json    # Comments as JSON (great for LLMs)

# Token-saving JSON output options
linear-cli i list --output json --fields identifier,title,state.name --compact
LINEAR_CLI_OUTPUT=json linear-cli i list --sort identifier --order desc

# Color control for logs/CI
linear-cli i list --no-color

# Table width control
linear-cli i list --width 80
linear-cli i list --no-truncate
```
