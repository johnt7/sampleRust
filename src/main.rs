

fn main() {
    println!("Repeated={}", m1::test1(4));
    test_res(42);
    test_res(230);
}


mod m1 {
    use std::iter;

    pub fn test1(len: usize) -> String {
        iter::repeat("x").take(len).collect()
    }
}

fn test_res(x: usize) {
    match fallible1(x) {
        Ok(s) => println!("worked 1={}", s),
        Err(e) => println!("error 2={}", e)
    }
    match fallible2(x) {
        Ok(s) => println!("worked 2={}", s),
        Err(e) => println!("error 2={}", e)
    }
}

fn fallible1(x: usize) -> Result<String, u64> {
    let mut answer = another_fallible(x)?;
    answer.push_str("ss");
    Ok(answer)
}
fn fallible2(x: usize) -> Result<String, u64> {
    match another_fallible(x) {
        Ok(mut s) => {
            s.push_str("qq");
            Ok(s)
        },
        Err(e) => Err(e)
    }
}

fn another_fallible(x: usize) -> Result<String, u64> {
    if x > 100 {
        return Err(42);
    }
    Ok("hello".to_string())
}


///////////////////////////////////////////////////

fn borrow1() {
    let a = 42;
    // a = 3; a not mut
    let b = 10;
    let mut c = 15;
    let x;
    {
//        let e = &mut b;   // b not mut
        let e = &mut c;
        *e = 5;

        let y = 56;
        x = &y;
        println!("x={}", x);
    }
//    println!("x again={}", x);
}


///////////////////////////////////////////////////

enum Jtres<S, E> {
    Ok(S),
    Err(E)
}
fn test_resjt(x: usize) {
    /* 
    match fallible1jt(x) {
        Ok(s) => println!("worked jt1={}", s),
        Err(e) => println!("error jt2={}", e)
    }
    */
    match fallible2jt(x) {
        Ok(s) => println!("worked 2={}", s),
        Err(e) => println!("error 2={}", e)
    }
}

/*
fn fallible1jt(x: usize) -> Jtres<String, u64> {
    let answer = another_falliblejt(x)?;  // Try trait
    answer.push_str("ss");
    Jtres::Ok(answer)
}
*/
fn fallible2jt(x: usize) -> Jtres<String, u64> {
    match another_falliblejt(x) {
        Jtres::Ok(mut s) => {
            s.push_str("qq");
            Jtres::Ok(s)
        },
        Jtres::Err(e) => Jtres::Err(e)
    }
}
fn another_falliblejt(x: usize) -> Jtres<String, u64> {
    if x > 100 {
        return Jtres::Err(42);
    }
    Jtres::Ok("hello".to_string())
}


fn option() {
    let mut foo: Option<u32>;
    foo = None;
    foo = Some(3);
}