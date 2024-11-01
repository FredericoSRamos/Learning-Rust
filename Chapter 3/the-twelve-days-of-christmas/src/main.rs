
fn main() {
    let day_order = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["And a partridge in a pear tree.", "Two turtle doves", "Three french hens", "Four calling birds",
                            "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking",
                            "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    println!("\nOn the {} day of Christmas,\nmy true love sent to me\nA partridge in a pear tree.", day_order[0]);

    for day in 2..13
    {
        println!("\nOn the {} day of Christmas,\nmy true love sent to me", day_order[day - 1]);

        for i in (2..day).rev()
        {
            println!("{},", gifts[i]);
        }

        for i in (0..2).rev()
        {
            println!("{}", gifts[i]);
        }
    }
}