#[derive(Debug, PartialEq, Clone)]
struct Book<T>{
    name        : String,
    language    : String,
    pages       : u16,
    etc         : Vec<T>,
    page        : u16,
}

#[derive(Debug, PartialEq, Clone)]
struct Note<T>{
    pages       : u16,
    etc         : Vec<T>,
    page        : u16,
}

trait BooksTrait<T>{
    fn open(&mut self, page:u16)->Option<u16>;
    fn close(&mut self)->Option<u16>;
    fn read(&self)->Option<T>;
    fn write(&mut self, s:T);
    fn print_info(&self);
}

trait MyCopy{
    fn clone(&self)->String{
        return self.clone();
    }
}

impl<T:Clone> Book<T>{
    fn new(name:String, language:String, pages:u16, t:T)->Self{
        let mut etc:Vec<T> = Vec::new();
        for _ in 0..pages{etc.push(t.clone());}
        Book{
            name        : name,
            language    : language,
            pages       : pages,
            etc         : etc,
            page        : 0,
        }
    }
}

impl<T:Clone> Note<T>{
    fn new(pages:u16, t:T)->Self{
        let mut etc:Vec<T> = Vec::new();
        for _ in 0..pages{etc.push(t.clone());}
        Note{
            pages   : pages,
            etc     : etc,
            page    : 0,
        }
    }
}

impl<T> BooksTrait<T> for Book<T> where
T: std::fmt::Display+Clone
{
    fn open(&mut self, page:u16)->Option<u16>{
        print!("{} ", self.name);
        if self.pages < page{
            println!("{} is over", page);
            return None;
        }
        println!("{}page open!", page);
        self.page = page;
        return Some(page);
    }

    fn close(&mut self)->Option<u16>{
        if self.pages == 0{
            println!("?? close fail");
            return None;
        }
        println!("close...");
        let page = self.page;
        self.page = 0;
        return Some(page);
    }

    fn read(&self)->Option<T>{
        let page:u16 = self.page;
        let pages:u16 = self.pages;
        if page == 0{
            println!("now close!!");
        }
        else if page > pages{
            println!("??? Weather Hackers");
        }
        else{
                let page:usize = page.into();
                return Some(self.etc[page].clone());
        }
        None
    }
    fn write(&mut self, s:T){
        let page:usize = self.page.into();
        self.etc[page] = s;
    }

    fn print_info(&self){
        println!("name      : {}", self.name);
        println!("language  : {}", self.language);
        println!("total     : {} pages", self.pages);
        println!("now       : {} page", self.page);
        match self.read(){
            Some(t) => println!("etc : {}", t),
            None    => println!("?? weather hackers!"),
        };
    }
}

impl<T> BooksTrait<T> for Note<T> where
T: std::fmt::Display+Clone
{
    fn open(&mut self, page:u16)->Option<u16>{
        print!("Note ");
        if self.pages < page{
            println!("{} is over", page);
            return None;
        }
        println!("{}page open!", page);
        self.page = page;
        return Some(page);
    }

    fn close(&mut self)->Option<u16>{
        if self.pages == 0{
            println!("?? close fail");
            return None;
        }
        println!("close...");
        let page = self.page;
        self.page = 0;
        return Some(page);
    }

    fn read(&self)->Option<T>{
        let page:u16 = self.page;
        let pages:u16 = self.pages;
        if page == 0{
            println!("now close!!");
        }
        else if page > pages{
            println!("??? Weather Hackers");
        }
        else{
                let page:usize = page.into();
                return Some(self.etc[page].clone());
        }
        None
    }

    fn write(&mut self, s:T){
        let page:usize = self.page.into();
        self.etc[page] = s;
    }

    fn print_info(&self){
        println!("total     : {} pages", self.pages);
        println!("now       : {} page", self.page);
        match self.read(){
            Some(t) => println!("etc : {}", t),
            None    => println!("?? weather hackers!"),
        };
    }
}

fn main() {
    let mut momo:Book<String> = Book::new(String::from("momo"), String::from("kr"), 367, String::new());
    let mut study_note:Note<String> = Note::new(30, String::new());
    momo.open(1);
    println!("");
    momo.print_info();
    println!("");
    study_note.open(20);
    println!("");
    study_note.print_info();
    println!("");
    momo.open(1000);
    println!("");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    study_note.write(s);
    study_note.print_info();
}
