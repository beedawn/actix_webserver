use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::path::PathBuf;
use std::path::Path;
//reads files and returns a string with them hyperlinked to their file paths relative to the host
//does not serve the files, just reads
fn read_files_convert_html_list (user_path:String)->String{
    //mutable string to build over course of function
    let mut path_string:String = "".to_owned();
    //vector to append results to
    let mut return_vec:Vec<PathBuf>=vec![];
        //loops through each of the files 
    //throws error if file is wrong
    let mut path_vector: Vec<PathBuf>= read_files_vec(vec![PathBuf::from(user_path.clone())]);
    //loops through items in return_vec, and renders the html to return
    for item in path_vector.clone(){
        //checks if path is dir, if so, no hyperlink
        if Path::new(&item).is_dir(){
            path_string.push_str(format!("<p>Directory:{}</p>\n",item.display()).as_str());

        }else{
            //else provide hyperlink
        path_string.push_str(format!("<p><a href=\"{}\">{}</a></p>\n",item.display(),item.display()).as_str());
        }
    }
    //returns string of html, list of links to each file in the supplied path from user_path
    path_string
}

 fn read_serve_files_as_bytes (user_path:String) -> Vec<u8> {
        let result:Vec<u8>=vec![];
        let file = fs::read(user_path);

        match file{
            Ok(file) => file,
            Err(err) => (format!("File not found. Error:{}",err).into())
        //could add something here to have a directory page populate
    }
}

fn read_files_string (user_path:String)->String{
    let mut path_string:String = "".to_owned();
    for entry in fs::read_dir(user_path.clone()).unwrap() {
    let entry_path = entry.unwrap().path();
    let entry_path_string = entry_path.display().to_string();
        if let Ok(entry) = fs::read_dir(user_path.clone()) {
            if Path::new(&entry_path_string).is_dir() {
                path_string.push_str(read_files_string(entry_path_string.clone()).as_str());
            }
        }else{
            println!("Error reading file directory");
        }
        path_string.push_str(entry_path_string.as_str());
    }
path_string
}

//the goal of this function is to take a string which is a path,
//then function then searchs for each file and directory recursively
// and converts their paths into a vector of PathBuf

//maybe need to rethink the logic and extract the string reading process from read_files into it's
//own function so it can be used in read_files and here.
fn read_files_vec (user_path_vec:Vec<PathBuf>)->Vec<PathBuf>{
    //so we need to loop over each string in the vector
    //find all the files and directories, collect those, and put them into the same vector

    //mutable string to build over course of function
    let mut path_string:String = "".to_owned();
    //do we need a veector? yes here we do
    let mut path_vector: Vec<PathBuf>= vec![];
    //loops through each of the files 
    //throws error if file is wrong
    //
    for single_path in user_path_vec{
       // println!("{:?}",single_path);
        //need error handling here for if a file is not a dir
        if let Ok(entry) = fs::read_dir(single_path.clone()){
        for entry in fs::read_dir(single_path.clone()).unwrap() {
            //unwraps entry into the path
            let entry_path = entry.unwrap().path();
            //gets a usable string from entry path because we use it alot right now
            let entry_path_string = entry_path.display().to_string();
            //if entry read is ok ?
                // println!("{:?}",entry_path);
                // checks if entry is a directory
                if Path::new(&entry_path_string).is_dir(){
                    //prints out found directories
                    //println!("{} is dir", entry_path_string);
                    //recursively calls read_files
                    //need to figureout if we should move to vector or keep string 
              
                    //throws compile error, need to figure out 
                
                    // path_string.push_str(read_files_vec(entry_path_string.clone()).as_str());
                   // read_files_vec(entry_path_string)
                    // path_vector.push(entry_path.clone());
                    let mut vec1 = read_files_vec(vec![entry_path.clone()]);


                    path_vector.append(&mut vec1);
                }
                    
                 // path_vector.push(entry_path.clone());
            
            // path_string.push_str(format!("<p><a href=\"{}\">{}</a></p>\n",entry_path_string,entry_path_string).as_str());
            path_vector.append(&mut vec![entry_path.clone()]);
         }
        }
    }
   // println!("{:?}",path_vector);

    //this is just here to get this to compile, the vec part
    //vec![path_string]
    path_vector
}

// slash route returns "irectory of files
#[get("/")]
async fn directory() -> impl Responder {
    let html_paths:String = read_files_convert_html_list(String::from("./html"));
   // println!("VECTOR::::{:?}", read_files_vec(vec![PathBuf::from("./html")]));
    HttpResponse::Ok().body(html_paths)
}
//test of error handling if file exists/did not exist
#[get("/gremlin")]
async fn gremlin() -> impl Responder {
    //error message if neither index.html or 404.html file(s) is(are) not found
    let error_var = "<p>Cannot read file.</p>";
    //reads index.html file
    let html = fs::read_to_string("./html/index.html");
    //reads 404 error page file
    let error_page = fs::read_to_string("./html/404.html");
    //check if file read of html variable(index.html) was successful
    match html{
        //no error, index.html path exists
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        //if index does not exist, go check if 404 page exists
        Err(err) => match error_page {
            //sends 404 error page
            Ok(error_page) => HttpResponse::Ok().content_type("text/html").body(error_page),
            //text displayed if both 404 page and index cannot be found
            Err(err)=> HttpResponse::Ok().body(error_var),
        }
    }
}
 async fn file_render_manual(path: web::Path<(String)>)->HttpResponse{
    let string = format!(".{}",path.clone());
   //let string= read_files(string);
   // println!("{}",string);
    let bytes = read_serve_files_as_bytes(string);
//println!("path:{}",path);
  //  HttpResponse::Ok().body(format!("User detail: {} {}", path.into_inner(),string))
    HttpResponse::Ok().body(bytes)

}
    
//404 error page for default service to handle all unaddressed endpoints
async fn error_page() -> impl Responder {
    //read 404.html to String
    let error_file = fs::read_to_string("./html/404.html");
//check if file read was successful or not
    match error_file {
        //404.html exists
        Ok(error_file) => HttpResponse::Ok().content_type("text/html").body(error_file),
        //404.html does not exist
        Err(err) => HttpResponse::Ok().body("File not found")
    }
}

//modular route configuration,
//do we need a loop here to create all the end points?
//the one bummer about this config is that it needs restarted everytime the app runs
fn config(cfg: &mut web::ServiceConfig) {
    //need to write a loop here that gets the file names and then creates an end point and
    //serves it at each end point
   let mut path_vec= read_files_vec(vec![PathBuf::from("./html")]);
    let mut x = 5;
    for item in path_vec {
    let mut s = item.display().to_string();
        if s.len() > 0 {
        s.remove(0);
        }
    cfg.service(web::resource(format!("{}",s))
        .route(web::get().to(move|| file_render_manual(s.clone().into())))
        .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
    );
    x-=1;
    }
}

//main server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //HttpServer instantiatiatiation
    HttpServer::new(|| {
        App::new()
            .configure(config)
            //slash / endpoint
            .service(directory)
            //handles all unaddressed endpoints
        .default_service(
        web::route().to(error_page)
            )
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
