fn main() {
    let lyrics = [
            "Twelve drummers drumming",
            "Eleven pipers piping",
            "Ten lords a-leaping",
            "Nine ladies dancing",
            "Eight maids a-milking",
            "Seven swans a-swimming",
            "Six geese a-laying",
            "Five gold rings (five golden rings)",
            "Four calling birds",
            "Three French hens",
            "Two turtledoves",
            "And a partridge in a pear tree",
        ];

    let days = [ "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", 
        "ninth", "tenth", "eleventh", "twelfth" ];

    for day in 1..13 {
        println!("On the {} day of Christmas, my true love sent to me", days[day - 1]);
        for i in (0..day).rev() {
            println!("{}", lyrics[lyrics.len() - i - 1]);
        }
        println!("");
    }

    println!("{}", lyrics.last().unwrap());
}
