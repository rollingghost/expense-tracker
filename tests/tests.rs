use expense_tracker::{map_category, Category, Expense};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_expense() {
        let new_expense = Expense::new("description".to_string(), 40.0, map_category("category"));

        assert_eq!(new_expense.description, "description");
        assert_eq!(new_expense.amount, 40.0);
        assert_eq!(new_expense.category, Category::Other)
    }

    #[test]
    fn test_map_category() {
        // Should be of category food
        let category = "food";
        assert_eq!(map_category(category), Category::Food);

        // Should be transportation
        let category = "transportation";
        assert_eq!(map_category(category), Category::Transportation);

        // Should be entertainment
        let category = "entertainment";
        assert_eq!(map_category(category), Category::Entertainment);

        // Should be other
        let category = "should be other";
        assert_eq!(map_category(category), Category::Other);
    }
}
