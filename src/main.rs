use std::mem::discriminant;



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



}
