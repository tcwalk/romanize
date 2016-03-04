use std::io;
use std::num::ParseIntError;

//I = 1, V = 5, X = 10, L = 50, C = 100, D = 500, M = 1,000

fn main() {

   let mut x: u16 = 0;
   let mut counter: u8 = 1;
   let mut answer = String::from("");

   while x == 0 && counter < 6 {
      x = get_input();
      counter = counter + 1;
      if counter == 5 {
         println!("\nQuitting after 5 failures to enter usable integer\n")
      }
   }

   answer = convert(x);
   println!("{0} = {1}", x, answer);

}

fn get_input() -> u16 {

   println!("\nPlease enter an integer from 1 to 3000: ");
   let mut input_text = String::new();
   io::stdin()
      .read_line(&mut input_text)
      .ok()
      .expect("failed to read from stdin");

   println!("You entered {}", input_text);

   let mut input_num: u16 = 0;
   match check_input(input_text.trim()) {
      Ok(n) => (input_num = n),
      Err(err) => println!("this was not an integer: {}", input_text),
   };

   if input_num < 1 || input_num > 3000 {
      input_num = 0;
   }

   input_num
}


fn check_input(number_str: &str) -> Result<u16, ParseIntError> {
    number_str.parse::<u16>()
}

fn convert(mut remainder: u16) -> String {

   let roman_numerals: [&'static str; 7] = ["M", "D", "C", "L", "X", "V", "I"];
   let eq_int: [u16; 7] = [1000, 500, 100, 50, 10, 5, 1];
   let mut number_of_numeral: u16 = 0;
   let mut diff: u16 = 0;
   let mut i: u8 = 0;

   let mut answer = String::from("");

   while remainder > 0 {
      if remainder >= eq_int[i as usize] {
         number_of_numeral = remainder/eq_int[i as usize];
         if number_of_numeral == 9 {
            answer = answer + roman_numerals[i as usize] + roman_numerals[i as usize - 2];
         }
         else if number_of_numeral == 4 {
            answer = answer + roman_numerals[i as usize] + roman_numerals[i as usize - 1];
         }
         else if number_of_numeral == 5 {
            answer = answer + roman_numerals[i as usize - 1];
         }
         else if number_of_numeral > 5  && number_of_numeral < 9  {
            answer = answer + roman_numerals[i as usize - 1];
            diff = number_of_numeral - 5;
            while diff > 0 {
               answer = answer + roman_numerals[i as usize];
               diff = diff - 1;
            }
         }
         else if number_of_numeral > 0  && number_of_numeral < 4  {
            diff = number_of_numeral;
            while diff > 0 {
               answer = answer + roman_numerals[i as usize];
               diff = diff - 1;
            }
         }

         remainder = remainder - number_of_numeral*eq_int[i as usize];
      }
      else {
         i = i + 2;
      }      
   }

   answer
} 

