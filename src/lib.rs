pub fn print_useless(){

        println!("Hello VladArtem")

}

#[derive(Debug)]
pub struct Info<'a,'b>{

   author:&'a str,
   info:&'b str

}

pub fn printInfo(author:&str,info:&str){

   println!("Struct info {:?}",Info{author,info})

}
