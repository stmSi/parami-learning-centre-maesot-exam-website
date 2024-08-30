use askama::Template;

use crate::routes::exam::Task;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "categories.html")]
pub struct CategoriesPageTemplate {}

#[derive(Template)]
#[template(path = "exam.html")]
pub struct ExamPageTemplate {
    pub tasks: Vec<Task>,
}
