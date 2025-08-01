use crate::hexchess::color::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;
use crate::hexchess::utils::walk;
use smallvec::SmallVec;

pub fn bishop_moves_unsafe(
    hexchess: &Hexchess,
    from: &u8,
    color: &Color,
) -> SmallVec<[San; 14]> {
    let mut result: SmallVec<[San; 14]> = SmallVec::new();

    for n in [1u8, 3, 5, 7, 9, 11] {
        let path = walk(hexchess, *from, n, color);

        for to in path {
            result.push(San {
                from: *from,
                promotion: None,
                to
            });
        }
    }

    result
}
