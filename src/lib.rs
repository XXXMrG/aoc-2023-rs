// mod input {
//     use std::iter::Iterator;
//     use std::path::Path;

//     pub struct InputIter {
//         text: &str,
//     }

//     impl InputIter {
//         pub fn new(path: &Path) -> InputIter {
//             let text =
//                 std::fs::read_to_string(path).expect("Something went wrong reading the file");
//             InputIter { text }
//         }
//     }

//     impl Iterator for InputIter {
//         type Item = String;

//         fn next(&mut self) -> Option<Self::Item> {
//             let mut lines = self.text.lines();
//         }
//     }
// }

#[cfg(test)]
mod tests {}
