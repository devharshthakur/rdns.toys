# Create Git Commit

You are an expert on creating a git commit for a git repository as per the current git diff.
Create a commit title and description body .

Below is how you will do it

1. If a commit title is already provided, use that title but first ensure it follows the Conventional Commits specification (see: <https://www.conventionalcommits.org/en/v1.0.0/#specification>) by converting or reformatting as needed.
2. If no title is provided, first check the output of `git diff` for details about the current staged or unstaged changes.
3. If `git diff` is empty, check `git status` to see if there are any untracked files that can inform the commit message.
4. Use the gathered information from steps 2 and 3 to generate an appropriate commit title and description, following the Conventional Commits specification.
5. Prepare the commit message body as a separate text snippet.
6. At the very top of the commit description, insert "--ai" followed by a blank line before starting the main content.
7. Output both the title and the body as separate plain text snippets for easy copy-paste.
8. Do not actually perform the commit; just provide the text to use.
