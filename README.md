[![Integration Tests](https://github.com/eva672/fibbot/actions/workflows/cli.yml/badge.svg)](https://github.com/eva672/fibbot/actions/workflows/cli.yml)
This documentation will help or guide you on the projects .

**INTRODUCTION**

1. **GITHUB ACTIONS**
GitHub Actions enables automation of workflows directly in your GitHub repository, providing a way to automate tasks like building, testing, and deploying code.

When you want to create a Github Action, you open your github repository any of your choice then you do the following:

- when opening the github repository you will a bar of options .
- you chose the Action Option 
- when you are in the Action you click new action 
- They will show you different templates you chose one of them and configure it with any name you want
- for example if you take the rust template you can rename it to any name of your choice and work on it 
- i will show some examples down .
**NB** : Take note that there is a difference between a Workflow and an action ;

<table>
<tr><th>Action</th><th>Workflow</th></tr>
<tr><td>an action is a specific task or operation within that workflow.</td><td>a workflow is a series of steps or actions designed to achieve a specific goal</td></tr>
<tr><td> is a reusable component that can be used within a workflow.</td><td>a workflow is a single file that defines a set of jobs and steps to be executed, triggered by specific events like a push or pull request. </td></tr>
<table>
Below are some key concepts concerning workflows;
**Key Concept**

- **Workflows**

A workflow is a unit in GitHub Actions. It defines a series of jobs that get triggered by specific events. Each workflow is defined in its own YAML file inside the ``.github/workflows/ ``directory of your repository.
Example of a workflow:

name: Rust

on:
  push:
    branches: [ "master" ]
  pull_request:
    branches: [ "master" ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4
    - name: Build
      run: cargo build --verbose
    - name: Run tests
      run: cargo test --verbose

 - **Action (action.yml)**

The metadata file, action.yml, is essential for defining a custom action. It contains information about the action’s behavior, inputs, and outputs.

- You can create an action.yml file in your GitHub repository
