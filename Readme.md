## GitHub-Beholder

**GitHub-Beholder** is a simple command-line interface (CLI) application that fetches the recent activity of a specified GitHub user and displays it in the terminal. This project helps you practice programming skills, such as working with APIs, handling JSON data, and building a CLI application.

### Requirements

- **CLI Functionality**: The application should run from the command line, accept the GitHub username as an argument, fetch the user’s recent activity using the GitHub API, and display it in the terminal.

### Features

- Provide the GitHub username as an argument when running the CLI:

  ```bash
  github-beholder <username>
  ```

- **Fetch the recent activity** of the specified GitHub user using the GitHub API. You can use the following endpoint to fetch the user’s activity:

  ```plaintext
  https://api.github.com/users/<username>/events
  ```

  Example:

  ```plaintext
  https://api.github.com/users/kamranahmedse/events
  ```

- **Display the fetched activity** in the terminal in a user-friendly format. For example:

  ```
  - Pushed 3 commits to kamranahmedse/developer-roadmap
  - Opened a new issue in kamranahmedse/developer-roadmap
  - Starred kamranahmedse/developer-roadmap
  ```

- **Error handling**: The application gracefully handles errors, such as:
  - Invalid usernames
  - API failures
  - Network issues

### Advanced Features (Optional)

- **Filtering by event type**: You can add the ability to filter activities based on event types such as `PushEvent`, `IssuesEvent`, etc.
- **Structured output**: Format the display of activities in a more readable way.
- **Caching data**: Cache fetched data locally to reduce API requests and improve performance.
- **Explore other GitHub API endpoints**: You can enhance the application by fetching additional information like user repositories, followers, or starred repositories.

### How to use

1. **Clone the repository**:

   ```bash
   git clone https://github.com/yourusername/github-beholder.git
   cd github-beholder
   ```

2. **Install dependencies** (if any).

3. **Run the application** with a GitHub username as an argument:

   ```bash
   ./github-beholder <github-username>
   ```

4. **View the activity** in the terminal:

   ```plaintext
   Pushed 3 commits to user/repository
   Starred user/repository
   ```

### Example

```bash
./github-beholder kamranahmedse
```

Output:

```plaintext
Pushed 3 commits to kamranahmedse/developer-roadmap
Opened a new issue in kamranahmedse/developer-roadmap
Starred kamranahmedse/developer-roadmap
```

### Error Handling

- **Invalid Username**:

  ```plaintext
  Error: Invalid GitHub username or no recent activity found.
  ```

- **API Failure**:

  ```plaintext
  Error: Unable to fetch data. Please check your internet connection or try again later.
  ```

### Contribution

Feel free to contribute to this project! You can suggest new features, improve error handling, or optimize the code.