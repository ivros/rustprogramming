
mod structenum;

fn main(){
    println!("\nStruct + Enum + Match\n");

    let title = String::from("Bird Pet Guide!");
    let author = String::from("A Bird Owner");
    let mut new_book = structenum::Book::new(title, author);

    println!("Struct: {:?}", new_book);
    let days:i32 = 10;
    new_book.check_out(days);
    println!("Enum: {:?}", new_book);
    new_book.report_lost();
    println!("Enum: {:?}", new_book);

    println!("Match: {}", new_book.display_status());








}


