# Create Feature Branch for Porting Go Code

You are an expert on creating git branches for porting specific Go code parts from the reference implementation to Rust.

Below is how you will do it:

1. Analyze the user's request to identify what Go feature/part is being ported (e.g., "Integer Literals", "prefix operators", "identifier expressions").
2. Extract the key feature name from the description as context (e.g., "Integer Literals" → "integer-literal-parsing").
3. Normalize the feature name by:
   - Converting to lowercase
   - Replacing spaces and underscores with hyphens
   - Using descriptive naming that indicates what's being added (e.g., "parsing", "support", "implementation")
4. Format the branch name as `feat/add-<normalized-feature-name>` following conventional commits structure.
5. Do not Create and checkout the new branch that will be done by me, only provide branch name as copy text snippet
6. Output a success message with the created branch name and what Go feature is being ported.
7. If the branch already exists, inform the user and suggest alternatives.

## Examples

- User says: "Integer Literals" or "porting integer literal parsing from Go"
  → Creates: `feat/add-integer-literal-parsing`
- User says: "prefix operators" or "porting prefix operator support"
  → Creates: `feat/add-prefix-operators`
- User says: "identifier expressions" or "porting identifier expression parsing"
  → Creates: `feat/add-identifier-expression-parsing`

## Context

This command is specifically for creating branches when porting features from the Go reference implementation (in `go/` directory) to the Rust implementation. The branch names should clearly indicate what Monkey language feature is being added to the Rust parser/evaluator.
