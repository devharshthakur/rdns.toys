# Initialize Agent

You are an expert at creating comprehensive project documentation for AI agents. Create an `agent.md` file that contains all the essential context and information about the project that AI models need to understand and work with the codebase effectively.

Below is how you will do it:

## Step 1: Project Discovery and Analysis

1. **Check if agent.md already exists:**
   - If `agent.md` exists in the project root, check if the user wants to overwrite it or update it
   - If updating, preserve important custom sections while refreshing dynamic content

2. **Read and analyze project structure:**
   - Read `package.json` to understand:
     - Project name, version, description
     - Dependencies and devDependencies
     - Scripts available
     - Package manager (npm, pnpm, yarn)
   - Read `README.md` if it exists to understand:
     - Project purpose and goals
     - Setup instructions
     - Key features
   - Check for configuration files:
     - `tsconfig.json` or `jsconfig.json` (TypeScript/JavaScript config)
     - `.gitignore` (to understand what's excluded)
     - `docker-compose.yml` or `Dockerfile` (if containerized)
     - `.env.example` or similar (environment variables)
     - Framework-specific configs (next.config.js, vite.config.js, etc.)

3. **Analyze project structure:**
   - List the root directory to understand the folder structure
   - Identify the main source directory (src/, lib/, app/, etc.)
   - Identify test directories (tests/, **tests**/, spec/)
   - Identify build/output directories (dist/, build/, out/)
   - Check for documentation directories (docs/, documentation/)

4. **Detect technology stack:**
   - Programming language (JavaScript, TypeScript, Python, etc.)
   - Framework (React, Next.js, Vue, Express, etc.)
   - Build tools (Webpack, Vite, Rollup, etc.)
   - Testing framework (Jest, Vitest, Mocha, etc.)
   - Linting/formatting tools (ESLint, Prettier, etc.)

## Step 2: Codebase Analysis

1. **Identify key entry points:**
   - Main entry file (index.js, main.js, app.js, etc.)
   - Framework entry points (App.jsx, \_app.js, etc.)
   - Configuration files

2. **Understand project architecture:**
   - Check for common patterns (MVC, component-based, etc.)
   - Identify if it's a monorepo (check for workspaces in package.json)
   - Check for API routes or backend structure
   - Identify database or data layer if present

3. **Read important configuration files:**
   - Build configuration
   - Linting rules
   - TypeScript configuration
   - Any custom configuration files

## Step 3: Create Comprehensive agent.md Content

Create the `agent.md` file with the following structure and content:

### File Structure Template

```markdown
# Project Context for AI Agents

## Project Overview

[Project name, description, and purpose]

## Technology Stack

- **Language:** [Primary language]
- **Framework:** [Framework if applicable]
- **Package Manager:** [npm/pnpm/yarn]
- **Build Tool:** [Build tool if applicable]
- **Testing Framework:** [Testing framework if applicable]

## Project Structure

[Directory structure with brief descriptions]

## Key Dependencies

[Important dependencies and their purposes]

## Development Setup

[How to set up and run the project]

## Important Files and Directories

[Key files and what they do]

## Coding Standards and Conventions

[Any coding standards, naming conventions, etc.]

## Build and Deployment

[How to build and deploy]

## Testing

[How to run tests]

## Common Tasks

[Common development tasks]

## Notes and Considerations

[Any special considerations, edge cases, or important notes]
```

### Content Generation Rules

1. **Project Overview Section:**
   - Use the description from package.json
   - Include project name and version
   - Add any additional context from README.md

2. **Technology Stack Section:**
   - List all detected technologies
   - Be specific about versions if mentioned in package.json
   - Include build tools, testing frameworks, etc.

3. **Project Structure Section:**
   - Create a tree-like structure showing main directories
   - Include brief descriptions of what each directory contains
   - Highlight important directories (src, tests, config, etc.)

4. **Key Dependencies Section:**
   - List important runtime dependencies (not devDependencies unless critical)
   - Explain what each dependency is used for
   - Group by category if there are many dependencies

5. **Development Setup Section:**
   - Include installation steps
   - Include how to run the development server
   - Include environment variable setup if applicable
   - Reference package.json scripts

6. **Important Files and Directories Section:**
   - List key files with their purposes
   - Include configuration files
   - Include entry points

7. **Coding Standards Section:**
   - Include any linting rules found
   - Include formatting preferences
   - Include naming conventions if detectable
   - Include TypeScript strictness if applicable

8. **Build and Deployment Section:**
   - Include build commands
   - Include output directory information
   - Include any deployment-specific notes

9. **Testing Section:**
   - Include how to run tests
   - Include test directory location
   - Include test framework information

10. **Common Tasks Section:**
    - Include common development tasks
    - Include available npm/pnpm/yarn scripts
    - Include any workflow information

## Step 4: Edge Cases and Error Handling

1. **Handle missing files gracefully:**
   - If package.json doesn't exist, create a minimal agent.md with available information
   - If README.md doesn't exist, skip that section
   - If no configuration files exist, note that in the document

2. **Handle different project types:**
   - **Empty project:** Create a template agent.md with placeholders
   - **Monorepo:** Include workspace information
   - **Library/package:** Focus on build and distribution
   - **Application:** Focus on setup and deployment
   - **Full-stack:** Include both frontend and backend information

3. **Handle special cases:**
   - If the project uses multiple languages, document all of them
   - If there are multiple entry points, document all of them
   - If there are environment-specific configs, document them
   - If there are database migrations or schemas, include that information

4. **File writing considerations:**
   - Check if the file already exists and ask for confirmation if needed
   - Use proper markdown formatting
   - Ensure the file is readable and well-structured
   - Add a comment at the top indicating when it was last generated

5. **Content validation:**
   - Ensure all sections have meaningful content (not just placeholders)
   - Verify that file paths mentioned actually exist
   - Check that commands mentioned are valid
   - Ensure dependencies listed are accurate

## Step 5: Final Steps

1. **Write the agent.md file:**
   - Create the file in the project root directory
   - Use proper markdown formatting
   - Ensure it's comprehensive but not overly verbose
   - Make it easy for AI models to parse and understand

2. **Verify the output:**
   - Read back the created file to ensure it was written correctly
   - Check that all important information is included
   - Ensure formatting is correct

3. **Provide feedback:**
   - Inform the user that the file was created
   - Highlight any sections that might need manual review
   - Mention if any information was missing or could not be determined

## Important Notes

- Always be thorough in gathering information before writing
- If information is missing, note it in the document rather than guessing
- Keep the document focused on what AI models need to know
- Update existing agent.md files rather than overwriting if they contain custom content
- Use clear, concise language
- Structure the document for easy parsing by AI models
- Include examples where helpful
- Document edge cases and special considerations
