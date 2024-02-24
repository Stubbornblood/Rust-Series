fn main(){
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];

pritnln!("The twelve Days of Christmas");
for day in 0..12{
    pritnln!("On the {} day of Christmas",days[day]);
    pritnln!("My true love send to me.");

    for gift_index in (0..=day).rev(){
        pritnln!("{}",gifts[gift_index]);
    }
    pritnln!();
}


}