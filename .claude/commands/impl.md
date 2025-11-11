# impl

Implementation Workflow - Execute GitHub issue implementation based on current mode.

## Usage

```
/impl [issue-number] [optional message]
```

## Examples

```bash
/impl 123                    # Implement issue #123
/impl 123 with extra context # Implement with additional context
/impl 456                    # Implement issue #456
```

## Implementation

### Pre-Implementation Validation

1. **Check Dependencies**:
   - Validate GitHub CLI (`gh`) availability
   - Verify Git tools are available

2. **Validate Issue**:
   - Check issue exists and is open
   - Verify issue has `task` label
   - Extract issue title and metadata

3. **Validate Environment**:
   - Ensure clean git working directory
   - Verify we're in a git repository

### Implementation Steps

1. **Sync Staging Branch**:
   ```bash
   git checkout staging
   git pull origin staging
   ```

2. **Create Feature Branch**:
   ```bash
   git checkout -b feature/task-[issue-number]-[description]
   ```
   - Extract description from issue title
   - Use naming convention: `feature/task-{issue}-{description}`

3. **Mode-Specific Execution**:

   **MANUAL Mode**:
   - Provide detailed implementation guidance
   - Show step-by-step workflow instructions
   - Include validation requirements
   - Display commit message format
   - Provide next steps for human developer

   **COPILOT Mode**:
   - Trigger automatic implementation
   - Handle all validation steps
   - Create commit with proper format
   - Push branch and create PR

4. **Validation Requirements** (100% required):
   ```bash
   cargo build --release           # Build validation
   cargo clippy --all-targets --all-features  # Lint validation
   cargo fmt -- --check           # Format validation
   cargo check                    # Type check validation
   cargo test                     # Test validation (if applicable)
   ```

5. **Commit Format**:
   ```bash
   git commit -m "feat: [feature description]

   - Address #[issue-number]: [task title]
   - Build validation: 100% PASS (cargo build --release)
   - Clippy validation: 100% PASS (cargo clippy)
   - Format validation: 100% PASS (cargo fmt)

   🤖 Generated with Claude Code
   Co-Authored-By: Claude <noreply@anthropic.com>"
   ```

## Mode-Specific Behavior

### MANUAL Mode

**Output Includes**:
- Task information (issue, title, branch)
- Implementation steps checklist
- Validation requirements
- Commit message template
- Next steps for human developer

**Human Developer Responsibilities**:
- Execute implementation according to task requirements
- Run all validations before committing
- Use proper commit format
- Push branch and create PR with `/pr`

### COPILOT Mode

**Automatic Execution**:
- Complete implementation workflow
- All validation steps
- Proper commit formatting
- Branch creation and push
- PR creation via `/pr`

## Error Handling

- **Issue not found**: Clear error with issue number
- **Invalid environment**: Git status and directory checks
- **Validation failures**: Stop workflow and report errors
- **Mode-specific**: Provide appropriate guidance per mode

## Integration

- **Before**: Use `/plan [task]` to create task issues
- **After**: Use `/pr [feedback]` to create pull request
- **Mode**: Use `/mode [manual|copilot]` to set execution mode
- **Context**: Use `/fcs [topic]` for context discussions

## Files

- Feature branches: `feature/task-{issue}-{description}`
- `.claude/current_mode` - Determines execution behavior
- GitHub Issues - Task definitions and requirements

## Notes

- Always works from staging branch as base
- Feature branch naming is strictly enforced
- 100% validation is mandatory before commits
- Mode affects who performs implementation steps
- Never merge PRs yourself - wait for team approval