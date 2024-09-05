use assert_cmd::Command;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_command() {
        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("add")
            .arg("-d")
            .arg("description")
            .arg("-a")
            .arg("40.0")
            .assert()
            .success();
    }

    #[test]
    fn test_update_command() {
        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("update").arg("--id").arg("arg").assert().success();

        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("update")
            .arg("-i")
            .arg("arg")
            .arg("-d")
            .arg("arg")
            .assert()
            .success();
    }

    #[test]
    fn test_delete_command() {
        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("delete").arg("arg").assert().failure();

        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("delete").arg("-d").assert().failure();

        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("delete").arg("-i").assert().failure();

        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("delete").arg("-i").arg("arg").assert().success();

        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("delete").arg("--id").arg("arg").assert().success();
    }

    #[test]
    fn test_view_command() {
        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("view").assert().success();
    }

    #[test]
    fn test_summary_command() {
        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("summary").assert().success();
    }

    #[test]
    fn test_category_command() {
        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("category").assert().success();
    }

    #[test]
    fn test_budget_command() {
        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("budget").assert().success();
    }

    #[test]
    fn test_export_command() {
        let mut cmd = Command::cargo_bin("expense-tracker").unwrap();
        cmd.arg("export")
            .arg("--file")
            .arg("filename")
            .assert()
            .success();
    }
}
