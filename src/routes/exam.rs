// Code for the exam module
use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use axum_extra::extract::Multipart;
use bytesize::ByteSize;
use log::debug;
use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tower_http::limit::RequestBodyLimitLayer;

use crate::askama_templates::ExamPageTemplate;
use crate::models::record::Record;
use crate::AppState;

pub const EXAM_TABLE: &str = "exam";
const MAX_TOTAL_UPLOAD_SIZE: usize = ByteSize::mb(280).as_u64() as usize;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExamForm {
    pub id: Option<RecordId>,
    pub student_name: String,
    pub task_name: String,
    pub answer_file: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub created_at_pretty: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub task_id: String,
    pub title: String,
    pub description: Vec<String>,
    pub image_paths: Vec<String>,
    pub task_name: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/",
        get(get_exam_page)
            .post(post_answer)
            .layer(RequestBodyLimitLayer::new(MAX_TOTAL_UPLOAD_SIZE)),
    )
}

async fn get_exam_page(State(app_state): State<AppState>) -> Html<String> {
    let tasks = vec![
        Task {
            task_id: "1".to_string(),
            title: "Task 1: Crop a landscape image to 1920x1080 pixel (10 marks)".to_string(),
            description: vec![
                "1. Crop a landscape image to 1920x1080 pixel (1920x1080 pixel ရှိသော image ပုံထွက်အောင် ဖြတ်ထုတ်ပါ)".to_string(),
                "2. Save the project as a .xcf file (.xcf project ဖိုင်အဖြစ် သိမ်းပါ)".to_string(),
                "3. Upload the .xcf file below (.xcf ဖိုင်ကို  submit တင်ပါ။".to_string(),
            ],
            image_paths: vec!["assets/images/nature-mountain-river-landscape.jpg".to_string()],
            task_name: "task_1_crop_image".to_string(),
        },
        Task {
            task_id: "2".to_string(),
            title: "Task 2: Exposure  သုံးပြီး ပုံတောက်လာအောင် လုပ်ပါ။ (10 marks)".to_string(),
            description: vec![
                "1. Exposure သုံးပြီး ပုံတောက်လာအောင် လုပ်ပါ။".to_string(),
                "2. Save the project as a .xcf file (.xcf project ဖိုင်အဖြစ် သိမ်းပါ)".to_string(),
                "3. Upload the .xcf file below (.xcf ဖိုင်ကို  submit တင်ပါ။".to_string(),
            ],
            image_paths: vec![
                "assets/images/apple-underexposed.jpg".to_string()
            ],
            task_name: "task_2_enhance_image".to_string(),
        },
        Task {
            task_id: "3".to_string(),
            title: "Task 3: Black and White color သို့ပြောင်းပေးပါ။ (10 marks)".to_string(),
            description: vec![
                "1. ပုံကို Black & White ပြောင်းပေးပါ။ ".to_string(),
                "2. Save the project as a .xcf file (.xcf project ဖိုင်အဖြစ် သိမ်းပါ)".to_string(),
                "3. Upload the .xcf file below (.xcf ဖိုင်ကို  submit တင်ပါ။".to_string(),
            ],
            image_paths: vec![
                "assets/images/dramatic_color_image.webp".to_string()
            ],
            task_name: "task_3_black_and_white_image".to_string(),
        },
        Task {
            task_id: "4".to_string(),
            title: "Task 4: လှပသောအသားအရည် ဖြစ်အောင် လုပ်ပေးပါ။ (15 Marks)".to_string(),
            description: vec![
                "1. လှပသောအသားအရည် ဖြစ်အောင် လုပ်ပေးပါ။".to_string(),
                "2. Save the project as a .xcf file (.xcf project ဖိုင်အဖြစ် သိမ်းပါ)".to_string(),
                "3. Upload the .xcf file below (.xcf ဖိုင်ကို  submit တင်ပါ။".to_string(),
            ],
            image_paths: vec![
                "assets/images/skin_imperfections.png".to_string()
            ],
            task_name: "task_4_skin_imperfections".to_string(),
        },
        Task {
            task_id: "5".to_string(),
            title: "Task 5: ကမ်းချေမှ လူကို ဖျောက်ပါ။ (15 Marks)".to_string(),
            description: vec![
                "1. ကမ်းချေမှ လူကို ဖျောက်ပါ။".to_string(),
                "2. Save the project as a .xcf file (.xcf project ဖိုင်အဖြစ် သိမ်းပါ)".to_string(),
                "3. Upload the .xcf file below (.xcf ဖိုင်ကို  submit တင်ပါ။".to_string(),
            ],
            image_paths: vec![
                "assets/images/person-on-a-beach.jpg".to_string()
            ],
            task_name: "task_5_person-on-a-beach".to_string(),
        },
        Task {
            task_id: "6".to_string(),
            title: "Task 6: To နဲ့ From မှာ စာများထည့်ပါ။ (10 Marks)".to_string(),
            description: vec![
                "1. To နဲ့ From မှာ စာများထည့်ပါ။".to_string(),
                "2. Save the project as a .xcf file (.xcf project ဖိုင်အဖြစ် သိမ်းပါ)".to_string(),
                "3. Upload the .xcf file below (.xcf ဖိုင်ကို  submit တင်ပါ။".to_string(),
            ],
            image_paths: vec![
                "assets/images/letter.jpg".to_string()
            ],
            task_name: "task_6_letter".to_string(),
        },
        Task {
            task_id: "7".to_string(),
            title: "Task 7: ကုလားအုတ် ကို ဖြတ်ထုတ်ပြီး ကန္တာရ ပုံနောက်တစ်ခုမှ သွားထည့်ပါ။ (15 Marks)".to_string(),
            description: vec![
                "1. ကုလားအုတ် ကို ဖြတ်ထုတ်ပြီး ကန္တာရ ပုံနောက်တစ်ခုမှ သွားထည့်ပါ။".to_string(),
                "2. Save the project as a .xcf file (.xcf project ဖိုင်အဖြစ် သိမ်းပါ)".to_string(),
                "3. Upload the .xcf file below (.xcf ဖိုင်ကို  submit တင်ပါ။".to_string(),
            ],
            image_paths: vec![
                "assets/images/camels.jpg".to_string(),
                "assets/images/desert.jpg".to_string()
            ],
            task_name: "task_7_camel_to_another_desert".to_string(),
        },
        Task {
            task_id: "8".to_string(),
            title: "Task 8: ရထားကြီး အရှိန်နဲ့ အလျင်မြန်စွာ မောင်းနေပုံလုပ်ပါ။ (20 Marks)".to_string(),
            description: vec![
                "1. ရထားကြီး အရှိန်နဲ့ အလျင်မြန်စွာ မောင်းနေပုံလုပ်ပါ။ ".to_string(),
                "2. Save the project as a .xcf file (.xcf project ဖိုင်အဖြစ် သိမ်းပါ)".to_string(),
                "3. Upload the .xcf file below (.xcf ဖိုင်ကို  submit တင်ပါ။".to_string(),
            ],
            image_paths: vec![
                "assets/images/train.jpg".to_string()
            ],
            task_name: "task_8_train_motion".to_string(),
        },
    ];
    Html(ExamPageTemplate { tasks }.render().unwrap())
}

