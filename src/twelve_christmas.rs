
pub fn twelve_christmas() {
  let count = [
      "first", "second", "third",
      "fourth", "fifth", "sixth",
      "seventh", "eighth", "ninth",
      "tenth", "eleventh", "twelfth"
    ];

    let verses = [
      "A partridge in a pear tree",
      "Two turtledoves",
      "Three French hens",
      "Four calling birds",
      "Five gold rings (five golden rings)",
      "Six geese a-laying",
      "Seven swans a-swimming",
      "Eight maids a-milking",
      "Nine ladies dancing",
      "Ten lords a-leaping",
      "Eleven pipers piping",
      "Twelve drummers drumming"
    ];

    let mut start: i32 = 0;

    println!("\n");

    while start < 12 {
      println!("On the {} day of Christmas, my true love sent to me", count[start as usize]);

      if start == 0 {
        println!("{}",  verses[start as usize]);
      } else {
        for number in (0..=start).rev() {
          if number == 0 {
            println!("And {}", verses[number as usize]);
          } else {
            println!("{}", verses[number as usize]);
          }
        }
      }

      println!("\n");
      start += 1;
    }
}
