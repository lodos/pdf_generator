use serde::Deserialize;
use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use tokio::fs as async_fs;
use warp::Filter;

#[derive(Deserialize)]
struct DoctorInfo {
    fio: String,
    post: String,
    speciality: String,
}

#[derive(Deserialize)]
struct PatientInfo {
    fio: String,
    treatments: String,
    anamnes_morbi: String,
    objective_data: String,
    ecog: u32,
    karnovsky_scale: u32,
    clinical_ds: String,
    mkb10_code: String,
    recommends: String,
    cardno: String,
    snils: String,
    date_born: String,
    age: String,
}

#[derive(Deserialize)]
struct RequestData {
    doctor: DoctorInfo,
    patient: PatientInfo,
}

async fn generate_pdf_from_html(html_file: &str, output_pdf: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(html_file).exists() {
        return Err(format!("HTML файл '{}' не найден.", html_file).into());
    }

    let output_dir = Path::new(output_pdf).parent().unwrap();
    if !output_dir.exists() {
        return Err(format!("Выходная директория '{}' не найдена.", output_dir.display()).into());
    }

    let output = Command::new("/usr/local/bin/wkhtmltopdf")
        .arg(html_file)
        .arg(output_pdf)
        .output()?;

    if output.status.success() {
        println!("PDF успешно создан: {}", output_pdf);
    } else {
        eprintln!("Ошибка при генерации PDF: {:?}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

async fn create_pdf_handler(data: RequestData) -> Result<impl warp::Reply, warp::Rejection> {
    let start_time = Instant::now();

    let html_file = "src/public/html/index.html";
    let output_pdf = "src/public/pdf/index.pdf";

    let mut template = async_fs::read_to_string(html_file).await.map_err(|e| {
        eprintln!("Ошибка чтения HTML файла: {}", e);
        warp::reject::not_found()
    })?;

    // Замена шаблонных значений для врача
    template = template.replace("[DOCTOR_FIO]", &data.doctor.fio);
    template = template.replace("[DOCTOR_POST]", &data.doctor.post);
    template = template.replace("[DOCTOR_SPECIALITY]", &data.doctor.speciality);

    // Замена шаблонных значений для пациента
    template = template.replace("[PATIENT_FIO]", &data.patient.fio);
    template = template.replace("[PATIENT_DATE_BORN]", &data.patient.date_born);
    template = template.replace("[PATIENT_AGE]", &data.patient.age);
    template = template.replace("[PATIENT_TREATMENTS]", &data.patient.treatments);
    template = template.replace("[PATIENT_CARDNO]", &data.patient.cardno);
    template = template.replace("[PATIENT_SNILS]", &data.patient.snils);
    template = template.replace("[ANAMNES_MORBI]", &data.patient.anamnes_morbi);
    template = template.replace("[OBJECTIVE_DATA]", &data.patient.objective_data);
    template = template.replace("[ECOG]", &data.patient.ecog.to_string());
    template = template.replace("[KARNOVSKY_SCALE]", &data.patient.karnovsky_scale.to_string());
    template = template.replace("[CLINICAL_DS]", &data.patient.clinical_ds);
    template = template.replace("[MKB10_CODE]", &data.patient.mkb10_code);
    template = template.replace("[RECOMMENDS]", &data.patient.recommends);

    let temp_html_path = "src/public/html/temp.html";
    async_fs::write(temp_html_path, template).await.map_err(|e| {
        eprintln!("Ошибка записи временного HTML файла: {}", e);
        warp::reject::not_found()
    })?;

    if let Err(e) = generate_pdf_from_html(temp_html_path, output_pdf).await {
        eprintln!("Ошибка генерации PDF: {}", e);
        return Err(warp::reject::not_found());
    }

    let duration = start_time.elapsed();
    println!("Время выполнения: {:?}", duration);

    Ok(warp::reply::json(&format!("PDF успешно создан за {:?}", duration)))
}

#[tokio::main]
async fn main() {
    let create_pdf = warp::post()
        .and(warp::path("create_pdf"))
        .and(warp::body::json())
        .and_then(create_pdf_handler);

    warp::serve(create_pdf)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
