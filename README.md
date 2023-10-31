# CRUD Operations using Rust CLI

This project uses Rust to build a CLI tool to perform CRUD (Create, Read, Update, Delete) operations on a simple SQLite database. The database just contains one details table primarily used to demonstrate CRUD operations. The table has two columns.

![Table Structure](images/Screenshot%202023-10-31%20at%201.45.07 PM.png)

## How to Use
- First clone this repository.
- Then, cd into the repository's directory and run `cargo build`.
- Use the command line to interact with the database (see next section).

## CRUD CLI Commands
Once the project has been built, you can use these commands to perform CRUD operations on the table.

### Read

```cargo run query```

![Read](images/Screenshot%202023-10-31%20at%201.35.10 PM.png)

### Create

```cargo run insert <id> <age>```

![Read](images/Screenshot%202023-10-31%20at%201.35.53 PM.png)

### Update

```cargo run update <id> <new_age>```

![Read](images/Screenshot%202023-10-31%20at%201.36.32 PM.png)

### Delete

```cargo run delete <id>```

![Read](images/Screenshot%202023-10-31%20at%201.37.11 PM.png)


# GitHub Copilot
Most of this code was written with the help of GitHub Copilot. 

Copilot is an AI programming assistant that's set to transform your development process. This ingenious companion simplifies the daunting task of translating code from Python to Rust, a process that can be both time-consuming and error-prone. It's not just about faster coding but ensuring greater accuracy in the transformation.

Copilot automates the creation of boilerplate code necessary for building Command-Line Interface (CLI) tools. That means less manual work and more streamlined tool setup.

Once your Rust code is written, you can rely on automated workflows to handle code formatting, linting, and testing. This ensures that your code aligns with best practices and is well-prepared for deployment.

In a nutshell, Copilot makes coding more efficient and accurate, improving code organization, and simplifying the setup of CLI tools. It's a valuable ally in your development journey.
