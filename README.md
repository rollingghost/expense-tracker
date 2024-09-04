# Expense Tracker

Build a simple expense tracker application to manage your finances. The application allows users to add, delete, and view their expenses, and provides a summary of the expenses.

## Project URL

[Expense Tracker Project](https://roadmap.sh/projects/expense-tracker)

## Features

- **Add Expense**: Users can add an expense with a description and amount.
- **Update Expense**: Users can update an existing expense.
- **Delete Expense**: Users can delete an expense.
- **View Expenses**: Users can view all expenses.
- **Summary**: Users can view a summary of all expenses.
- **Monthly Summary**: Users can view a summary of expenses for a specific month (of the current year).

## Additional Features

- **Expense Categories**: Add expense categories and allow users to filter expenses by category.
- **Monthly Budget**: Allow users to set a budget for each month and show a warning when the user exceeds the budget.
- **Export to CSV**: Allow users to export expenses to a CSV file.

## Commands and Expected Output

```sh
$ expense-tracker add --description "Lunch" --amount 20
# Expense added successfully (ID: 1)

$ expense-tracker add --description "Dinner" --amount 10
# Expense added successfully (ID: 2)

$ expense-tracker list
# ID  Date       Description  Amount
# 1   2024-08-06  Lunch        $20
# 2   2024-08-06  Dinner       $10

$ expense-tracker summary
# Total expenses: $30

$ expense-tracker delete --id 1
# Expense deleted successfully

$ expense-tracker summary
# Total expenses: $20

$ expense-tracker summary --month 8
# Total expenses for August: $20
```

## Implementation

You can implement the application using any programming language of your choice. Here are some suggestions:

- Use any programming language with a module for parsing command arguments (e.g., Python with `argparse`, Node.js with `commander`, etc.).
- Use a simple text file to store the expenses data. You can use JSON, CSV, or any other format to store the data.
- Add error handling to handle invalid inputs and edge cases (e.g., negative amounts, non-existent expense IDs, etc.).
- Use functions to modularize the code and make it easier to test and maintain.

This project is a great way to practice your logic-building skills and learn how to interact with the filesystem using a CLI application. It will also help you understand how to manage data and provide useful information to users in a structured way. Good luck!  :smile:
