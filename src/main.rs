fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    // for region in regions.iter() {
    //     println!("{}", &region);
    // }
    // 2021 edition 版本之后，可以直接 for in regions.
    for region in regions {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}