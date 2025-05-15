// use crate::Sei;

// use embedded_graphics::primitives::Rectangle;
// use heapless::Vec;

// pub struct SeiStack<'a, C> {
//     stack: Vec<Sei<'a, C>, 8>,
//     area: Rectangle,
// }

// pub enum Layer {
//     Above,
//     Below,
// }

// impl<'a, C> SeiStack<'a, C> {
//     pub fn new(sei: Sei<'a, C>, area: Rectangle) -> Self {
//         let mut stack = Vec::new();

//         stack.push(sei).unwrap();
//         // .expect("Vec is 8 long, this should be ok");

//         Self { stack, area }
//     }

//     pub fn stack(&mut self, sei: Sei<'a, C>, layer: Layer) -> Result<(), Sei<'a, C>> {
//         match layer {
//             Layer::Above => self.stack.push(sei),
//             Layer::Below => self.stack.insert(0, sei),
//         }
//         // todo!()
//     }
// }
