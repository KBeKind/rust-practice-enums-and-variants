use std::{mem::discriminant, result, fs::File};



// ENUMS IN RUST ARE A CUSTOM DATA TYPE
enum DiskType {
    // VARIANTS - NOT PROPERTIES OR FIELDS
    SSD,
    HDD,
}

// #[derive(Debug)] MUST BE USED DIRECTLY ABOVE THE ENUM IN ORDER TO BE AVAILABLE FOR THAT ENUM
#[derive(Debug)]
enum WineRegions {
    Region1,
    Region2,
    Region3,
    Region4,
}

struct Wine {
    name: String,
    // WineRegions IS USED AS A TYPE HERE
    region: WineRegions,
}

fn supported_regions(wine_region: WineRegions) {
    match wine_region {
        WineRegions::Region1 => println!("region 1"),
        WineRegions::Region2 => println!("region 2"),
        _ => println!("this region is not acceptable")
    }
}


fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        // THIS IS VALID BECAUSE IS IS THE OTHER VARIANT OF Option
        None
    } else {
        // CREATES THE Option<i32> VALUE.  Some() CREATES A NEW INSTANCE OF Option
        Some(x / y)
    }
}


enum FileSize {
    Bytes(u64),
    KiloBytes(u64),
    MegaBytes(u64),
    GigaBytes(u64),
}

impl FileSize {
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::KiloBytes(kb) => format!("{} KB", kb),
            FileSize::MegaBytes(mb) => format!("{} MB", mb),
            FileSize::GigaBytes(gb) => format!("{} GB", gb),
        }
    }
}

fn format_size2(size: u64) -> String {
    let filesize: FileSize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::KiloBytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::MegaBytes(size / 1_000_000),
        _ => FileSize::GigaBytes(size / 1_000_000_000),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::KiloBytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0),
        FileSize::MegaBytes(mb) => format!("{:.2} MB", mb as f64 / 1000.0),
        FileSize::GigaBytes(gb) => format!("{:.2} GB", gb as f64 / 1000.0),
    }
}



fn main() {
    let disk_type: DiskType = DiskType::SSD;

    // CANT COMPARE THEM LIKE THIS BELOW
    // if disk_type == DiskType::SSD {
    //     println!("SSD");
    // } else {
    //     println!("HDD");
    // }

    match disk_type {
        DiskType::SSD => println!("SSD"),
        DiskType::HDD => println!("HDD"),
    }

   
    let wine1: Wine = Wine {
        name: String::from("wine1 name"),
        region: WineRegions::Region1,
    };

    let wine2: Wine = Wine {
        name: String::from("wine2 name"),
        region: WineRegions::Region2,
    };

    let wine3: Wine = Wine {
        name: String::from("wine3 name"),
        region: WineRegions::Region3,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);

    supported_regions(wine1.region);
    supported_regions(wine2.region);
    supported_regions(wine3.region);
    supported_regions(WineRegions::Region4);


    let a: i32 = 10;
    let b: i32 = 2;

    let result: Option<i32> = divide(a, b);
    let result2: Option<i32> = divide(a, 0);

    println!("unwrapped option: {:?}", result.unwrap());
    // CANT UNWRAP A NONE, THE LINE BELOW WILL CAUSE A PANIC
    // println!("{:?}", result2.unwrap());

    match result  {
        Some(x) => println!("{}", x),
        None => println!("Cannot divide by zero")
    }

    match result2 {
        Some(x) => println!("{}", x),
        None => println!("Cannot divide by zero")
    }


    // let result = format_size(68880000837399);
    // println!("{}", result);


    let size = 2_000_000; // 2 million bytes, which should be 2 MB
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::KiloBytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::MegaBytes(size / 1_000_000),
        _ => FileSize::GigaBytes(size / 1_000_000_000)
    };

    println!("FileSize: {}", filesize.format_size());


}
