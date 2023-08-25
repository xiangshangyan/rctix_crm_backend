use rust_xlsxwriter::{Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    println!("Hello, world!");
    let mut work_book = Workbook::new();
    let mut worksheet = work_book.add_worksheet();
    worksheet.write(0, 0, "Hello, world!")?;
    worksheet.write(1, 0, 1234)?;
    work_book.save("hello.xlsx").unwrap();
    Ok(())
}
