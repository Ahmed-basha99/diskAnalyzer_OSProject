
use std::fs::{self, metadata};
use std::path::Path;
use filesize::PathExt;

fn rec (x:String, depth :i32 ,max_depth:i32) -> bool{
    if depth ==max_depth+1  {
        return false ;
    }
    
    let s_slice: &str = &x.to_owned()[..] ;
    let path = metadata(s_slice).unwrap();
    if path.is_file() {return false;}

    let mut count = depth;
   
    for file  in fs::read_dir(s_slice).unwrap(){
        let mut count = depth;
        while count > 1 {
            print!("       ");
            count = count-1;
        } 
        print!("|_____");
        let strr = file.unwrap().path().display().to_string();
        let tempPath = Path::new(&strr[..]);
        let size = tempPath.symlink_metadata();
        let ssize = tempPath.size_on_disk();
        println!(" {}   :  {:?} ",strr, ssize);
        
         rec(strr,depth+1,max_depth);
    }
    return true;
}

fn main (){
let x:String = String::from("./");
rec(x,1,3);
}