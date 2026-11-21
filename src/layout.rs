// Slot IDs 0–11 belong to the original two rows. Eastward extensions use pairs
// after those slots so older buildings, visits and delivery targets keep their IDs.
pub const ORIGINAL_COLUMNS: usize = 6;
pub const MAX_COLUMNS: usize = 9;
pub const LOTS: usize = MAX_COLUMNS * 2;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Shore {
    Cove,
    Headland,
    Terrace,
    Lagoon,
}

impl Shore {
    pub fn name(self) -> &'static str {
        match self {
            Self::Cove => "Bucht",
            Self::Headland => "Landzunge",
            Self::Terrace => "Terrassenhafen",
            Self::Lagoon => "Lagune",
        }
    }
}

#[derive(Clone, Copy)]
pub struct Layout {
    rows: [u8; 2],
    dock: u8,
    pub shore: Shore,
}

impl Layout {
    pub const fn legacy(columns: u8) -> Self {
        Self {
            rows: [columns; 2],
            dock: columns - 1,
            shore: Shore::Cove,
        }
    }

    pub const fn for_chapter(mut self, index: usize) -> Self {
        const SIZES: [[u8; 2]; 48] = [
            [3, 3],
            [4, 4],
            [4, 4],
            [5, 4],
            [5, 5],
            [5, 6],
            [6, 6],
            [6, 7],
            [7, 6],
            [8, 7],
            [6, 6],
            [7, 7],
            [6, 7],
            [7, 6],
            [7, 8],
            [8, 6],
            [6, 6],
            [8, 8],
            [9, 7],
            [7, 6],
            [6, 7],
            [8, 7],
            [9, 8],
            [9, 9],
            [6, 6],
            [7, 6],
            [6, 6],
            [8, 6],
            [7, 7],
            [6, 7],
            [8, 7],
            [7, 8],
            [7, 7],
            [6, 6],
            [7, 6],
            [8, 6],
            [6, 8],
            [9, 6],
            [7, 7],
            [6, 7],
            [8, 8],
            [7, 9],
            [8, 7],
            [9, 7],
            [8, 6],
            [7, 8],
            [9, 8],
            [9, 9],
        ];
        self.rows = SIZES[index];
        self.shore = match index {
            0 | 4 | 5 | 12 | 16 | 19 | 24 | 25 | 38 | 41 => Shore::Cove,
            1 | 6 | 9 | 11 | 13 | 18 | 22 | 26 | 33 | 37 | 45 => Shore::Headland,
            3 | 7 | 10 | 15 | 21 | 23 | 27 | 28 | 34 | 35 | 40 | 44 | 46 | 47 => Shore::Terrace,
            _ => Shore::Lagoon,
        };
        self
    }

    pub fn row(self, row: usize) -> usize {
        self.rows[row] as usize
    }
    pub fn width(self) -> usize {
        self.row(0).max(self.row(1))
    }
    pub fn count(self) -> usize {
        self.row(0) + self.row(1)
    }
    pub fn dock(self) -> usize {
        lot_id(1, self.dock as usize)
    }
    pub fn contains(self, lot: usize) -> bool {
        lot < LOTS && lot_col(lot) < self.row(lot_row(lot))
    }

    pub fn lots(self) -> impl Iterator<Item = usize> {
        (0..2).flat_map(move |row| (0..self.row(row)).map(move |col| lot_id(row, col)))
    }
}

pub const fn lot_id(row: usize, col: usize) -> usize {
    if col < ORIGINAL_COLUMNS {
        row * ORIGINAL_COLUMNS + col
    } else {
        ORIGINAL_COLUMNS * 2 + (col - ORIGINAL_COLUMNS) * 2 + row
    }
}

pub const fn lot_row(lot: usize) -> usize {
    if lot < ORIGINAL_COLUMNS * 2 {
        lot / ORIGINAL_COLUMNS
    } else {
        (lot - ORIGINAL_COLUMNS * 2) % 2
    }
}

pub const fn lot_col(lot: usize) -> usize {
    if lot < ORIGINAL_COLUMNS * 2 {
        lot % ORIGINAL_COLUMNS
    } else {
        (lot - ORIGINAL_COLUMNS * 2) / 2 + ORIGINAL_COLUMNS
    }
}

pub fn distance(a: usize, b: usize) -> usize {
    lot_col(a).abs_diff(lot_col(b)) + lot_row(a).abs_diff(lot_row(b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::CHAPTERS;

    #[test]
    fn layouts_extend_without_renumbering_original_plots_or_docks() {
        for i in 0..LOTS {
            assert_eq!(lot_id(lot_row(i), lot_col(i)), i);
        }
        for (index, chapter) in CHAPTERS.iter().enumerate() {
            let legacy = [3, 4, 4, 4, 5, 5].get(index).copied().unwrap_or(6);
            assert_eq!(chapter.layout.dock(), lot_id(1, legacy - 1));
            for row in 0..2 {
                for col in 0..legacy {
                    assert!(chapter.layout.contains(lot_id(row, col)));
                }
            }
            assert_eq!(chapter.layout.lots().count(), chapter.layout.count());
        }
    }
}
