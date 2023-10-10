pub mod course{
    use chrono::NaiveDate;

    #[derive(Debug)]
    pub struct Course {
        pub id: i32,
        pub teacher_id: i32,
        pub name: String,
        pub time: Option<NaiveDate>,
    }
}
