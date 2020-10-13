use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "(x)lsx (t)o (c)sv => convert xlsx file to a csv file")]
pub struct Args {
    /// Path to xlsx file
    #[structopt(long, short, parse(from_os_str))]
    pub xlsx: PathBuf,
    /// Sheet name
    #[structopt(long, short, parse(from_str))]
    pub sheet: Option<String>,
    /// Print out sheet names
    #[structopt(long, short)]
    pub print_sheets: bool,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let xlsx_file = &args.xlsx;
    let sheet_name = &args.sheet.unwrap_or(String::from("Sheet1"));
    let print_sheet_names = args.print_sheets;

    validate_xlsx_file(xlsx_file)?;

    let mut xl: Xlsx<_> = open_workbook(&xlsx_file).unwrap();

    if print_sheet_names {
        println!("The following sheet names are contained within {}: {:#?}", xlsx_file.to_str().unwrap(), xl.sheet_names());
        return Ok(())
    }

    validate_sheet_name(&xl, sheet_name)?;

    let range = xl.worksheet_range(&sheet_name).unwrap().unwrap();

    println!("Converting sheet {} within the xlsx file {}", sheet_name, xlsx_file.to_str().unwrap());

    let csv_file = xlsx_file.with_extension("csv");
    println!("Creating the csv file{}", csv_file.to_str().unwrap());

    let mut wtr = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Always)
        .from_path(csv_file)?;

    // let mut xlsxwb: Xlsx<_> = open_workbook(xlsx_file).unwrap();
    // if let Some(Ok(r)) = xlsxwb.worksheet_range("Sheet1") {
    //     for row in r.rows() {
    //         println!("row={:?}, row[0]={:?}", row, row[0])
    //     }
    // }

    write_range(&mut wtr, &range).unwrap();
    Ok(())
}

fn validate_xlsx_file(f: &PathBuf) -> Result<(), Box<dyn Error>> {
    match f.extension().and_then(|s| s.to_str()) {
        Some("xlsx") => Ok(()),
        _ => Err(From::from("Expecting to receive an xlsx file"))
    }
}

fn validate_sheet_name(xl: &Xlsx<BufReader<File>>, s: &str) -> Result<(), Box<dyn Error>> {
    if xl.sheet_names().iter().any(|sn| sn==s) {
        Ok(())
    } else {
        Err(From::from(format!("The sheet {} cannot be found within the xlsx file", s)))
    }
}

// https://github.com/tafia/calamine/blob/master/examples/excel_to_csv.rs
fn write_range(wtr: &mut csv::Writer<File>, range: &Range<DataType>) -> Result<(), Box<dyn Error>> {
    for r in range.rows() {
        // https://github.com/zitsen/xlsx2csv.rs/blob/master/src/main.rs
        let cols: Vec<String> = r
            .iter()
            .map(|c|
                match *c {
                    DataType::String(ref s) => format!("{}", s),
                    DataType::Float(ref f) => format!("{}", f),
                    DataType::Int(ref i) => format!("{}", i),
                    DataType::Bool(ref b) => format!("{}", b),
                    _ => String::from("")
                })
                .collect();
        wtr.serialize(cols)?;
    }
    wtr.flush()?;
    Ok(())
}