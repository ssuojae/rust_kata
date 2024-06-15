
## 스크린샷

<img width="566" alt="image" src="https://github.com/suojae3/rust_kata/assets/126137760/7911c219-1939-40b5-96ba-ebe2de99f536">



<br/>
<br/>



## 새로 알게된 점

### 매크로
-  예제코드에서 `print!`를 보고 느낌표가 어떤 역할을 하는지 찾아보았다.
-  Rust에서 일반적인 함수 호출과 구분시켜주기위해 매크로 호출시에는 `!`를 붙여준다는 사실을 인지
-  직접 매크로를 만들어보았다.

#### 매크로 선언 기본 틀
```rust
macro_rules! macro_name {
    (매크로가 받을 인자) => {
        매크로가 할 일
    };
}
```


#### 긍정 사고 매크로 만들기

```rust
macro_rules! print_positive_thinking {
    ($expression:expr) => {
        println!("{} so lucky! 🥰", $expression);
    };
}

fn main() {
    print_wonyoung_thinking!("I'm hungry");
}

// I'm hungry so lucky! 🥰
```
- 여기서 expr은 매크로 인자의 종류를 나태내는 매크로 구문으로 리터럴, 변수, 함수 호출, 산술 연산 등을 의미한다.
- 매크로 인자 앞에는 `$`를 붙여준다. expression 앞에 `$`를 붙이는 이유이다.

<br/>

---

###  Formatting

- **Formatting**이란? <br/>
 데이터를 특정 형식으로 변환하여 출력 및 저장하는 작업.<br/>
 주로 문자열이나 숫자 등의 데이터를 보기 좋은 형태로 변환하기 위해 사용.

#### 기본 formatting 구문
```rust
fn main() {
    let name = "Alice";
    let age = 30;
    println!("Name: {}, Age: {}", name, age);
}

// Name: Alice, Age: 30
```

#### 위치 지정자
```rust
fn main() {
    let name = "Alice";
    let age = 30;
    println!("{1} is {0} years old.", age, name);
}

// Alice is 30 years old.
```

#### 이름 지정자
```rust
fn main() {
    let name = "Alice";
    let age = 30;
    println!("{name} is {age} years old.", name=name, age=age);
}

// Alice is 30 years old.
```

#### format specifier - 정렬
```rust
fn main() {
    println!("{:<5}", "x"); // align left
    println!("{:>5}", "x"); // align right
    println!("{:^5}", "x"); // align center
    println!("{:*<5}", "x"); // fill '*' and align left
}

/*
x    
    x
  x  
x**** */
```

#### format specifier - 최소너비
```rust
fn main() {
    println!("{:5}", "x");
}
```


#### format specifier - 정밀도

```rust
fn main() {
    let s = "Hello, world!";
    println!("{:.5}", s); // 최대 5글자 출력
    let pi = 3.141592;
    println!("{:.2}", pi); // 소수점 이하 2자리까지 출력
}

// Hello
// 3.14
```

#### format specifier - 진수

```rust
fn main() {
    let number = 255;
    println!("{:b}", number); // 2진수
    println!("{:o}", number); // 8진수
    println!("{:x}", number); // 16진수 (소문자)
    println!("{:X}", number); // 16진수 (대문자)
}

/*
11111111
377
ff
FF
*/
```

<br/>

---

### Vec 

- 다른 언어와 다르게 왜 Rust는 배열을 Vec(벡터)라고 부르는지 궁금해졌다.
- 알고보니 Rust에서는 배열 선언 방식이 `[T; N]` 정적 배열과 `Vec<T>`동적 배열 두가지가 있었다.

#### 배열 `([T; N])`
```rust
fn main() {
    let arr: [i32; 3] = [1, 2, 3]; // 크기가 고정된 배열
    println!("{:?}", arr); // 출력: [1, 2, 3]
}
```
- 고정된 크기를 가지며, 크기가 컴파일 타임에 결정된다. 
- 배열의 크기는 변경할 수 없고 주로 스택(stack)에 저장된다.

#### 벡터 `(Vec<T>)`

```rust
fn main() {
    let mut vec: Vec<i32> = Vec::new(); // 빈 벡터 생성
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", vec); // 출력: [1, 2, 3]
}
```

- 벡터는 크기가 동적으로 변경될 수 있는 배열이다. 크기가 런타임에 결정되며 주로 힙(heap)에 저장된다. 
