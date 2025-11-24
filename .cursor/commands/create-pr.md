# Create GitHub Pull Request

You are an expert at creating GitHub pull requests with detailed change comparisons.

Create a pull request comparing the current feature branch with the main branch, including a comprehensive analysis of all changes.

Below is how you will do it:

## Step 1: Get Current Branch Information

1. Run `git branch --show-current` to get the current branch name (assumed to be a feature branch).
2. Run `git remote get-url origin` to get the repository URL and extract owner/repo information.
3. Verify the branch exists and has commits: `git log --oneline main..HEAD` to see commits not in main.

## Step 2: Compare Changes with Main Branch

1. Get the list of changed files: `git diff --name-status main...HEAD` to see all modified, added, and deleted files.
2. Get detailed diff statistics: `git diff --stat main...HEAD` to see lines changed per file.
3. Get commit history: `git log main..HEAD --oneline --no-merges` to list all commits in the feature branch.
4. For each changed file, get a summary: `git diff main...HEAD -- <file>` to understand what changed (use this for key files only, not all files).

## Step 3: Analyze Changes

1. Categorize changes by type:
   - New features added
   - Bug fixes
   - Refactoring/Code improvements
   - Documentation updates
   - Test additions/modifications
   - Configuration changes
2. Identify the scope of changes:
   - Which modules/crates/packages were affected
   - What functionality was added or modified
   - Any breaking changes (if applicable)

## Step 4: Generate PR Title

1. Use Conventional Commits format: `type(scope): description`
2. Extract the main feature/fix from the branch name and commit messages
3. Keep it concise but descriptive (max 72 characters recommended)
4. Examples:
   - `feat(parser): add infix operator parsing support`
   - `fix(lexer): handle edge cases in tokenization`
   - `refactor(ast): improve expression node structure`

## Step 5: Generate PR Description

Create a comprehensive PR description with the following sections:

### Description Section

- Brief overview of what this PR accomplishes
- Context about why these changes were made

### Changes Made Section

- List all major changes grouped by category
- Include file paths and what was modified
- Reference specific functions/modules if significant

### Files Changed Section

- Summary table or list of files changed with brief descriptions:

  ```markdown
  | File              | Status   | Description                          |
  | ----------------- | -------- | ------------------------------------ |
  | parser/src/lib.rs | Modified | Added infix expression parsing logic |
  ```

### Testing Section

- Describe how the changes were tested
- Mention any new tests added
- Note if manual testing was performed

### Checklist Section

- [ ] Code follows project conventions
- [ ] Tests pass (if applicable)
- [ ] Documentation updated (if needed)
- [ ] No breaking changes (or breaking changes documented)

### Additional Context (if needed)

- Link to related issues
- Screenshots or examples (if applicable)
- Notes for reviewers

## Step 6: Create the Pull Request

1. Use the GitHub MCP tool `mcp_github_create_pull_request` with:
   - `owner`: Extracted from git remote URL
   - `repo`: Extracted from git remote URL
   - `head`: Current branch name
   - `base`: "main" (or "master" if that's the default branch)
   - `title`: Generated PR title
   - `body`: Generated PR description
   - `draft`: Set to `false` (or `true` if user requests draft PR)

## Step 7: Verify and Report

1. After creating the PR, confirm success and provide:
   - PR number
   - PR URL
   - Summary of what was created

## Important Notes

- Always compare against `main` branch (or `master` if that's the default)
- If the current branch is already `main`, inform the user and ask for clarification
- If there are no differences between branches, inform the user
- If the branch hasn't been pushed to remote, inform the user they need to push first
- Include all relevant context to help reviewers understand the changes
- Use markdown formatting for better readability
- Follow the project's contribution guidelines (check CONTRIBUTING.md if available)
