extern crate ical;

use std::io::BufReader;
use std::fs::File;
use std::error::Error;


fn test_raw_line(buf: BufReader<File>) {
    let reader = ical::IcalReader::new(buf);

    let mut i = 0;

    for res in reader {
        match res {
            Ok(line_parsed) => println!("{:?}", line_parsed),
            Err(err) => println!("{}", err),
        };

        i += 1;

        if i > 10 {
            break;
        }
    }
}



fn test_raw_line_2(buf: BufReader<File>) {
    let reader = ical::IcalReader::new(buf);


    for res in reader {
        match res {
            Ok(line_parsed) => println!("{:?}", line_parsed),
            Err(err) => println!("{}", err.description()),
        };

    }

    assert!(false, "END")
}


#[test]
fn test_mltiple_root_components() {
    let buf = BufReader::new(File::open("./tests/ressources/multiple_root_components.ics")
        .unwrap());

    test_raw_line(buf);
}

#[test]
fn test_rfc() {
    let buf = BufReader::new(File::open("./tests/ressources/rfc.ics").unwrap());

    test_raw_line(buf);
}

#[test]
fn test_component() {
    let buf = BufReader::new(File::open("./tests/ressources/component.ics").unwrap());

    test_raw_line_2(buf);
}