use std::arch::naked_asm;

fn is_even(num:i32)->bool{
    if num %2==0{
        return true;
    }
    return false;
}

fn fibo(num:i32)->i32{
    let mut first=0;
    let mut second=1;
    if num== 0 {
        return first;
    }

    if num==1 {
        return 1;
    }


    for _ in 0..(num-1){
        let temp=second;
        second=second+first;
        first=temp;
    }
    return second;
    
}

fn str_len(s:&str)->usize{
    s.chars().count()
}

struct User{
    first_name:String,
    last_name:String,
    age:i32
}



struct Rect{
    width:i32,
    height:i32
}

impl  Rect {
    fn area(&self)->i32{
        self.width*self.height
    }
}


fn main() {
//     println!("{}",is_even(20));
//     println!("{}",fibo(1));
//     println!("Hello, world!");
// let my_string=String::from("hemanth");
// let len=str_len(&my_string);
// println!("{}",len)


// let user=User{
//     first_name:String::from("hemanth"),
//     last_name:String::from("k"),
//     age:23
// };

// println!("{}",user.first_name)


let react2=Rect{
    width:10,
    height:20,
};
println!("{}",react2.area());


}
