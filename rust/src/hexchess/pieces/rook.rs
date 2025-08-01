use crate::hexchess::color::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;
use crate::hexchess::utils::walk;
use smallvec::SmallVec;

pub fn rook_moves_unsafe(
    hexchess: &Hexchess,
    from: &u8,
    color: &Color,
) -> SmallVec<[San; 30]> {
    let mut result: SmallVec<[San; 30]> = SmallVec::new();

    for n in [0u8, 2, 4, 6, 8, 10] {
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
