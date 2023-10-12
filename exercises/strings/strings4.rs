// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {  //辨别&str和String
    /*
    "blue": 是 &str 类型，因为它是一个字符串字面量，它自动具有 &str 类型
    。
    "red".to_string(): 是 String 类型，因为 to_string() 方法将 &str 转换为 String 类型。

    String::from("hi"): 是 String 类型，因为 String::from() 将 &str 转换为 String 类型。

    "rust is fun!".to_owned(): 是 String 类型，因为 to_owned() 方法将 &str 转换为 String 类型。

    "nice weather".into(): 是 String 类型，因为 into() 方法将 &str 转换为 String 类型。

    format!("Interpolation {}", "Station"): 是 String 类型，因为 format! 宏返回一个 String 类型的字符串。

    &String::from("abc")[0..1]: 是 &str 类型，因为通过切片操作符 [] 取得的是字符串切片。

    " hello there ".trim(): 是 &str 类型，因为 trim() 方法返回一个字符串切片。

    "Happy Monday!".to_string().replace("Mon", "Tues"): 是 String 类型，因为 to_string() 方法将 &str 转换为 String 类型，
    并且 replace() 方法返回一个新的 String 类型的字符串。

    "mY sHiFt KeY iS sTiCkY".to_lowercase(): 是 String 类型，因为 to_lowercase() 方法返回一个新的 String 类型的字符串。
     */
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
