use calamine::{open_workbook_auto, DataType, Range, Reader};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "(x)lsx (t)o (c)sv => convert xlsx file to a csv file")]
pub struct Args {
    /// Path to xlsx file
    #[structopt(long, short, parse(from_os_str))]
    pub xlsx: PathBuf,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let xlsx_file = &args.xlsx;

    validate_xlsx_file(xlsx_file)?;
    println!("Passed in the xlsx file {}", xlsx_file.to_str().unwrap());

    let csv_file = xlsx_file.with_extension("csv");
    println!("Creating the csv file{}", csv_file.to_str().unwrap());

    let mut csv_file = BufWriter::new(File::create(csv_file).unwrap());

    // let mut xlsxwb: Xlsx<_> = open_workbook(xlsx_file).unwrap();
    // if let Some(Ok(r)) = xlsxwb.worksheet_range("Sheet1") {
    //     for row in r.rows() {
    //         println!("row={:?}, row[0]={:?}", row, row[0])
    //     }
    // }
    let mut xl = open_workbook_auto(&xlsx_file).unwrap();
    let range = xl.worksheet_range("Sheet1").unwrap().unwrap();

    write_range(&mut csv_file, &range).unwrap();
    Ok(())
}

fn validate_xlsx_file(f: &PathBuf) -> Result<(), Box<dyn Error>> {
    match f.extension().and_then(|s| s.to_str()) {
        Some("xlsx") => Ok(()),
        _ => Err(From::from("Expecting to receive an xlsx file"))
    }
}

// https://github.com/tafia/calamine/blob/master/examples/excel_to_csv.rs
fn write_range<W: Write>(dest: &mut W, range: &Range<DataType>) -> std::io::Result<()> {
    let n = range.get_size().1 - 1;
    for r in range.rows() {
        for (i, c) in r.iter().enumerate() {
            match *c {
                DataType::Empty => Ok(()),
                DataType::String(ref s) => write!(dest, "{}", s),
                DataType::Float(ref f) => write!(dest, "{}", f),
                DataType::Int(ref i) => write!(dest, "{}", i),
                DataType::Error(ref e) => write!(dest, "{:?}", e),
                DataType::Bool(ref b) => write!(dest, "{}", b),
            }?;
            if i != n {
                write!(dest, ",")?;
            }
        }
        write!(dest, "\r\n")?;
    }
    Ok(())
}