async fn post_answer(
    State(app_state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Html<String>, StatusCode> {
    let created_at = chrono::Local::now().naive_local();
    let mut exam_form = ExamForm {
        id: None,
        student_name: String::new(),
        task_name: String::new(),
        answer_file: None,
        created_at,
        created_at_pretty: created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    debug!("Exam form: {:?}", exam_form);

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "student_name" => exam_form.student_name = field.text().await.unwrap_or_default(),
            "task_name" => exam_form.task_name = field.text().await.unwrap_or_default(),
            "answer_file" => {
                if let Some(filename) = field.file_name() {
                    // TODO: Use a better way to store the file
                    // TODO: Validation
                    let filename2 =
                        format!("{}-{}", ulid::Ulid::new().to_string(), filename.to_string());
                    let file_path = format!("assets/exam_files/{}", &filename2); // FIXME: Security issue?
                    if let Some(parent) = std::path::Path::new(&file_path).parent() {
                        fs::create_dir_all(parent).await.unwrap();
                    }
                    let mut file = fs::File::create(&file_path).await.unwrap();
                    while let Some(chunk) = field.chunk().await.unwrap() {
                        file.write_all(&chunk).await.unwrap();
                    }
                    exam_form.answer_file = Some(format!("/assets/exam_files/{}", filename2));
                }
            }
            _ => (),
        }
    }

    debug!("Exam form: {:?}", exam_form);
    // TODO: Optimize this to return the created product directly
    let _created_exam_record: Vec<Record> = app_state
        .db
        .lock()
        .await
        .create(EXAM_TABLE)
        .content(&exam_form)
        .await
        .unwrap();

    if _created_exam_record.is_empty() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Html(format!(
        "Answer submitted Successful.. AnswerID: {}",
        _created_exam_record[0].id
    )))
}
