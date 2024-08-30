// use std::str;
//
// use serde::{Deserialize, Serialize};
// use std::string::ToString;
// use surrealdb::sql::Thing;
// use ulid::Ulid;
//
// pub const STUDENT_TABLE: &str = "student";
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct StudentRecordId(Thing);
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Student {
//     pub id: StudentRecordId,
//     pub name: String,
//     pub answer_files: Vec<String>,
//     pub created_at: chrono::NaiveDateTime,
// }
//
// #[derive(Deserialize, Debug)]
// pub struct SearchQuery {
//     pub query: String,
// }
//
// impl Student {
//     pub fn new(name: String) -> Self {
//         Self {
//             id: StudentRecordId(Thing {
//                 tb: STUDENT_TABLE.to_string(),
//                 id: surrealdb::sql::Id::String(Ulid::new().to_string()),
//             }),
//             name,
//             answer_files: Vec::new(),
//             created_at: chrono::Local::now().naive_local(),
//         }
//     }
// }
//
// #[derive(Serialize, Debug)]
// pub struct StudentViewModel {
//     pub id: String,
//     pub name: String,
// }
//
// impl From<StudentForm> for Student {
//     fn from(student_form: StudentForm) -> Self {
//         Student::new(student_form.name)
//     }
// }
//
// impl From<Student> for StudentViewModel {
//     fn from(student: Student) -> Self {
//         StudentViewModel {
//             id: format!("{}", student.id.0.id),
//             name: student.name,
//         }
//     }
// }
//
// impl From<&Student> for StudentViewModel {
//     fn from(student: &Student) -> Self {
//         StudentViewModel {
//             id: format!("{}", student.id.0.id),
//             name: student.name.clone(),
//         }
//     }
// }
