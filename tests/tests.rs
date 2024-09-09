use expense_tracker::expense_cli::functions::{
    convert_from_system_time, get_month_from_date_string, map_category,
};
use expense_tracker::expense_cli::structs_enums::{Category, Expense};
use std::time::SystemTime;

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

    #[test]
    fn test_get_month_from_date_string() {
        let month = get_month_from_date_string("2024-09-05 16:06:22");

        assert_eq!(month, 9);
    }

    #[test]
    fn test_convert_from_system_time() {
        let time = convert_from_system_time(SystemTime::now());
        assert!(time.contains("2024"));
        assert!(time.contains("09"));
    }
}
