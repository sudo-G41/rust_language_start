# variables
## Rust variables property
Rust variables is immutable  
이유는 컴파일러가 컴파일 과정에서 변수들을 직접 확인하여 잘못된 변경이 발생하지 않도록 보장해 주기 위해 불변하게 만들었다.  
```rust
let x = 5;
x = 6; //Error! variable is immutable! can't not change
```
그렇다고 변수가 변경되지 않으면 반복문같이 계속 변경되면서 해당 조건에 멈춰야 하는데 계속 변수를 만들 수 없다.  
그래서 있는 기능이 mut와 shadowing이다.

## mut keyword
mut는 해당 변수가 immutable가 아닌 mutability하다고 선언해주는 것?이다.  
사용법은 "let mut 변수명" 형태이다.
```rust
let q = 10; //immutable
let mut w = 10; //mutability
q = 2; //Error!
w = 2; //Success!
```

## shadowing
shadowing은 키워드는 아니다.  
변수의 재할당? 이런 느낌으로 이해하면 될거 같다. 아래 예제를 보자
```rust
let hello_len = "Hello sudo-G41?"
let hello_len = hello_len.len();
```
hello라는 변수가 처음에는 인사말의 String 변수였는데 다음줄을 보면 해당 String 길이를 저장하는 변수가 되어있다.  
이런식으로 변수를 다시 선언하고 그곳에 저장하는 것을 shadowing라 한다.

## mut vs shadowing
솔직히 그때  그때 다르니 뭐가 더 좋다 말 못하겠고 차이점이나 말해보자.  
mut는 그냥 값을 변경 가능하다. 다만 언제 불변이 될지 모르니 애매하다. 그렇다고 안좋냐? 그렇다기에는 값을 입력 받는다던가 반복분에서 증감하는 변수들 가이 간단하게 쓰기 좋다.  
shadowed는 다시 선언하는 것이라 데이터 타입이 변경되어도 가능하다는 장점이 있다. shadowing 예제같이 String -> int로 변경이 가능하지만 mut는 같은 데이터 타입에서 변경되는거라 불가능하다.

## 후기
헤헿 어려운건지 쉬운건지 모르겠다. 근데 확실한건 너무 다르다. 이거 c/c++과 python과 rust 3개 다 다르냐...  
변수 어려워... ㅎㅎ....