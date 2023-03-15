use std::iter::Iterator;

#[derive(Debug)]
pub enum TokenKind {
    LiteralInt,
    LiteralFloat,
    Word,
}

pub fn slice_to_tokenkind(slice: &str) -> TokenKind {
    if slice.parse::<i32>().is_ok() {
        return TokenKind::LiteralInt;
    } else if slice.parse::<f32>().is_ok() {
        return TokenKind::LiteralFloat;
    } else {
        return TokenKind::Word;
    }
}

#[derive(Debug)]
pub struct Lexem(TokenKind, usize, usize);

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            index: 0
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Lexem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.input.len() { return None }

        let start = self.index;
        while let Some(ch) = self.input.chars().nth(self.index) {//&& !ch.is_whitespace() {
            if ch.is_whitespace() { break }
            self.index += 1;
        }
        let end = self.index - 1;
        self.index += 1;

        return Some(
            Lexem(
                slice_to_tokenkind(&self.input[start..=end]),
                start,
                end
            )
        );
    }
}

fn main() {
    // Too large file? Just buy more RAM.
    //let string1 = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque cursus viverra urna quis gravida. Sed gravida, libero in consectetur mattis, lorem erat condimentum diam, ut accumsan diam augue a erat. Pellentesque fermentum id tortor eu ullamcorper. Proin luctus tristique risus, non malesuada odio commodo sit amet. Nam ultricies odio in augue eleifend, id porttitor quam congue. In porta velit in mattis elementum. Aliquam lobortis tincidunt turpis non ultricies. Aenean lacinia a purus id tincidunt. Donec enim turpis, accumsan vitae cursus vitae, porta at nisl. Maecenas scelerisque eros sed lacus lacinia vestibulumAenean dictum a eros sed elementum. Donec dignissim rutrum sem, ut fermentum justo. Morbi feugiat massa eleifend, faucibus enim sed, dictum augue. Vivamus in luctus urna. Nunc feugiat tincidunt nisl at consectetur. Donec nec sapien nec quam eleifend fermentum. Maecenas maximus lacinia velit. Morbi lobortis libero sed augue congue, sed rutrum velit dictum. Vivamus nec dui finibus, gravida risus a, feugiat dolor. Morbi sed odio in nibh dignissim accumsan sit amet vitae nisi. Nunc varius sodales tellus, quis faucibus magna molestie in. Mauris ut luctus metus. Integer accumsan rhoncus enim eu sollicitudin. Donec blandit risus nec velit varius, sed semper erat varius. Morbi non congue leo, ac ultricies mauris. Vivamus luctus dui vel ex rutrum volutpat. Phasellus tempus dictum tincidunt. Suspendisse molestie sapien purus, at tincidunt mauris consectetur non. Proin ac urna interdum, aliquet neque non, sollicitudin nisl. Integer vehicula nibh in dolor iaculis lobortis. Suspendisse suscipit cursus vehicula. In a leo felis. Pellentesque dictum molestie maximus. Etiam porttitor, purus ac laoreet pellentesque, sem massa pulvinar risus, sit amet blandit eros nulla eu diam. Nunc vitae sem ac nunc luctus tempor. Suspendisse pharetra lacus nec ligula sagittis, ut dictum quam condimentum. Morbi fringilla sit amet sem quis tincidunt. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis eu consectetur urna. Duis sed ipsum et arcu sagittis eleifend. Integer risus quam, vulputate quis ipsum non, facilisis lobortis lorem. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque cursus viverra urna quis gravida. Sed gravida, libero in consectetur mattis, lorem erat condimentum diam, ut accumsan diam augue a erat. Pellentesque fermentum id tortor eu ullamcorper. Proin luctus tristique risus, non malesuada odio commodo sit amet. Nam ultricies odio in augue eleifend, id porttitor quam congue. In porta velit in mattis elementum. Aliquam lobortis tincidunt turpis non ultricies. Aenean lacinia a purus id tincidunt. Donec enim turpis, accumsan vitae cursus vitae, porta at nisl. Maecenas scelerisque eros sed lacus lacinia vestibulumAenean dictum a eros sed elementum. Donec dignissim rutrum sem, ut fermentum justo. Morbi feugiat massa eleifend, faucibus enim sed, dictum augue. Vivamus in luctus urna. Nunc feugiat tincidunt nisl at consectetur. Donec nec sapien nec quam eleifend fermentum. Maecenas maximus lacinia velit. Morbi lobortis libero sed augue congue, sed rutrum velit dictum. Vivamus nec dui finibus, gravida risus a, feugiat dolor. Morbi sed odio in nibh dignissim accumsan sit amet vitae nisi. Nunc varius sodales tellus, quis faucibus magna molestie in. Mauris ut luctus metus. Integer accumsan rhoncus enim eu sollicitudin. Donec blandit risus nec velit varius, sed semper erat varius. Morbi non congue leo, ac ultricies mauris. Vivamus luctus dui vel ex rutrum volutpat. Phasellus tempus dictum tincidunt. Suspendisse molestie sapien purus, at tincidunt mauris consectetur non. Proin ac urna interdum, aliquet neque non, sollicitudin nisl. Integer vehicula nibh in dolor iaculis lobortis. Suspendisse suscipit cursus vehicula. In a leo felis. Pellentesque dictum molestie maximus. Etiam porttitor, purus ac laoreet pellentesque, sem massa pulvinar risus, sit amet blandit eros nulla eu diam. Nunc vitae sem ac nunc luctus tempor. Suspendisse pharetra lacus nec ligula sagittis, ut dictum quam condimentum. Morbi fringilla sit amet sem quis tincidunt. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis eu consectetur urna. Duis sed ipsum et arcu sagittis eleifend. Integer risus quam, vulputate quis ipsum non, facilisis lobortis lorem.";
    // abc: 0-2
    // 1º: 3
    // 30: 4-5
    // 2º space: 6
    // def: 7-9
    let string1 = "abc 30 def";
    let x = Lexer::new(string1);
    for i in x {
        println!("{:?}", i);
        println!("{:?}", &string1[i.1..=i.2]);
    }
}
