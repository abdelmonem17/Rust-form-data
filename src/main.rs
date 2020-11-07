use actix_form_data::{Error, Field, Form, Value};
use actix_web::{web::{post, resource}, App, HttpResponse, HttpServer, web};
use actix_web::get;
use futures::stream::StreamExt;
use std::path::PathBuf;
use futures::{SinkExt};
use futures::stream::Stream;
use std::fs;
use std::borrow::Borrow;
use actix_web::web::get;


async fn upload(uploaded_content: Value) -> HttpResponse {

   // println!("Uploaded Content: {:#?}", uploaded_content);



     let map= uploaded_content.map().unwrap();
    //  let files= &map["hi"];
    // let arr= files.text();



    for (key ,val) in map
    {
        match key.as_str()
         // if key.contains( "username" )
        {
             "username"=>
                    {
                    let txt = val.text().unwrap();
                    println!("username : {:#?}", txt);
                    }
                "files"=>
                    {

                        let arr = val.array().unwrap();
                        for file in arr
                        {
                          //println!("{:?}",file.file().unwrap().saved_as);
                            println!("{:#?}",file);
                            let cur_file = file.file().unwrap();
                            let file_data :Vec<&str>= cur_file.filename.split('.').collect();
                            let file_name= file_data[0];
                            //println!("file data : {}",file_data[0]);

/*
                            let paths = fs::read_dir("./examples").unwrap();
                            for path in paths
                            {
                                let file_path =format!("{}",  path.unwrap().path().display() );
                                match file_path.as_str()
                                {
                                    r#"./examples\upload.jpeg"#=>{
                                        let name = format!(r#"./examples\{}-{}.jpeg"# ,file_name ,uuid::Uuid::new_v4());
                                        fs::rename(r#"./examples\upload.jpeg"#, name.as_str()).unwrap();
                                        println!("good!!");
                                    },
                                    _=>()
                                };



                            }*/
                            let name = format!(r#"./examples\{}-{}.jpeg"# ,file_name ,uuid::Uuid::new_v4());

                          //  r#"./examples\upload.jpeg"#
                           // upload.jpeg

                           if let Err(_)=  fs::rename( format!(r#"{}\{}"#,IMG_DIR,IMG_DEF_NAME).as_str(), name.as_str())
                           {
                                    println!("bad rename");
                           }
                        }
                        //println!("{:?}",arr);
                    }
                 _=>{}

        }

    }

    HttpResponse::Created().finish()
}



const IMG_DEF_NAME:&str="upload.jpeg";
const IMG_DIR:&str="./examples";
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let paths = fs::read_dir("./examples").unwrap();

    for path in paths {
      //  let file = path.unwrap().path();
        println!("Name: {}",path.unwrap().path().display());
    }

    fs::create_dir_all(IMG_DIR)?;
    let form = Form::new()
        .field("username", Field::text())
        .field(
            "files",
            Field::array(Field::file(|_, _, mut stream| async move {
                use futures_lite::io::AsyncWriteExt;

                let filename = format!("{}/{}",IMG_DIR , IMG_DEF_NAME);

                let mut file = async_fs::File::create(&filename).await.unwrap();
                while let Some(res) = stream.next().await {
                  let bytes=  res?;
                    file.write_all(&bytes).await.unwrap();
                }
                file.flush().await.unwrap();
                Ok(None) as Result<_, Error>
            })),
        );

    println!("{:?}", form);

    HttpServer::new(move || {
        App::new()
            .service(resource("/test/{id}").route(get().to(upload)))
            .wrap(form.clone())
            .service(resource("/upload").route(post().to(upload)))


    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}


//#[get("/test/{id}")]
pub async fn test(web::Path((id)):web::Path<(u32)>) ->&'static str
{
    println!("good");

    r#"you get the api successfully "#

}
