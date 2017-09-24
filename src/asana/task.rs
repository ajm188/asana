pub struct Task {
    title: String,
    description: String,
    due_date: String,
    // TODO: real Project type?
    project: Option<String>,
    // TODO: real Tag type?
    tags: Vec<String>,
}
