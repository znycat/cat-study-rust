use std::rc::Rc;

fn main() {
    // 传递变量到闭包
    let a = Rc::new(1);
    let b = Rc::new(2);
    let c = Rc::new(3);

    let closure = {
        // 好的实现 在一个新的scope中去做clone 借用
        // 对对象进行重绑定的时候，放到一个单独的作用域下会更好一些
        let b1 = b.clone();
        let c1 = c.as_ref();
        move || {
            let ret = *a + *b1 + *c1;
            println!("ret: {}", ret);
        }
    };

    closure();

    // bad:
    let a = Rc::new(1);
    let b = Rc::new(2);
    let c = Rc::new(3);

    let b_cloned = b.clone();
    let c_borrowed = c.as_ref();
    let closure = move || {
        let ret = *a + *b_cloned + *c_borrowed;
        println!("ret: {}", ret);
    };
    closure();


}
