#[derive(Debug, PartialEq)]
struct Qwer{
    asdf:i32,
    zxcv:String,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2(){
        let result = add(1,1);
        assert_ne!(result, 4);
    }

    fn is_not_test(){
        println!("is not test!");
        assert_eq!(1,1);
    }

    #[test]
    fn it_works3(){
        let s1 = String::from("qwer");
        let s2 = "qwer";
        assert_eq!(s1, s2);
    }

    #[test]
    fn it_works4(){
        let a1 = [1,2,3,4];
        let mut a2:[i32; 4] = [0; 4];
        a2[0] = 1;
        a2[1] = 2;
        a2[2] = 3;
        a2[3] = 4;
        assert_eq!(a1,a2);
    }

    #[test]
    fn it_works5(){
        let v1 = vec![1,2,3,4];
        let mut v2:Vec<i32> = Vec::new();
        for i in 1..=4{
            v2.push(i);
        }
        assert_eq!(v1,v2);
    }

    #[test]
    fn it_works6(){
        let asdf = Qwer{asdf:1234,zxcv:String::from("qaz"),};
        let zxcv = Qwer{asdf:-1234,zxcv:String::from("edc"),};
        assert_ne!(asdf,zxcv);
    }

    #[test]
    fn not_works(){
        assert_eq!(1,2);
    }
}
