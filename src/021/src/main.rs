use actix_web::{web, web::Path, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::io::Write;

fn flush_stdout() {
    std::io::stdout().flush().unwrap();
}

fn delete_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Deleting file \"{}\" ... ", filename);
    flush_stdout();

    // TODO: Delete the file.

    println!("Deleted file \"{}\"", filename);
    HttpResponse::Ok()
}

fn download_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Downloading file \"{}\" ... ", filename);
    flush_stdout();

    // TODO: Read the contents of the file.
    let contents = "Contents of the file.\n".to_string();

    println!("Downloaded file \"{}\"", filename);
    HttpResponse::Ok().content_type("text/plain").body(contents)
}

fn upload_specified_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Uploading file \"{}\" ... ", filename);
    flush_stdout();

    // TODO: Get from the client the contents to write into the file.
    let _contents = "Contents of the file.\n".to_string();

    // TODO: Create the file and write the contents into it.

    println!("Uploaded file \"{}\"", filename);
    HttpResponse::Ok()
}

fn upload_new_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Uploading file \"{}*.txt\" ... ", filename);
    flush_stdout();

    // TODO: Get from the client the contents to write into the file.
    let _contents = "Contents of the file.\n".to_string();

    // TODO: Generate new filename and create that file.
    let file_id = 17;

    let filename = format!("{}{}.txt", filename, file_id);

    // TODO: Write the contents into the file.

    println!("Uploaded file \"{}\"", filename);
    HttpResponse::Ok().content_type("text/plain").body(filename)
}

fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}

fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080";

    println!("Listening at address {} ...", server_address);
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/{filename}")
                    .route(web::delete().to(delete_file))
                    .route(web::get().to(download_file))
                    .route(web::put().to(upload_specified_file))
                    .route(web::post().to(upload_new_file)),
            )
            .default_service(web::route().to(invalid_resource))
    })
    .bind(server_address)?
    .run()
}
