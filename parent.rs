use rust_issue_51798_example_child;

mod server {
    use crate::rust_issue_51798_example_child;

    fn f() {
        let mut v = rust_issue_51798_example_child::vec();
        v.clear();
    }
}
