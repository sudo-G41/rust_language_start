# Functions
Rust의 함수에 관한 이야기
## Simple
-------------
Rust는 fn으로 함수가 시작되고 함수이름 소괄호 중괄호로 이루어져 있다.  
중괄호에 함수 몸체가 들어간다.  
코드 한줄 끝마다 ';'들어가야 한다.  
println!은 함수가 아니다! 이거 중요!!!!  
```rust
fu function_name(){
    //your code;
}
```

## Parameters
-------------
함수의 소괄호에 파라메터가 들어간다.  
파라메터는 (변수 이름:변수 데이터 타입)의 형태로 들어가며 예를들어(x:i32) 같이 들어간다.  
물론 여러개가 들어가도 상관없다!
```rust
fn function(i:i32, f:f64, b:bool, c:char, t:tup, ...){
    ...
}
```

## Statements and Expressions
-------------
구문과 표현식...?  
???  
 * statement(구문?)은 action 있고 return 없는거  
 * expression(표현식?)은 result를 return 하는거?
 잘 모르겠으니 코드를 보자...???
 ```rust
 fn function(){
    let x = 4;
    // let x = (let y = 4) is Error
    // x = y = value can not this code
    let y = {//"let y" is statement, {...} is expression
        let y = 3;
        y+1
    };
    println!("y:{y}");
}
 ```

## Return
-------------
rust는 마지막에 ;를 안붙이면 그걸 리턴하더라... 물론 좀 찾아보니 return 키워드 써도 되는거 같고 신기하구만...  
그리고 값을 반환하는 함수는 () -> date type{} 이런식으로 어떤타입을 리턴할 것인지 명시해줘야 한다.
```rust
fu function()->str{
    "Hello world!!!"
            or
    return "Hello world!!!!";
}
```

## 후기
------
뭔가... 뭔가 뭐랄까 리턴 신기하구만.... 그리고 그 뭐야 이름없는 함수 익명함수?... 그거 비슷한거 있는듯? 표현식? 그거..... 신기하구만